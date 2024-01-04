use std::fmt::Write;

use anyhow::{bail, Result};

use crate::libraries::events::Event;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::networking::{PacketFromClientEvent, SendTarget, ServerNetworking};
use crate::server::server_core::players::ServerPlayers;
use crate::server::server_core::send_to_ui;
use crate::server::server_ui::{ConsoleMessageType, UiMessageType};
use crate::shared::chat::ChatPacket;
use crate::shared::entities::EntityId;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::players::PlayerComponent;

struct Command {
    name: String,
    description: String,
    from_mod: String,
}

pub struct CommandManager {
    commands: Vec<Command>,
}

impl CommandManager {
    pub const fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn init(&mut self, mod_manager: &mut ModManager) {
        for game_mod in mod_manager.mods_iter_mut() {
            for global_symbol in game_mod.get_all_symbols() {
                if global_symbol.starts_with("command_") {
                    let command_name = global_symbol.get(8..).unwrap_or("").to_owned();
                    let command_description = game_mod
                        .call_function(&format!("describe_command_{command_name}"), ())
                        .unwrap_or_else(|_| "No description provided.".to_owned());

                    self.commands.push(Command {
                        name: command_name,
                        description: command_description,
                        from_mod: game_mod.get_name().to_owned(),
                    });
                }
            }
        }
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
                    let name = entities.get_entities().ecs.get::<&mut PlayerComponent>(player_entity)?.get_name().to_owned();
                    let player_id = entities.get_entities().get_id_from_entity(player_entity)?;

                    let mut output = String::new();
                    let result = self.execute_command(&command, mod_manager, Some(player_id));

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

    pub fn execute_command(&self, command: &str, mod_manager: &mut ModManager, executor: Option<EntityId>) -> Result<String> {
        // split the command into Vec<String>
        let mut arguments = command.split_whitespace().map(std::borrow::ToOwned::to_owned).collect::<Vec<_>>();
        let command_name = arguments.remove(0);
        let function_name = format!("command_{command_name}");

        if command_name == "help" {
            return self.help_command(&arguments);
        }

        // go through all mods and search for the function
        for game_mod in mod_manager.mods_iter_mut() {
            if game_mod.is_symbol_defined(&function_name)? {
                let result = game_mod.call_function(&function_name, (arguments, executor))?;
                return Ok(result);
            }
        }

        Ok(format!("Unknown command: \"{command_name}\""))
    }

    fn help_command(&self, arguments: &Vec<String>) -> Result<String> {
        if arguments.is_empty() {
            let mut result = "/help - builtin command
/help - displays all commands and their descriptions
/help <command> - displays a description of a specific command\n"
                .to_owned();
            for command in &self.commands {
                let help = Self::get_command_help(command);
                write!(result, "\n{help}\n")?;
            }
            Ok(result)
        } else if arguments.len() == 1 {
            let mut result = String::new();
            let command_name = arguments.first().map_or("", |s| s.as_str());

            for command in &self.commands {
                if command.name == command_name {
                    let help = Self::get_command_help(command);
                    write!(result, "\n{help}\n")?;
                }
            }

            if result.is_empty() {
                bail!("Unknown command: \"{command_name}\"")
            }

            Ok(result)
        } else {
            bail!("Help only takes 0 or 1 arguments.")
        }
    }

    fn get_command_help(command: &Command) -> String {
        format!("/{} - from \"{}\"\n{}", command.name, command.from_mod, command.description)
    }
}
