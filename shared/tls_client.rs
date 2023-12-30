use anyhow::Result;
use rustls::ClientConfig;
use std::net::{TcpStream, ToSocketAddrs};

const PORT: u16 = 28603;
const ADDR: &str = "home.susko.si";

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
    pub is_authenticated: bool,
    pub state: ConnectionState,
    socket: std::net::SocketAddr,
    config: std::sync::Arc<ClientConfig>,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            is_authenticated: false,
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
