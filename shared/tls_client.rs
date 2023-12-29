use anyhow::Result;
use rustls::ClientConfig;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

const PORT: u16 = 28603;
const ADDR: &str = "home.susko.si";

pub enum State {
    DISCONNECTED,
    CONNECTING(std::thread::JoinHandle<Result<(rustls::ClientConnection, TcpStream)>>),
    CONNECTED(Box<(rustls::ClientConnection, TcpStream)>),
    FAILED,
}

impl State {
    fn connect(&mut self) {
        if let Self::CONNECTING(handle) = std::mem::replace(self, Self::FAILED) {
            let handle = match handle.join() {
                Ok(handle) => match handle {
                    Ok(handle) => handle,
                    Err(e) => {
                        eprintln!("error connecting to server:\n{e}\n\nbacktrace:\n{}", e.backtrace());
                        *self = Self::FAILED;
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("error joining handles:\n{e:?}");
                    *self = Self::FAILED;
                    return;
                }
            };
            *self = Self::CONNECTED(Box::new(handle));
        }
    }
}

pub struct TlsClient {
    pub is_authenticated: bool,
    pub state: State,
    socket: std::net::SocketAddr,
    config: std::sync::Arc<ClientConfig>,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            is_authenticated: false,
            state: State::DISCONNECTED,
            socket: (ADDR, PORT).to_socket_addrs()?.next().ok_or_else(|| anyhow::anyhow!("incorrect DNS"))?,
            config: get_client_config(),
        })
    }

    pub fn connect(&mut self) {
        match &self.state {
            State::DISCONNECTED => {
                let temp_socket = self.socket;
                let temp_config = self.config.clone();
                let thread = std::thread::spawn(move || -> Result<(rustls::ClientConnection, TcpStream)> {
                    let socket = temp_socket;
                    let tls_conn = rustls::ClientConnection::new(temp_config, ADDR.try_into()?)?;
                    let tcp_stream = TcpStream::connect_timeout(&socket, std::time::Duration::from_millis(10000))?;
                    Ok((tls_conn, tcp_stream))
                });
                self.state = State::CONNECTING(thread);
            }
            State::CONNECTING(handle) => {
                if !handle.is_finished() {
                    return;
                }
                self.state.connect();
            }
            _ => {}
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tcp_stream = TcpStream::connect_timeout(&self.socket, std::time::Duration::from_millis(1000))?; //i should multithread/async this so it doesn't block the thread
        let mut tls_conn = rustls::ClientConnection::new(self.config.clone(), ADDR.try_into()?)?;
        let mut tls_stream = rustls::Stream::new(&mut tls_conn, &mut tcp_stream);

        self.is_authenticated = true;

        tls_stream.write_all("am i authenticated".to_owned().as_bytes())?;
        let mut buf = [0; 1024];
        let res = tls_stream.read(&mut buf)?;
        //SAFETY: safe for now since i know what i'm sending, will make safe when arbitrary data is sent
        println!("Read {res} bytes: {}", unsafe { std::str::from_utf8_unchecked(&buf) });

        tls_stream.conn.send_close_notify();
        tls_stream.write_all(b"closing conn")?;
        Ok(())
    }
}

fn get_client_config() -> std::sync::Arc<ClientConfig> {
    let anchors = webpki_roots::TLS_SERVER_ROOTS;

    let mut root_store = rustls::RootCertStore::empty();
    root_store.roots.extend(anchors.iter().map(|e| rustls_pki_types::TrustAnchor {
        subject: e.subject.clone(),
        subject_public_key_info: e.subject_public_key_info.clone(),
        name_constraints: e.name_constraints.clone(),
    }));

    std::sync::Arc::new(ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth())
}
