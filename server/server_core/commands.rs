use std::fmt::Write;

use anyhow::Result;

use crate::libraries::events::Event;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::networking::{PacketFromClientEvent, SendTarget, ServerNetworking};
use crate::server::server_core::players::ServerPlayers;
use crate::server::server_core::send_to_ui;
use crate::server::server_ui::{ConsoleMessageType, UiMessageType};
use crate::shared::chat::ChatPacket;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::players::PlayerComponent;

/// contains all the commands
pub struct CommandManager {}

impl CommandManager {
    pub const fn new() -> Self {
        Self {}
    }

    /// receives an event and executes the command
    pub fn on_event(&self, event: &Event, players: &ServerPlayers, entities: &ServerEntities, networking: &mut ServerNetworking, mod_manager: &mut ModManager) -> Result<()> {
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
                    let result = self.execute_command(&command, mod_manager);

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

    pub fn execute_command(&self, command: &str, mod_manager: &mut ModManager) -> Result<String> {
        // split the command into Vec<String>
        let mut command = command.split_whitespace().map(std::borrow::ToOwned::to_owned).collect::<Vec<_>>();
        let command_name = command.remove(0);
        let function_name = format!("command_{command_name}");

        // go through all mods and search for the function
        for game_mod in mod_manager.mods_iter_mut() {
            if game_mod.is_symbol_defined(&function_name)? {
                let result = game_mod.call_function(&function_name, command)?;
                return Ok(result);
            }
        }

        Ok(format!("Unknown command: \"{command_name}\""))
    }
}
