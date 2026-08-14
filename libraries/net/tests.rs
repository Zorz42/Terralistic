#![allow(clippy::unwrap_used, clippy::panic)] // a test that cannot go on should fail loudly
#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    use crate::libraries::net::{no_logging, BindAddress, ClientError, LogLevel, Logger, Packet, PacketClient, PacketServer, ServerEvent};
    use crate::libraries::serialization;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Greeting {
        text: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Farewell;

    // --- Packet ---

    /// Packet ids are a hash of the rust type, so a packet that arrives is offered to every
    /// type in turn and only the right one answers. That is the only thing keeping the
    /// protocol coherent, since there is no registry and no declared ids.
    #[test]
    fn test_a_packet_only_deserializes_as_its_own_type() {
        let packet = Packet::new(Greeting { text: "hello".to_owned() }).unwrap();

        assert!(packet.try_deserialize::<Farewell>().is_none());
        assert!(packet.try_deserialize::<u32>().is_none());
        assert_eq!(packet.try_deserialize::<Greeting>().unwrap().text, "hello");
    }

    #[test]
    fn test_a_packet_survives_the_wire() {
        let packet = Packet::new(Greeting { text: "hello".to_owned() }).unwrap();
        let bytes = serialization::serialize(&packet).unwrap();
        let received: Packet = serialization::deserialize(&bytes).unwrap();

        assert_eq!(received.try_deserialize::<Greeting>().unwrap().text, "hello");
    }

    /// `is` answers the same question as `try_deserialize` without paying to decode, which
    /// is what a handshake predicate wants.
    #[test]
    fn test_is_agrees_with_try_deserialize() {
        let packet = Packet::new(Farewell).unwrap();

        assert!(packet.is::<Farewell>());
        assert!(!packet.is::<Greeting>());
        assert_eq!(packet.is::<Greeting>(), packet.try_deserialize::<Greeting>().is_some());
    }

    // --- BindAddress ---

    /// Loopback is what a server that must not be reachable from the network binds, and the
    /// distinction is the whole reason this is not a bare string.
    #[test]
    fn test_bind_addresses_are_what_they_say() {
        assert_eq!(BindAddress::Loopback.as_ip(), "127.0.0.1");
        assert_eq!(BindAddress::AllInterfaces.as_ip(), "0.0.0.0");
    }

    // --- ClientError ---

    /// The kinds exist so an owner can say what a failure *means*; their own wording stays
    /// neutral about it.
    #[test]
    fn test_client_errors_describe_themselves() {
        assert!(ClientError::NotAccepted.to_string().contains("not accepted"));
        assert!(ClientError::ClosedDuringHandshake.to_string().contains("handshake"));
        assert_eq!(ClientError::Failed("boom".to_owned()).to_string(), "boom");
    }

    // --- the transport, over a real loopback socket ---

    /// Ports are counted rather than asked of the OS, because two tests probing one after
    /// the other can otherwise be handed the same port.
    fn free_port() -> u16 {
        use std::sync::atomic::{AtomicU16, Ordering};
        static NEXT: AtomicU16 = AtomicU16::new(50_600);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    /// Spins until `condition` holds, or fails the test rather than hanging the suite.
    #[track_caller]
    fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            if condition() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("timed out waiting for {what}");
    }

    fn collecting_logger() -> (Logger, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
        let lines = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = lines.clone();
        let logger: Logger = std::sync::Arc::new(move |level: LogLevel, message: &str| {
            sink.lock().unwrap().push(format!("{level:?}: {message}"));
        });
        (logger, lines)
    }

    /// Sends one frame of arbitrary bytes, through `message_io` so the framing itself is
    /// valid and only the contents are nonsense.
    fn send_raw_frame(port: u16, bytes: &[u8]) {
        use message_io::network::{SendStatus, Transport};
        use message_io::node;

        let (handler, _listener) = node::split::<()>();
        let (endpoint, _) = handler.network().connect(Transport::FramedTcp, format!("127.0.0.1:{port}")).unwrap();

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            match handler.network().send(endpoint, bytes) {
                SendStatus::Sent | SendStatus::ResourceNotFound => return,
                SendStatus::MaxPacketSizeExceeded => panic!("the test tried to send an oversized frame"),
                SendStatus::ResourceNotAvailable => std::thread::sleep(std::time::Duration::from_millis(1)),
            }
        }
        panic!("could not send the raw frame within the timeout");
    }

    fn listening_server(port: u16, log: Logger) -> PacketServer {
        let mut server = PacketServer::new(port, BindAddress::Loopback, log);
        server.listen();
        wait_until("the server to bind its port", || server.is_listening());
        server
    }

    /// The bind happens on the networking thread, so nothing may assume it has happened when
    /// `listen` returns. Probing the port from outside would mean binding it, which races the
    /// bind being waited for.
    #[test]
    fn test_a_server_reports_when_it_is_listening() {
        let mut server = PacketServer::new(free_port(), BindAddress::Loopback, no_logging());
        assert!(!server.is_listening(), "nothing is bound before listen is called");

        server.listen();
        wait_until("the server to bind its port", || server.is_listening());

        server.stop().unwrap();
    }

    /// A port already taken is the common way binding fails, and it surfaces through a later
    /// `poll` rather than from `listen` - so the error has to name the address, or a server
    /// that accepts nobody forever looks like one that simply started.
    #[test]
    fn test_a_port_already_taken_is_reported_by_poll() {
        let port = free_port();
        let mut first = listening_server(port, no_logging());

        let mut second = PacketServer::new(port, BindAddress::Loopback, no_logging());
        second.listen();

        let mut error = String::new();
        wait_until("the second server to fail to bind", || {
            if let Err(e) = second.poll() {
                error = e.to_string();
                return true;
            }
            false
        });

        assert!(error.contains(&format!("{port}")), "the error should name the address it could not bind: {error}");
        first.stop().unwrap();
    }

    #[test]
    fn test_a_packet_goes_both_ways() {
        let port = free_port();
        let mut server = listening_server(port, no_logging());
        let mut client = PacketClient::new("127.0.0.1".to_owned(), port, no_logging());

        client.connect(vec![Packet::new(Greeting { text: "hello".to_owned() }).unwrap()], Packet::is::<Farewell>).unwrap();

        // the greeting arrives, and with it the connection to answer on
        let mut conn = None;
        wait_until("the server to receive the greeting", || {
            for event in server.poll().unwrap() {
                if let ServerEvent::Received { conn: from, packet } = event {
                    assert_eq!(packet.try_deserialize::<Greeting>().unwrap().text, "hello");
                    conn = Some(from);
                }
            }
            conn.is_some()
        });

        let conn = conn.unwrap();
        server.send(&Packet::new(Greeting { text: "hi back".to_owned() }).unwrap(), &[conn]).unwrap();

        let mut answered = false;
        wait_until("the client to receive the answer", || {
            for received in client.poll().unwrap() {
                assert_eq!(received.packet.try_deserialize::<Greeting>().unwrap().text, "hi back");
                assert!(received.during_handshake, "nothing has ended the handshake yet");
                answered = true;
            }
            answered
        });

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// The predicate is what ends the handshake, and everything up to and including the
    /// packet that matched is marked as having arrived during it.
    #[test]
    fn test_the_handshake_ends_on_the_packet_the_owner_named() {
        let port = free_port();
        let mut server = listening_server(port, no_logging());
        let mut client = PacketClient::new("127.0.0.1".to_owned(), port, no_logging());
        client.connect(vec![Packet::new(Greeting { text: "hello".to_owned() }).unwrap()], Packet::is::<Farewell>).unwrap();

        let mut conn = None;
        wait_until("the server to see the client", || {
            for event in server.poll().unwrap() {
                if let ServerEvent::Received { conn: from, .. } = event {
                    conn = Some(from);
                }
            }
            conn.is_some()
        });
        assert!(client.is_handshaking());

        // the thread parks on this until the owner says it is ready
        client.resume_receiving();
        server.send(&Packet::new(Farewell).unwrap(), &[conn.unwrap()]).unwrap();

        wait_until("the client to finish its handshake", || {
            client.poll().unwrap();
            !client.is_handshaking()
        });

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// `connect` does not block, so a port with nothing behind it cannot be reported from
    /// there. It has to come out of the client's own state, or a caller spinning on
    /// `is_handshaking` waits for something that is never coming.
    #[test]
    fn test_connecting_to_a_closed_port_fails_instead_of_hanging() {
        let mut client = PacketClient::new("127.0.0.1".to_owned(), free_port(), no_logging());
        client.connect(vec![Packet::new(Greeting { text: "anyone?".to_owned() }).unwrap()], |_| false).unwrap();

        // the greeting is the first thing to find out, and it finds out by failing to send:
        // that ends the networking thread, so the failure comes back from `poll` itself
        let mut error = String::new();
        wait_until("the client to give up on a dead port", || match client.poll() {
            Err(e) => {
                error = e.to_string();
                true
            }
            Ok(_) => client.take_error().is_some_and(|kind| {
                error = kind.to_string();
                true
            }),
        });

        // the wording matters: this is what an owner shows when the port is empty, so
        // "Resource not found" is not good enough
        assert!(error.contains("nothing is listening") || error.contains("not accepted"), "unexpected error: {error}");
    }

    /// A peer that is dropped mid-handshake is the case a version check exists for, and the
    /// kind is what lets an owner say so.
    #[test]
    fn test_a_connection_dropped_during_the_handshake_says_so() {
        let port = free_port();
        let mut server = listening_server(port, no_logging());
        let mut client = PacketClient::new("127.0.0.1".to_owned(), port, no_logging());
        client.connect(vec![Packet::new(Greeting { text: "let me in".to_owned() }).unwrap()], Packet::is::<Farewell>).unwrap();

        let mut conn = None;
        wait_until("the server to see the client", || {
            for event in server.poll().unwrap() {
                if let ServerEvent::Received { conn: from, .. } = event {
                    conn = Some(from);
                }
            }
            conn.is_some()
        });

        server.disconnect(&conn.unwrap()).unwrap();

        let mut error = None;
        wait_until("the client to notice it was dropped", || {
            client.poll().ok();
            error = client.take_error();
            error.is_some()
        });

        assert_eq!(error, Some(ClientError::ClosedDuringHandshake));
        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// A caller that gives up mid-handshake has to be able to stop. The handshake is driven
    /// entirely by what the peer sends, so without a timer of its own there was no point at
    /// which the thread looked at the running flag and `stop` waited forever.
    #[test]
    fn test_a_client_can_be_stopped_while_it_is_still_handshaking() {
        let port = free_port();
        let mut server = listening_server(port, no_logging());
        let mut client = PacketClient::new("127.0.0.1".to_owned(), port, no_logging());
        client.connect(vec![Packet::new(Greeting { text: "waiting".to_owned() }).unwrap()], |_| false).unwrap();

        wait_until("the server to see the client", || {
            server.poll().unwrap().iter().any(|event| matches!(event, ServerEvent::Received { .. }))
        });
        assert!(client.is_handshaking());

        // on a thread of its own, so a regression fails this test instead of hanging the suite
        let (stopped_sender, stopped_receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || stopped_sender.send(client.stop()).unwrap_or(()));

        let stopped = stopped_receiver.recv_timeout(std::time::Duration::from_secs(10));
        assert!(stopped.is_ok(), "stopping a client that was still handshaking never returned");
        stopped.unwrap().unwrap();

        server.stop().unwrap();
    }

    /// A malformed frame from one peer must not take networking down for everyone.
    #[test]
    fn test_a_malformed_frame_does_not_stop_the_server() {
        let port = free_port();
        let (log, lines) = collecting_logger();
        let mut server = listening_server(port, log);

        // one well formed frame whose contents are not a Packet at all
        send_raw_frame(port, &[0xff, 0x00, 0x13, 0x37, 0xff, 0xff, 0xff, 0xff]);

        wait_until("the server to complain about the frame", || {
            server.poll().unwrap();
            lines.lock().unwrap().iter().any(|line| line.contains("could not be deserialized"))
        });

        // and it is still serving
        assert!(server.is_listening());
        assert!(server.poll().is_ok(), "a malformed frame must not have killed the networking thread");
        server.stop().unwrap();
    }
}
