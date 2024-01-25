use crate::shared::tls_client;
use crate::shared::tls_client::ConnectionState;
use anyhow::Result;

#[allow(non_camel_case_types)] //shut up clippy
#[derive(Debug)]
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
            user_credentials: Some(("test".to_owned(), "test".to_owned())),
            authentication_state: AuthenticationState::NOT_AUTHENTICATED, //read from file and set this to NOT_AUTHENTICATED if the password is saved or change when the user enters the password
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
        if matches!(self.authentication_state, AuthenticationState::AUTHENTICATED) || matches!(self.authentication_state, AuthenticationState::FAILED) {
            return;
        }
        //this is ugly lol
        match self.get_connection_state() {
            ConnectionState::CONNECTED(_) => match &self.authentication_state {
                AuthenticationState::NOT_AUTHENTICATED => match &self.user_credentials {
                    Some((username, password)) => {
                        let res = self.write(&format!("{username} {password}"));
                        if let Err(e) = res {
                            println!("error writing to server: {e}");
                            self.authentication_state = AuthenticationState::FAILED;
                            return;
                        }
                        self.authentication_state = AuthenticationState::AUTHENTICATING;
                    }
                    None => {
                        self.authentication_state = AuthenticationState::NO_CREDENTIALS;
                    }
                },
                AuthenticationState::AUTHENTICATING => match self.read() {
                    Ok(message) => {
                        println!("received a message");
                        if message == "auth_success" {
                            self.authentication_state = AuthenticationState::AUTHENTICATED;
                            println!("client is authenticated");
                            self.client.close();
                        } else if message == "auth_failed" {
                            self.authentication_state = AuthenticationState::FAILED;
                            self.client.close();
                            println!("auth failed");
                        }
                    }
                    Err(e) => {
                        eprintln!("error reading from server: {e}");
                    }
                },
                _ => {}
            },
            _ => self.connect(),
        }
    }

    pub fn print_state(&self) {
        println!("{:?} {:?}", self.client.get_connection_state(), self.authentication_state);
    }

    #[must_use]
    pub const fn get_connection_state(&self) -> &ConnectionState {
        self.client.get_connection_state()
    }

    #[must_use]
    pub const fn get_authentication_state(&self) -> &AuthenticationState {
        &self.authentication_state
    }
}
