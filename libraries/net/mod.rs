//! A TCP transport that carries Rust values.
//!
//! A `Packet` is any serializable type, identified on the wire by a hash of that type, so there is
//! no registry to keep in step and no ids to allocate. `PacketServer` accepts connections and
//! `PacketClient` makes one; both run their socket on a thread of their own and hand everything
//! over through `poll`.
//!
//! **Not in scope: the protocol.** Who may connect, what is said first, and what a packet
//! means are the owner's. Every packet from every connected peer is reported, including from
//! one about to be refused; `PacketServer::disconnect` acts on a refusal, and the client's
//! handshake is a greeting plus a predicate saying which packet ends it.
//!
//! Authentication and encryption: there is neither, so a `PacketServer` on
//! `BindAddress::AllInterfaces` is reachable by anyone who can reach the port.

pub use client::*;
pub use connection::*;
pub use packet::*;
pub use server::*;

mod client;
mod connection;
mod packet;
mod server;
mod tests;
