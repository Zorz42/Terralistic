//! A real client against a real server: joining, being spawned, chatting, running
//! commands and changing the world.
//!
//! This is the path that nothing else covers. Client and server keep separate copies of
//! the world and agree only by sending each other packets, so a desync does not fail
//! anywhere - it just means two players see different things. These tests hold both
//! copies at once and compare them.
#![allow(clippy::unwrap_used)] // tests assert on results directly
mod tests {
    use crate::integration_tests::harness::{join, wait_until, TestServer};
    use crate::libraries::events::EventManager;
    use crate::shared::blocks::{BlockBreakStartPacket, BlockChangePacket, Blocks, BlocksWelcomePacket, ClientBlockBreakStartPacket};
    use crate::shared::chat::ChatPacket;
    use crate::shared::entities::PositionComponent;
    use crate::shared::mod_manager::ModsWelcomePacket;
    use crate::shared::packet::{Packet, WelcomeCompletePacket};
    use crate::shared::players::PlayerSpawnPacket;
    use crate::shared::walls::WallsWelcomePacket;

    fn server(tag: &str) -> TestServer {
        TestServer::start_on_small_world(tag, (60, 40)).unwrap()
    }

    /// The welcome sequence, in full: everything the client needs before it can render a
    /// frame arrives before the server says the handshake is over.
    #[test]
    fn test_a_joining_client_receives_the_world() {
        let mut server = server("join-world");
        let mut client = join(&mut server, "Player").unwrap();

        assert!(client.received::<ModsWelcomePacket>(), "the client was not sent the mods");
        assert!(client.received::<BlocksWelcomePacket>(), "the client was not sent the blocks");
        assert!(client.received::<WallsWelcomePacket>(), "the client was not sent the walls");
        assert!(client.received::<WelcomeCompletePacket>());

        // and all of it arrived during the welcome phase, not after
        assert!(client.welcome_packets.len() >= 4, "the world arrived after the welcome, so a client would start playing without it");

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// The world the client is sent is the world the server has. This is the desync check:
    /// the client deserializes the packet into its own `Blocks` exactly as `ClientBlocks`
    /// does, and the two grids have to agree block for block.
    #[test]
    fn test_the_world_the_client_receives_matches_the_server() {
        let mut server = server("join-blocks");

        // put something recognisable in the world before anyone connects
        let stone = {
            let mut blocks = server.server.get_blocks();
            let stone = blocks.get_block_id_by_name("stone_block").unwrap();
            let mut events = EventManager::new();
            blocks.set_block(&mut events, 5, 6, stone).unwrap();
            stone
        };

        let mut client = join(&mut server, "Player").unwrap();

        let packet = client.find::<BlocksWelcomePacket>().unwrap();
        let mut client_blocks = Blocks::new();
        client_blocks.deserialize(&packet.data).unwrap();

        let server_blocks = server.server.get_blocks();
        assert_eq!(client_blocks.get_size(), server_blocks.get_size(), "the client got a world of a different size");

        let (width, height) = server_blocks.get_size();
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                assert!(
                    client_blocks.get_block(x, y).unwrap() == server_blocks.get_block(x, y).unwrap(),
                    "the client and the server disagree about the block at ({x}, {y})"
                );
            }
        }
        assert!(client_blocks.get_block(5, 6).unwrap() == stone);

