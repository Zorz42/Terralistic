use crate::shared::tls_client;
use crate::shared::tls_client::ConnectionState;
use anyhow::Result;

#[allow(non_camel_case_types)] //shut up clippy
pub enum AuthenticationState {
    NOT_AUTHENTICATED,
    AUTHENTICATING,
    AUTHENTICATED,
    NO_CREDENTIALS,
    FAILED,
}

pub struct TlsClient {
    user_credentials: Option<(String, String)>,
    authentication_state: AuthenticationState,
    client: tls_client::TlsClient,
}

impl TlsClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            user_credentials: None,
            authentication_state: AuthenticationState::NO_CREDENTIALS, //read from file and set this to NOT_AUTHENTICATED if the password is saved
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
        self.authentication_state = AuthenticationState::NOT_AUTHENTICATED;
    }

    pub fn authenticate(&mut self) {
        //this is ugly lol
        match self.client.get_state() {
            ConnectionState::FAILED(_) => {}
            ConnectionState::DISCONNECTED | ConnectionState::CONNECTING(_) => self.client.connect(),
            ConnectionState::CONNECTED(_) => match &self.authentication_state {
                AuthenticationState::NOT_AUTHENTICATED => match &self.user_credentials {
                    Some((username, password)) => {
                        self.client.write(&format!("{} {}", username, password)); //TODO handle errors
                        self.authentication_state = AuthenticationState::AUTHENTICATING;
                    }
                    None => {
                        self.authentication_state = AuthenticationState::NO_CREDENTIALS;
                    }
                },
                AuthenticationState::AUTHENTICATING => {
                    if let Ok(message) = self.client.read() {
                        if message == "auth_success" {
                            self.authentication_state = AuthenticationState::AUTHENTICATED;
                        } else if message == "auth_failed" {
                            self.authentication_state = AuthenticationState::FAILED;
                        }
                    }
                }
                AuthenticationState::AUTHENTICATED | AuthenticationState::NO_CREDENTIALS | AuthenticationState::FAILED => {}
            },
        }
    }

    #[must_use]
    pub const fn get_state(&self) -> &ConnectionState {
        self.client.get_state()
    }

    #[must_use]
    pub const fn is_authenticated(&self) -> &AuthenticationState {
        &self.authentication_state
    }
}
