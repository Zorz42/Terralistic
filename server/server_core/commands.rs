use std::fmt::Write;

use anyhow::{Error, Result};

use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::{PacketFromClientEvent, SendTarget, ServerNetworking};
use crate::server::server_core::{entities, players};
use crate::server::server_core::{items, send_to_ui};
use crate::server::server_ui::{ConsoleMessageType, ServerState, UiMessageType};
use crate::shared::chat::ChatPacket;
use crate::shared::packet::Packet;
use crate::shared::players::PlayerComponent;

/// struct that contains the command name and the function that will be executed when the command is called
pub struct Command {
    pub call_name: String,
    pub name: String,
    pub description: String,
}

/// contains all the commands
pub struct CommandManager {
    pub commands: Vec<Command>,
}

impl CommandManager {
    pub const fn new() -> Self {
        Self { commands: Vec::new() }
    }

    /// adds a command to the command manager
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// receives an event and executes the command
    pub fn on_event(&self, event: &Event, players: &mut players::ServerPlayers, entities: &mut entities::ServerEntities, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<ChatPacket>() {
                let mut command = packet.message;

                if !command.starts_with('/') {
                    return Ok(());
                }

                // remove the slash
                command = command.get(1..).unwrap_or("").to_owned();

                let player_entity = players.get_player_from_connection(&event.conn)?;
                if let Some(player_entity) = player_entity {
                    let name = entities.entities.ecs.get::<&mut PlayerComponent>(player_entity)?.get_name().to_owned();

                    let mut output = String::new();
                    let result = self.execute_command(&command);

                    writeln!(output, "Player \"{name}\" executed a command: {command}")?;
                    let result = result.unwrap_or_else(|e| format!("Error: {e}"));

                    writeln!(output, "Command result: {result:?}")?;

                    send_to_ui(UiMessageType::SrvToUiConsoleMessage(ConsoleMessageType::Info(output)), None);

                    let packet = Packet::new(ChatPacket { message: result })?;

                    networking.send_packet(&packet, SendTarget::Connection(event.conn.clone()))?;
                }
            }
        }

        Ok(())
    }

    pub fn execute_command(&self, command: &str) -> Result<String> {
        Ok("".to_owned())
    }
}
