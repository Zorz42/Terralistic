use crate::shared::tls_client;
use crate::shared::tls_client::ConnectionState;
use anyhow::Result;

pub struct TlsClient {
    is_authenticated: bool,
    client: tls_client::TlsClient,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            is_authenticated: false,
            client: tls_client::TlsClient::new()?,
        })
    }

    pub fn connect(&mut self) {
        self.client.connect();
    }

    pub fn read(&mut self) -> Result<String> {
        self.client.read()
    }

    pub fn write(&mut self, message: &str) -> Result<()> {
        self.client.write(message)
    }

    pub fn close(&mut self) {
        self.client.close();
    }

    pub fn authenticate(&mut self) {
        match self.client.get_state() {
            ConnectionState::FAILED(_) => {}
            ConnectionState::DISCONNECTED | ConnectionState::CONNECTING(_) => self.client.connect(),
            ConnectionState::CONNECTED(_) => {
                let res = self.client.write("authenticate me");
                if res.is_err() {
                    self.client.close();
                }
                self.is_authenticated = true;
                self.close();
            }
        }

        self.is_authenticated = true;
    }

    #[must_use]
    pub const fn get_state(&self) -> &ConnectionState {
        self.client.get_state()
    }

    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }
}
