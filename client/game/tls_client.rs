use crate::shared::tls_client;
use crate::shared::tls_client::ConnectionState;
use anyhow::Result;
use directories::BaseDirs;
use std::collections::HashMap;

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
        let dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("could not get base dir"))?;
        let user_file = dirs.data_dir().join("Terralistic").join("user.txt");
        //if the file doesn't exist, create it
        if !user_file.exists() {
            std::fs::create_dir_all(user_file.parent().ok_or_else(|| anyhow::anyhow!("err"))?)?;
            std::fs::write(user_file.clone(), "")?;
        }
        let user_credentials = std::fs::read_to_string(user_file).map_or(None, |data| {
            let mut lines = data.lines();
            let username = lines.next();
            let password = lines.next();
            if let (Some(username), Some(password)) = (username, password) {
                Some((username.to_owned(), password.to_owned()))
            } else {
                None
            }
        });

        Ok(Self {
            user_credentials,
            authentication_state: AuthenticationState::NOT_AUTHENTICATED, //read from file and set this to NOT_AUTHENTICATED if the password is saved or change when the user enters the password
            client: tls_client::TlsClient::new()?,
        })
    }

    pub fn connect(&mut self) {
        self.client.connect();
    }

    pub fn read(&mut self) -> Result<kvptree::ValueType> {
        self.client.read()
    }

    pub fn write(&mut self, message: kvptree::ValueType) -> Result<()> {
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

        if !matches!(self.get_connection_state(), ConnectionState::CONNECTED(_)) {
            self.connect();
        }
        if matches!(self.get_connection_state(), ConnectionState::CONNECTED(_)) {
            self.inner_authenticate();
        }
    }

    fn inner_authenticate(&mut self) {
        match &self.authentication_state {
            AuthenticationState::NOT_AUTHENTICATED => match &self.user_credentials {
                Some((username, password)) => {
                    let res = self.write(kvptree::ValueType::LIST(HashMap::from([
                        ("auth_type".to_owned(), kvptree::ValueType::STRING("login".to_owned())),
                        (
                            "credentials".to_owned(),
                            kvptree::ValueType::LIST(HashMap::from([
                                ("username".to_owned(), kvptree::ValueType::STRING(username.to_owned())),
                                ("password".to_owned(), kvptree::ValueType::STRING(password.to_owned())),
                            ])),
                        ),
                    ])));
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
                    println!("received a message:\n\n{message}\n\n");
                    let message = message.get_str("result").unwrap_or_else(|_| "login_failed".to_owned());
                    if message == "login_successful" {
                        self.authentication_state = AuthenticationState::AUTHENTICATED;
                        println!("client is authenticated");
                        self.client.close();
                    } else if message == "login_failed" {
                        self.authentication_state = AuthenticationState::FAILED;
                        self.client.close();
                        println!("auth failed");
                    }
                }
                Err(e) => {
                    if e.to_string() != *"receiving on an empty channel" {
                        //there's probably a better way idk
                        eprintln!("error reading from server: {e}");
                    }
                }
            },
            _ => {}
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

    pub fn reset(&mut self) {
        self.client.reset();
        self.authentication_state = AuthenticationState::NOT_AUTHENTICATED;

        #[allow(clippy::let_underscore_must_use)]
        let _ = self.set_credentials();
    }

    fn set_credentials(&mut self) -> Result<()> {
        let dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("err"))?;
        let user_file = dirs.data_dir().join("Terralistic").join("user.txt");
        //if the file doesn't exist, create it
        if !user_file.exists() {
            std::fs::create_dir_all(user_file.parent().ok_or_else(|| anyhow::anyhow!("err"))?)?;
            std::fs::write(user_file.clone(), "")?;
        }
        self.user_credentials = std::fs::read_to_string(user_file).map_or(None, |data| {
            let mut lines = data.lines();
            let username = lines.next();
            let password = lines.next();
            if let (Some(username), Some(password)) = (username, password) {
                Some((username.to_owned(), password.to_owned()))
            } else {
                None
            }
        });

        Ok(())
    }
}
