//! A TCP transport that carries Rust values.
//!
//! A `Packet` is any serializable type, identified on the wire by a hash of that type, so
//! there is no registry to keep in step and no ids to allocate. `PacketServer` accepts
//! connections and `PacketClient` makes one; both run their socket on a thread of their own
//! and hand everything to the owner through `poll`.
//!
//! # Not in scope
//!
//! **The protocol.** Who is allowed to connect, what has to be said first, what counts as a
//! peer being ready, and what any packet means are all the owner's. This layer will report
//! every packet from every peer that finished a TCP connection, including one the owner is
//! about to refuse; `PacketServer::disconnect` is how a refusal is acted on, and the
//! handshake phase on the client is described by two things the owner supplies - a greeting
//! to send and a predicate saying which packet ends it.
//!
//! Authentication and encryption. There is neither, and a `PacketServer` on
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
