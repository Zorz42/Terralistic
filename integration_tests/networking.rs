//! The wire protocol, driven end to end over a real loopback socket.
//!
//! Neither networking module had a test before: they are the two halves of a protocol
//! with no registry and no negotiation beyond the version packet, so the only way to
//! check they still agree is to run them against each other.
#![allow(clippy::unwrap_used, clippy::panic)] // tests assert on results directly
mod tests {
    use crate::integration_tests::harness::{complete_handshake, free_port, wait_until, NetServer, RawClient, TestClient};
    use crate::libraries::serialization;
    use crate::server::server_core::SendTarget;
    use crate::shared::chat::ChatPacket;
    use crate::shared::packet::{Packet, WelcomeCompletePacket};
    use crate::shared::players::NamePacket;
    use crate::shared::versions::{VersionPacket, VERSION};

    /// The documented handshake: client sends its version and name, server answers with
    /// `WelcomeCompletePacket`, client leaves welcoming mode.
    #[test]
    fn test_client_and_server_complete_the_handshake() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        wait_until("the handshake to complete", || {
            server.pump()?;
            client.pump()?;
            Ok(!server.joined.is_empty() && client.received::<WelcomeCompletePacket>())
        });

        assert_eq!(server.joined, vec!["Player".to_owned()]);
        assert!(!client.net.is_welcoming(), "client should leave welcoming once welcomed");

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// The name is carried by `NamePacket` and is what the server files the connection
    /// under, so it has to survive the round trip verbatim - including characters that a
    /// fixed width encoding would have mangled.
    #[test]
    // the point of the test is a name that is not ascii, so the literal has to be one
    #[allow(clippy::non_ascii_literal)]
    fn test_the_name_survives_the_handshake() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let name = "Ana Sofía ✧ 玩家";
        let mut client = TestClient::connect(port, name).unwrap();

        complete_handshake(&mut server, &mut client);

        assert_eq!(server.joined.first().unwrap(), name);

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// A client built from a different release is refused before it can act on packets it
    /// would not understand. Without this the two just misread each other's ids silently.
    #[test]
    fn test_server_refuses_a_mismatched_version() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let client = RawClient::connect(port).unwrap();

        client.send(VersionPacket {
            version: "0.0.0-not-this-build".to_owned(),
        });
        client.send(NamePacket { name: "Old".to_owned() });

        wait_until("the server to drop the mismatched client", || {
            server.pump()?;
            Ok(client.is_disconnected())
        });

        assert!(server.joined.is_empty(), "a mismatched client must not be let in");

