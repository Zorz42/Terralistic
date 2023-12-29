use anyhow::Result;
use rustls::ClientConfig;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;

const PORT: u16 = 28603;
const ADDR: &str = "home.susko.si";

pub struct TlsClient {
    socket: std::net::SocketAddr,
    tls_conn: rustls::ClientConnection,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        let config = get_client_config();
        let socket = (ADDR, PORT).to_socket_addrs()?.next().ok_or_else(|| anyhow::anyhow!("incorrect DNS"))?;

        let server_name = ADDR.try_into()?;
        Ok(Self {
            socket,
            tls_conn: rustls::ClientConnection::new(config, server_name)?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tcp_stream = std::net::TcpStream::connect_timeout(&self.socket, std::time::Duration::from_millis(1000))?; //i should multithread/async this so it doesn't block the thread
        let mut tls_stream = rustls::Stream::new(&mut self.tls_conn, &mut tcp_stream);

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