        drop(server_blocks);
        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// Joining spawns a player, on the server and in the client's copy of the world.
    #[test]
    fn test_joining_spawns_a_player() {
        let mut server = server("join-spawn");
        let mut client = join(&mut server, "Player").unwrap();

        wait_until("the player to be spawned", || {
            server.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        let spawn = client.find::<PlayerSpawnPacket>().unwrap();
        assert_eq!(spawn.name, "Player");

        // the server has an entity for it too, not just a packet about one
        let entities = server.server.get_entities();
        assert!(entities.get_entity_from_id(spawn.id).is_ok(), "the server sent a spawn for an entity it does not have");

        drop(entities);
        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// A second player sees the first, which is what makes the world shared rather than
    /// two people playing alone in the same file.
    #[test]
    fn test_a_second_client_is_told_about_the_first() {
        let mut server = server("join-two");
        let mut first = join(&mut server, "First").unwrap();
        let mut second = join(&mut server, "Second").unwrap();

        wait_until("both players to be known to the second client", || {
            server.server.update()?;
            first.pump()?;
            second.pump()?;

            let names: Vec<String> = second
                .welcome_packets
                .iter()
                .chain(second.packets.iter())
                .filter_map(Packet::try_deserialize::<PlayerSpawnPacket>)
                .map(|spawn| spawn.name)
                .collect();
            Ok(names.contains(&"First".to_owned()) && names.contains(&"Second".to_owned()))
        });

        first.stop().unwrap();
        second.stop().unwrap();
        server.stop().unwrap();
    }

    /// Chat goes to everyone, with the sender's name attached by the server rather than
    /// by the client - so a client cannot claim to be someone else.
    #[test]
    fn test_a_chat_message_reaches_the_other_player() {
        let mut server = server("chat");
        let mut sender = join(&mut server, "Sender").unwrap();
        let mut listener = join(&mut server, "Listener").unwrap();

        sender.net.send_packet(Packet::new(ChatPacket { message: "hello".to_owned() }).unwrap()).unwrap();

        let mut heard = String::new();
        wait_until("the chat message to reach the other player", || {
            server.server.update()?;
            sender.pump()?;
            listener.pump()?;

            for packet in &listener.packets {
                if let Some(chat) = packet.try_deserialize::<ChatPacket>() {
                    heard = chat.message;
                    return Ok(true);
                }
            }
            Ok(false)
        });

        assert_eq!(heard, "Sender: hello", "the server attributed the message to the wrong player");

        sender.stop().unwrap();
        listener.stop().unwrap();
        server.stop().unwrap();
    }

    /// A chat message starting with a slash is a command, and its output goes back to
    /// whoever ran it rather than to everyone.
    #[test]
    fn test_a_command_from_chat_answers_only_the_sender() {
        let mut server = server("chat-command");
        let mut sender = join(&mut server, "Sender").unwrap();
        let mut listener = join(&mut server, "Listener").unwrap();

        sender.net.send_packet(Packet::new(ChatPacket { message: "/help".to_owned() }).unwrap()).unwrap();

        wait_until("the command output to come back", || {
            server.server.update()?;
            sender.pump()?;
            listener.pump()?;
            Ok(sender.packets.iter().any(|packet| packet.try_deserialize::<ChatPacket>().is_some()))
        });

        let answer = sender.packets.iter().find_map(Packet::try_deserialize::<ChatPacket>).unwrap();
        assert!(answer.message.contains("/help"), "the command output is not help text: {}", answer.message);

        // and the other player was not shown it
        assert!(
            !listener.packets.iter().any(|packet| packet.try_deserialize::<ChatPacket>().is_some()),
            "a command's output was broadcast to everyone"
        );

        sender.stop().unwrap();
        listener.stop().unwrap();
        server.stop().unwrap();
    }

    /// Breaking a block, all the way through: the client asks, the server simulates the
    /// break over several ticks, changes its own world, and tells every client.
    ///
    /// `wood_planks` is the block to break bare handed. It has no effective tool, so
    /// empty hands are enough, and unlike `grass_block` it stays what it is - a grass
    /// block placed in mid air turns itself into dirt on the next block update, and dirt
    /// wants a shovel.
    #[test]
    fn test_a_client_can_break_a_block() {
        let mut server = server("break-block");

        let planks = {
            let mut blocks = server.server.get_blocks();
            let planks = blocks.get_block_id_by_name("wood_planks").unwrap();
            let mut events = EventManager::new();
            blocks.set_block(&mut events, 10, 12, planks).unwrap();
            planks
        };

        let mut client = join(&mut server, "Player").unwrap();

        // the server only starts breaking for a connection it has a player for
        wait_until("the player to be spawned", || {
            server.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        assert!(server.server.get_blocks().get_block(10, 12).unwrap() == planks);

        client.net.send_packet(Packet::new(ClientBlockBreakStartPacket { x: 10, y: 12 }).unwrap()).unwrap();

        wait_until("the server to start breaking the block", || {
            server.update_slowly()?;
            client.pump()?;
            Ok(client.received::<BlockBreakStartPacket>())
        });

        wait_until("the block to break", || {
            server.update_slowly()?;
            client.pump()?;
            // one lock at a time: `get_blocks` takes the server's mutex, and asking for
            // it twice in one expression waits on a lock this thread is already holding
            let blocks = server.server.get_blocks();
            let is_air = blocks.get_block(10, 12).unwrap() == blocks.air();
            Ok(is_air)
        });

        // and the client was told, so its copy of the world can follow
        wait_until("the client to be told the block changed", || {
            server.server.update()?;
            client.pump()?;
            Ok(client
                .packets
                .iter()
                .filter_map(Packet::try_deserialize::<BlockChangePacket>)
                .any(|change| change.x == 10 && change.y == 12))
        });

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// A block that names an effective tool cannot be broken without it. The server does
    /// not answer at all in that case, so a client that tries gets nothing rather than a
    /// break it can see and the server does not.
    #[test]
    fn test_a_client_cannot_break_a_block_it_has_no_tool_for() {
        let mut server = server("break-needs-tool");

        let dirt = {
            let mut blocks = server.server.get_blocks();
            let dirt = blocks.get_block_id_by_name("dirt").unwrap();
            let mut events = EventManager::new();
            blocks.set_block(&mut events, 10, 12, dirt).unwrap();
            dirt
        };

        let mut client = join(&mut server, "Player").unwrap();
        wait_until("the player to be spawned", || {
            server.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        client.net.send_packet(Packet::new(ClientBlockBreakStartPacket { x: 10, y: 12 }).unwrap()).unwrap();

        // long enough that a block with no tool requirement would have been gone twice
        // over: dirt takes 700ms to break and this runs for a good deal more than that
        for _ in 0..400 {
            server.update_slowly().unwrap();
            client.pump().unwrap();
        }

        assert!(server.server.get_blocks().get_block(10, 12).unwrap() == dirt, "dirt was broken with bare hands");
        assert!(!client.received::<BlockBreakStartPacket>(), "the server told the client a break had started that it never started");

        client.stop().unwrap();
        server.stop().unwrap();
    }

    /// A player who leaves is removed, so the next broadcast does not go to a socket that
    /// is not there and the world is not left with a motionless body in it.
    #[test]
    fn test_a_leaving_player_is_removed_from_the_world() {
        let mut server = server("leave");
        let mut client = join(&mut server, "Player").unwrap();

        wait_until("the player to be spawned", || {
            server.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        let spawned = client.find::<PlayerSpawnPacket>().unwrap();
        client.stop().unwrap();

        wait_until("the player to be despawned", || {
            server.server.update()?;
            Ok(server.server.get_entities().get_entity_from_id(spawned.id).is_err())
        });

        server.stop().unwrap();
    }

    /// A player's position is saved with the world and restored when they come back,
    /// which is what makes a world worth returning to.
    ///
    /// The player is left to fall for a while first, so the position being restored is
    /// clearly the one they left from and not the spawn point they would get anyway.
    #[test]
    fn test_a_player_is_remembered_across_a_restart() {
        let mut server = server("player-persist");
        let mut client = join(&mut server, "Returning").unwrap();

        wait_until("the player to be spawned", || {
            server.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        let first_spawn = client.find::<PlayerSpawnPacket>().unwrap();

        // an all air world, so the player falls away from where they started
        for _ in 0..100 {
            server.update_slowly().unwrap();
            client.pump().unwrap();
        }

        let left_at = {
            let entities = server.server.get_entities();
            let entity = entities.get_entity_from_id(first_spawn.id).unwrap();
            let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
            (position.x(), position.y())
        };
        assert!(left_at.1 > first_spawn.y + 1.0, "the player did not move, so this would pass without remembering anything");

        client.stop().unwrap();

        wait_until("the player to be despawned", || {
            server.server.update()?;
            Ok(server.server.get_entities().get_entity_from_id(first_spawn.id).is_err())
        });

        let dir = server.stop().unwrap();

        let mut reloaded = TestServer::start_in(dir, "player-persist").unwrap();
        let mut client = join(&mut reloaded, "Returning").unwrap();

        wait_until("the player to be spawned again", || {
            reloaded.server.update()?;
            client.pump()?;
            Ok(client.received::<PlayerSpawnPacket>())
        });

        let second_spawn = client.find::<PlayerSpawnPacket>().unwrap();
        assert_eq!(second_spawn.name, "Returning");

        // a block of slack: the player keeps falling for the few milliseconds between
        // reading the position above and the server noticing the disconnect
        assert!(
            (second_spawn.x - left_at.0).abs() < 1.0 && (second_spawn.y - left_at.1).abs() < 1.0,
            "the player came back somewhere else: left at ({}, {}), came back at ({}, {})",
            left_at.0,
            left_at.1,
            second_spawn.x,
            second_spawn.y
        );

        client.stop().unwrap();
        reloaded.stop().unwrap();
    }

    /// The server keeps running when a client vanishes without saying goodbye, which is
    /// what a crash or a pulled cable looks like from its side.
    #[test]
    fn test_the_server_survives_a_client_disappearing() {
        let mut server = server("client-vanishes");
        let mut client = join(&mut server, "Player").unwrap();
        client.stop().unwrap();

        for _ in 0..50 {
            server.server.update().unwrap();
        }

        // and it will still take a new one
        let mut second = join(&mut server, "Second").unwrap();
        assert!(second.received::<WelcomeCompletePacket>());

        second.stop().unwrap();
        server.stop().unwrap();
    }
}