        server.stop().unwrap();
    }

    /// A client old enough not to send a version at all is refused too. The server can
    /// only tell by the absence of the packet, so this is checked separately.
    #[test]
    fn test_server_refuses_a_client_that_sends_no_version() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let client = RawClient::connect(port).unwrap();

        client.send(NamePacket { name: "Ancient".to_owned() });

        wait_until("the server to drop the versionless client", || {
            server.pump()?;
            Ok(client.is_disconnected())
        });

        assert!(server.joined.is_empty(), "a client with no version must not be let in");

        server.stop().unwrap();
    }

    /// The matching version is accepted, which is what stops the two tests above from
    /// passing for the wrong reason.
    #[test]
    fn test_server_accepts_the_current_version() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let client = RawClient::connect(port).unwrap();

        client.send(VersionPacket { version: VERSION.to_owned() });
        client.send(NamePacket { name: "Current".to_owned() });

        wait_until("the server to accept the current version", || {
            server.pump()?;
            Ok(!server.joined.is_empty())
        });

        assert!(!client.is_disconnected());

        // and it is welcomed rather than merely tolerated
        wait_until("the welcome to come back", || {
            server.pump()?;
            Ok(client.received_packets().iter().any(|packet| packet.try_deserialize::<WelcomeCompletePacket>().is_some()))
        });

        server.stop().unwrap();
    }

    /// A frame that is not a packet at all is dropped with a warning. It used to be able
    /// to take the whole networking thread down, which would disconnect every other
    /// player because one client sent nonsense.
    #[test]
    fn test_a_malformed_frame_does_not_stop_the_server() {
        let port = free_port();
        let mut server = NetServer::start(port);

        let vandal = RawClient::connect(port).unwrap();
        vandal.send_bytes(&[0xff, 0x00, 0x13, 0x37, 0xff, 0xff, 0xff, 0xff]);

        // the server is still serving: a well behaved client that connects afterwards
        // still gets through
        let mut client = TestClient::connect(port, "Player").unwrap();
        wait_until("a later client to be welcomed", || {
            server.pump()?;
            client.pump()?;
            Ok(client.received::<WelcomeCompletePacket>())
        });

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// Packets sent from the game code reach the client, and arrive as the type they were
    /// sent as. This is the whole of the "any serializable struct is a packet" contract.
    #[test]
    fn test_a_packet_reaches_the_client() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        complete_handshake(&mut server, &mut client);

        server
            .net
            .send_packet(
                &Packet::new(ChatPacket {
                    message: "hello from the server".to_owned(),
                })
                .unwrap(),
                SendTarget::All,
            )
            .unwrap();

        wait_until("the chat packet to arrive", || {
            server.pump()?;
            client.pump()?;
            Ok(client.received::<ChatPacket>())
        });

        assert_eq!(client.find::<ChatPacket>().unwrap().message, "hello from the server");

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// And the other direction: a packet the client sends turns up as a
    /// `PacketFromClientEvent` tagged with the connection it came from.
    #[test]
    fn test_a_packet_reaches_the_server() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        complete_handshake(&mut server, &mut client);

        client
            .net
            .send_packet(
                Packet::new(ChatPacket {
                    message: "hello from the client".to_owned(),
                })
                .unwrap(),
            )
            .unwrap();

        wait_until("the chat packet to reach the server", || {
            server.pump()?;
            client.pump()?;
            Ok(!server.packets.is_empty())
        });

        let (packet, conn) = server.packets.first().unwrap();
        assert_eq!(packet.try_deserialize::<ChatPacket>().unwrap().message, "hello from the client");
        assert!(conn == server.connections.first().unwrap(), "the packet was attributed to the wrong connection");

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// `SendTarget::All` reaches every connection and `AllExcept` skips one. Both are used
    /// to broadcast world changes, so getting them the wrong way round would either
    /// desync one player or echo an action back to whoever caused it.
    #[test]
    fn test_send_targets_pick_the_right_clients() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut first = TestClient::connect(port, "First").unwrap();
        let mut second = TestClient::connect(port, "Second").unwrap();

        wait_until("both clients to be welcomed", || {
            server.pump()?;
            first.pump()?;
            second.pump()?;
            Ok(first.received::<WelcomeCompletePacket>() && second.received::<WelcomeCompletePacket>())
        });
        assert_eq!(server.connections.len(), 2);

        // the connection order is whatever the network gave us, so find the one that
        // belongs to the second client by name rather than assuming
        let second_index = server.joined.iter().position(|name| name == "Second").unwrap();
        let second_conn = server.connections.get(second_index).unwrap().clone();

        server.net.send_packet(&Packet::new(ChatPacket { message: "everyone".to_owned() }).unwrap(), SendTarget::All).unwrap();
        server
            .net
            .send_packet(&Packet::new(NamePacket { name: "not-second".to_owned() }).unwrap(), SendTarget::AllExcept(second_conn))
            .unwrap();

        wait_until("the broadcast to arrive", || {
            server.pump()?;
            first.pump()?;
            second.pump()?;
            Ok(first.received::<NamePacket>() && second.received::<ChatPacket>())
        });

        // both got the broadcast
        assert!(first.received::<ChatPacket>());
        assert!(second.received::<ChatPacket>());
        // only the excluded one missed the second packet. The ordering above is what
        // makes this safe to assert: the packets were queued in order on one channel, so
        // the second client having the later packet means it would have had this one too.
        assert!(!second.received::<NamePacket>(), "AllExcept sent to the excluded connection");

        first.stop().unwrap();
        second.stop().unwrap();
        server.stop().unwrap();
    }

    /// A client going away produces a `DisconnectEvent`, which is what frees its player
    /// slot. Losing it leaks the connection into every later broadcast.
    #[test]
    fn test_a_leaving_client_produces_a_disconnect_event() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        complete_handshake(&mut server, &mut client);

        client.stop().unwrap();

        wait_until("the disconnect to be noticed", || {
            server.pump()?;
            Ok(!server.disconnected.is_empty())
        });

        assert!(
            server.disconnected.first().unwrap() == server.connections.first().unwrap(),
            "the disconnect was attributed to the wrong connection"
        );

        server.stop().unwrap();
    }

    /// Stopping the server reports every still open connection as disconnected, so the
    /// rest of the game tears those players down rather than leaving them in the world.
    #[test]
    fn test_stopping_the_server_disconnects_everyone() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        complete_handshake(&mut server, &mut client);

        server.stop().unwrap();

        let mut disconnects = 0;
        while let Some(event) = server.events.pop_event() {
            if event.downcast::<crate::server::server_core::DisconnectEvent>().is_some() {
                disconnects += 1;
            }
        }
        assert_eq!(disconnects, 1);

        client.stop().unwrap();
    }

    /// Connecting to a port with nothing behind it has to end in an error rather than a
    /// wait that never ends.
    ///
    /// This one the client already got right: the connection is refused outright, so the
    /// networking thread returns the error and `update` reports it when it joins the
    /// finished thread. `run_game`'s welcome loop polls exactly this way.
    #[test]
    fn test_connecting_to_a_closed_port_fails_instead_of_hanging() {
        let port = free_port();
        let mut client = TestClient::connect(port, "Player").unwrap();

        let mut error = String::new();
        wait_until("the client to give up on a dead server", || {
            if let Err(e) = client.pump() {
                error = e.to_string();
                return Ok(true);
            }
            Ok(false)
        });

        assert!(!error.is_empty());
    }

    /// And the same for a server that accepts the socket and then refuses the client.
    ///
    /// This is the case the version handshake exists for, so hanging here defeated the
    /// point: the client would sit on the loading screen instead of reporting that the
    /// server would not have it.
    #[test]
    fn test_a_refused_client_reports_it_instead_of_hanging() {
        let port = free_port();
        let mut server = NetServer::start(port);

        // a server that never welcomes anyone: it drops the connection as soon as it
        // sees the name, the same way the version check does
        let mut client = TestClient::connect(port, "Player").unwrap();
        wait_until("the server to see the client", || {
            server.pump_without_answering()?;
            Ok(!server.connections.is_empty())
        });
        server.stop().unwrap();

        wait_until("the client to notice it was dropped", || Ok(!client.net.is_welcoming()));

        let error = client.pump().unwrap_err().to_string();
        assert!(error.contains("refused to let this client join"), "unexpected error: {error}");

        client.stop().unwrap();
    }

    /// A client that gives up in the middle of the handshake can be stopped.
    ///
    /// This is a player closing the window while a world is still loading. The welcome phase is
    /// driven entirely by what the server sends, so with nothing else ticking there was no point
    /// at which the thread looked at the running flag, and `stop` waited on a thread that was
    /// never going to end. The game could not offer to quit during a join at all.
    ///
    /// The stop runs on a thread of its own so that a regression fails this test instead of
    /// hanging it - which, in a suite with no per-test timeout, is the difference between a red
    /// build and a job that runs until CI kills it.
    #[test]
    fn test_a_client_can_be_stopped_while_it_is_still_welcoming() {
        let port = free_port();
        let mut server = NetServer::start(port);
        let mut client = TestClient::connect(port, "Player").unwrap();

        // the server takes the socket and says nothing back, the way one that is still
        // generating a world does
        wait_until("the server to see the client", || {
            server.pump_without_answering()?;
            Ok(!server.connections.is_empty())
        });
        assert!(client.net.is_welcoming(), "the client should still be waiting to be welcomed");

        let (stopped_sender, stopped_receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || stopped_sender.send(client.stop()).unwrap_or(()));

        let stopped = stopped_receiver.recv_timeout(std::time::Duration::from_secs(10));
        assert!(stopped.is_ok(), "stopping a client that was still welcoming never returned");
        stopped.unwrap().unwrap();

        server.stop().unwrap();
    }

    /// Packet ids are a hash of the rust type, so the same struct has to hash the same on
    /// both sides of the socket - that is the only thing keeping the protocol coherent.
    /// A packet that arrives is offered to every type in turn, and only the right one
    /// answers.
    #[test]
    fn test_a_packet_only_deserializes_as_its_own_type() {
        let packet = Packet::new(ChatPacket { message: "hello".to_owned() }).unwrap();
        let bytes = serialization::serialize(&packet).unwrap();
        let received: Packet = serialization::deserialize(&bytes).unwrap();

        assert!(received.try_deserialize::<NamePacket>().is_none());
        assert!(received.try_deserialize::<WelcomeCompletePacket>().is_none());
        assert_eq!(received.try_deserialize::<ChatPacket>().unwrap().message, "hello");
    }
}
