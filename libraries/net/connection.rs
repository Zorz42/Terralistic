use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;

use message_io::network::Endpoint;

/// One peer of a `PacketServer`.
///
/// Compared and hashed by address, so the same peer is the same `Connection` however many
/// copies of it have been handed out.
#[derive(Clone, Eq, Debug)]
pub struct Connection {
    pub(super) endpoint: Endpoint,
}

impl Connection {
    #[must_use]
    pub(super) const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }

    #[must_use]
    pub fn addr(&self) -> SocketAddr {
        self.endpoint.addr()
    }
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.endpoint.addr().hash(state);
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint.addr() == other.endpoint.addr()
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.endpoint.addr())
    }
}

/// Which interfaces a server accepts connections on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindAddress {
    /// Only this machine can connect.
    Loopback,
    /// Every interface, so peers on other machines can connect.
    AllInterfaces,
}

impl BindAddress {
    #[must_use]
    pub const fn as_ip(self) -> &'static str {
        match self {
            Self::Loopback => "127.0.0.1",
            Self::AllInterfaces => "0.0.0.0",
        }
    }
}

/// How bad a thing the transport is telling the caller about.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Where the transport's running commentary goes.
///
/// A callback rather than a `println!`, because the transport runs on a thread of its own
/// and the owner usually has somewhere specific it wants this to end up - a server console,
/// a ui panel, a test's buffer. It is `Send + Sync` for the same reason.
pub type Logger = Arc<dyn Fn(LogLevel, &str) + Send + Sync>;

/// A logger that throws everything away, for callers that do not want the commentary.
#[must_use]
pub fn no_logging() -> Logger {
    Arc::new(|_level, _message| {})
}
