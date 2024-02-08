use anyhow::Result;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc};

const PORT: u16 = 28603;
const ADDR: &str = "home.susko.si";

#[derive(Debug)]
pub enum ConnectionState {
    DISCONNECTED,
    CONNECTING(std::thread::JoinHandle<Result<(rustls::ClientConnection, TcpStream)>>),
    CONNECTED(std::thread::JoinHandle<Result<()>>),
    FAILED(anyhow::Error),
}

fn handle_connection(mut client_conf: rustls::ClientConnection, mut tcp_stream: TcpStream, receiver: &mpsc::Receiver<kvptree::ValueType>, sender: &mpsc::Sender<kvptree::ValueType>) -> Result<()> {
    let mut tls_stream = rustls::Stream::new(&mut client_conf, &mut tcp_stream);
    tls_stream.sock.set_nonblocking(true)?;
    loop {
        let mut size_buf = [0u8; 4];
        let res = tls_stream.read_exact(&mut size_buf);
        if let Err(e) = res {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                eprintln!("error reading from server:\n{e}");
                return Err(e.into());
            }
        } else {
            let buf_size = u32::from_be_bytes(size_buf) as usize;
            let mut buf = vec![0; buf_size];
            tls_stream.sock.set_nonblocking(false)?;
            tls_stream.read_exact(&mut buf)?;
            tls_stream.sock.set_nonblocking(true)?;
            #[allow(clippy::unwrap_used)] //is safe
            let res = match kvptree::from_byte_vec(buf.clone()) {
                Err(e) => {
                    eprintln!("error converting bytes to kvp tree:\n{e}");
                    continue;
                }
                Ok(res) => res,
            };
            let res = sender.send(res);
            if res.is_err() {
                eprintln!("client has disconnected while the server is still sending messages");
                tls_stream.conn.send_close_notify();
                tls_stream.write_all(b"closing connection")?;
                return Ok(());
            }
        }

        if let Ok(message) = receiver.try_recv() {
            tls_stream.sock.set_nonblocking(false)?;
            let byte_vec = kvptree::to_byte_vec(message);
            tls_stream.write_all(&(byte_vec.len() as u32).to_be_bytes())?; //this makes an 4 byte number
            tls_stream.write_all(&byte_vec)?;
            tls_stream.flush()?;
            tls_stream.sock.set_nonblocking(true)?;
        }
    }
}

impl ConnectionState {
    fn connect(&mut self, server_to_client: mpsc::Sender<kvptree::ValueType>, client_to_server: mpsc::Receiver<kvptree::ValueType>) {
        if let Self::CONNECTING(handle) = std::mem::replace(self, Self::FAILED(anyhow::anyhow!("temp err"))) {
            let handle = match handle.join() {
                Ok(handle) => match handle {
                    Ok(handle) => handle,
                    Err(e) => {
                        eprintln!("error connecting to server:\n{e}");
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

            let spawn_result = std::thread::Builder::new().name("TLS connection".to_owned()).spawn(move || -> Result<()> {
                let res = handle_connection(handle.0, handle.1, &client_to_server, &server_to_client);
                match res {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        println!("error: {e}\n\nbacktrace:\n{}", e.backtrace());
                        Err(e)
                    }
                }
            });

            match spawn_result {
                Ok(thread_handle) => {
                    *self = Self::CONNECTED(thread_handle);
                }
                Err(e) => {
                    eprintln!("error creating TLS connection init thread: {e}");
                    *self = Self::FAILED(e.into());
                }
            }
        }
    }
}

pub struct TlsClient {
    state: ConnectionState,
    socket: std::net::SocketAddr,
    config: Arc<rustls::ClientConfig>,
    srv_to_client: mpsc::Receiver<kvptree::ValueType>,
    client_to_srv: mpsc::Sender<kvptree::ValueType>,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        let (sender, _) = mpsc::channel(); //temp channel to fill values
        let (_, receiver) = mpsc::channel(); //temp channel to fill values
        Ok(Self {
            state: ConnectionState::DISCONNECTED,
            socket: (ADDR, PORT).to_socket_addrs()?.next().ok_or_else(|| anyhow::anyhow!("incorrect DNS"))?,
            config: get_client_config(),
            srv_to_client: receiver, //both will have the other half disconnected
            client_to_srv: sender,
        })
    }

    fn new_connection(&mut self) {
        let temp_socket = self.socket;
        let temp_config = self.config.clone();

        let spawn_result = std::thread::Builder::new()
            .name("TLS connection init".to_owned())
            .spawn(move || -> Result<(rustls::ClientConnection, TcpStream)> {
                let socket = temp_socket;
                let tls_conn = rustls::ClientConnection::new(temp_config, ADDR.try_into()?)?;
                std::thread::sleep(std::time::Duration::from_secs(1)); //artificial delay for debugging
                let tcp_stream = TcpStream::connect_timeout(&socket, std::time::Duration::from_secs(10))?;
                Ok((tls_conn, tcp_stream))
            });

        match spawn_result {
            Ok(thread) => self.state = ConnectionState::CONNECTING(thread),
            Err(e) => {
                eprintln!("error creating TLS connection init thread: {e}");
                self.state = ConnectionState::FAILED(e.into());
            }
        }
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
                let (srv_to_cl_send, srv_to_cl_recv) = mpsc::channel();
                let (cl_to_srv_send, cl_to_srv_recv) = mpsc::channel();
                self.srv_to_client = srv_to_cl_recv;
                self.client_to_srv = cl_to_srv_send;
                self.state.connect(srv_to_cl_send, cl_to_srv_recv);
            }
            _ => {}
        }
    }

    pub fn read(&mut self) -> Result<kvptree::ValueType, TryRecvError> {
        let res = self.srv_to_client.try_recv();
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                if matches!(e, TryRecvError::Disconnected) {
                    self.close();
                }
                Err(e)
            }
        }
    }

    pub fn write(&mut self, message: kvptree::ValueType) -> Result<()> {
        self.client_to_srv.send(message)?;
        Ok(())
    }

    pub fn close(&mut self) {
        //this function simply disconnects the client side of the channel. the thread will close when it realizes this as there is no way to properly kill a thread in rust
        let (sender, _) = mpsc::channel(); //temp channel to fill values
        let (_, receiver) = mpsc::channel(); //temp channel to fill values
        self.srv_to_client = receiver;
        self.client_to_srv = sender;
        self.state = ConnectionState::DISCONNECTED;
    }

    #[must_use]
    pub const fn get_connection_state(&self) -> &ConnectionState {
        &self.state
    }

    pub fn reset(&mut self) {
        self.close();
        self.state = ConnectionState::DISCONNECTED;
    }
}

fn get_client_config() -> Arc<rustls::ClientConfig> {
    let anchors = webpki_roots::TLS_SERVER_ROOTS;

    let mut root_store = rustls::RootCertStore::empty();
    root_store.roots.extend(anchors.iter().map(|e| rustls_pki_types::TrustAnchor {
        subject: e.subject.clone(),
        subject_public_key_info: e.subject_public_key_info.clone(),
        name_constraints: e.name_constraints.clone(),
    }));

    Arc::new(rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth())
}
