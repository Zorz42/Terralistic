use anyhow::Result;
use rustls::ClientConfig;
use std::borrow::BorrowMut;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::DerefMut;

const PORT: u16 = 28603;
const ADDR: &str = "home.susko.si";

#[derive(Debug)]
pub enum ConnectionState {
    DISCONNECTED,
    CONNECTING(std::thread::JoinHandle<Result<(rustls::ClientConnection, TcpStream)>>),
    CONNECTED(Box<(rustls::ClientConnection, TcpStream)>),
    FAILED(anyhow::Error),
}

impl ConnectionState {
    fn connect(&mut self) {
        if let Self::CONNECTING(handle) = std::mem::replace(self, Self::FAILED(anyhow::anyhow!("temp err"))) {
            let handle = match handle.join() {
                Ok(handle) => match handle {
                    Ok(handle) => handle,
                    Err(e) => {
                        eprintln!("error connecting to server:\n{e}\n\nbacktrace:\n{}", e.backtrace());
                        *self = Self::FAILED(e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("error joining handles:\n{e:?}");
                    *self = Self::FAILED(anyhow::anyhow!("{e:?}"));
                    return;
                }
            };
            *self = Self::CONNECTED(Box::new(handle));
        }
    }
}

pub struct TlsClient {
    state: ConnectionState,
    socket: std::net::SocketAddr,
    config: std::sync::Arc<ClientConfig>,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: ConnectionState::DISCONNECTED,
            socket: (ADDR, PORT).to_socket_addrs()?.next().ok_or_else(|| anyhow::anyhow!("incorrect DNS"))?,
            config: get_client_config(),
        })
    }

    fn new_connection(&mut self) {
        let temp_socket = self.socket;
        let temp_config = self.config.clone();
        let thread = std::thread::spawn(move || -> Result<(rustls::ClientConnection, TcpStream)> {
            let socket = temp_socket;
            let tls_conn = rustls::ClientConnection::new(temp_config, ADDR.try_into()?)?;
            std::thread::sleep(std::time::Duration::from_secs(2)); //artificial delay for debugging
            let tcp_stream = TcpStream::connect_timeout(&socket, std::time::Duration::from_millis(10000))?;
            tcp_stream.set_nonblocking(true)?; //TODO: this for some reason makes the connection fail, fix it
            Ok((tls_conn, tcp_stream))
        });
        self.state = ConnectionState::CONNECTING(thread);
    }

    pub fn connect(&mut self) {
        match &self.state {
            ConnectionState::DISCONNECTED => {
                self.new_connection();
            }
            ConnectionState::CONNECTING(handle) => {
                if !handle.is_finished() {
                    return;
                }
                self.state.connect();
            }
            _ => {}
        }
    }

    pub fn read(&mut self) -> Result<String> {
        if let ConnectionState::CONNECTED(connection) = self.state.borrow_mut() {
            #[allow(clippy::explicit_deref_methods)] //the solution by clippy is way uglier
            let (connection, tcp_stream) = connection.deref_mut();
            let mut tls_conn = rustls::Stream::new(connection, tcp_stream);
            let mut buffer = String::new();
            tls_conn.read_to_string(&mut buffer)?;
            return Ok(buffer);
        }
        anyhow::bail!("not connected to the cloud server")
    }

    pub fn write(&mut self, message: &str) -> Result<()> {
        return if let ConnectionState::CONNECTED(connection) = self.state.borrow_mut() {
            #[allow(clippy::explicit_deref_methods)] //the solution by clippy is way uglier
            let (connection, tcp_stream) = connection.deref_mut();
            let mut tls_conn = rustls::Stream::new(connection, tcp_stream);
            let mut read = 0;
            loop {
                if read == message.len() {
                    break;
                }
                let res = tls_conn.write(message.as_bytes().get(read..).ok_or_else(|| anyhow::anyhow!("index out of bounds"))?);
                match res {
                    Ok(n) => read += n,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            continue;
                        }
                        return Err(e.into());
                    }
                }
            }
            Ok(())
        } else {
            anyhow::bail!("not connected to the cloud server")
        };
    }

    pub fn close(&mut self) {
        if let ConnectionState::CONNECTED(connection) = self.state.borrow_mut() {
            #[allow(clippy::explicit_deref_methods)] //the solution by clippy is way uglier
            let (connection, tcp_stream) = connection.deref_mut();
            connection.send_close_notify();
            let mut tls_conn = rustls::Stream::new(connection, tcp_stream);
            #[allow(clippy::let_underscore_must_use)] //if it fails we don't care
            let _ = tls_conn.write_all(b"closing connection");
            self.state = ConnectionState::DISCONNECTED;
        }
    }

    #[must_use]
    pub const fn get_connection_state(&self) -> &ConnectionState {
        &self.state
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
