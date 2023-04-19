use crate::libraries::events::EventManager;
use crate::server::server_core::{entities, players};
use crate::server::server_core::{items, send_to_ui};
use crate::server::server_ui::{ConsoleMessageType, ServerState, UiMessageType};
use crate::shared::inventory::Inventory;
use crate::shared::items::ItemStack;
use anyhow::{anyhow, Error};
use core::fmt::Write;
use std::sync::mpsc::{Receiver, Sender};

/**
 * This struct contains all parameters that are needed to execute any command
 * It is used so that when a new argument is needed to be added to a command, it can be added here and all commands will be updated
 */
#[allow(clippy::single_char_lifetime_names)]
pub struct CommandParameters<'a> {
    pub command_manager: &'a CommandManager,
    pub state: &'a mut ServerState,
    pub executor: Option<String>,
    pub players: &'a mut players::ServerPlayers,
    pub items: &'a mut items::ServerItems,
    pub entities: &'a mut entities::ServerEntities,
    pub event_manager: &'a mut EventManager,
    pub arguments: Vec<String>,
}

//struct that contains the command name and the function that will be executed when the command is called
pub struct Command {
    pub call_name: String,
    pub name: String,
    pub description: String,
    pub function: fn(&mut CommandParameters) -> anyhow::Result<String>,
}

//contains all the commands
pub struct CommandManager {
    pub commands: Vec<Command>,
}

impl CommandManager {
    pub const fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    //adds a command to the command manager
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    //executes a command
    #[allow(clippy::too_many_arguments)]
    pub fn execute_command(
        &self,
        command: &str,
        state: &mut ServerState,
        executor: Option<String>,
        players: &mut players::ServerPlayers,
        items: &mut items::ServerItems,
        entities: &mut entities::ServerEntities,
        event_manager: &mut EventManager,
    ) -> anyhow::Result<String> {
        let arguments: Vec<String> = command
            .split(' ')
            .map(alloc::borrow::ToOwned::to_owned)
            .collect();

        for c in &self.commands {
            if c.call_name == *arguments.get(0).unwrap_or(&String::new()) {
                return (c.function)(&mut CommandParameters {
                    //returns the feedback message from the command
                    command_manager: self,
                    state,
                    executor,
                    players,
                    items,
                    entities,
                    event_manager,
                    arguments: arguments.get(1..).unwrap_or(&[]).to_vec(),
                });
            }
        }
        Err(Error::msg("Invalid command"))
    }

    //executes all commands
    #[allow(clippy::too_many_arguments)]
    pub fn execute_commands(
        &self,
        receiver: &Receiver<UiMessageType>,
        ui_message_sender: &Option<Sender<UiMessageType>>,
        state: &mut ServerState,
        players: &mut players::ServerPlayers,
        items: &mut items::ServerItems,
        entities: &mut entities::ServerEntities,
        event_manager: &mut EventManager,
    ) {
        //goes through the messages received from the server
        while let Ok(UiMessageType::UiToSrvConsoleMessage(message)) = receiver.try_recv() {
            println!("[server UI console input] {message}");
            let feedback = self.execute_command(
                &message,
                state,
                None,
                players,
                items,
                entities,
                event_manager,
            );
            let feedback = match feedback {
                Ok(feedback) => {
                    println!("{feedback}");
                    ConsoleMessageType::Info(feedback)
                }
                Err(val) => ConsoleMessageType::Warning(val.to_string()),
            };
            send_to_ui(
                UiMessageType::SrvToUiConsoleMessage(feedback),
                ui_message_sender,
            );
        }
    }
}

//help command
#[allow(clippy::unnecessary_wraps)] //all command functions must return the same type
pub fn help(parameters: &mut CommandParameters) -> anyhow::Result<String> {
    let mut string = String::new();
    string.push_str("Commands:\n");
    for c in &parameters.command_manager.commands {
        writeln!(string, "{} - {}", c.call_name, c.description)?;
    }
    anyhow::Ok(string)
}

#[allow(clippy::unnecessary_wraps)] //all command functions must return the same type
pub fn stop(parameters: &mut CommandParameters) -> anyhow::Result<String> {
    *parameters.state = ServerState::Stopping;
    anyhow::Ok(String::from("Stopping server..."))
}

//this command gives an item to the player
pub fn give(parameters: &mut CommandParameters) -> anyhow::Result<String> {
    let item_name = parameters
        .arguments
        .first()
        .ok_or_else(|| anyhow!("no item name specified"))?;
    let player_name = parameters.arguments.get(1);
    let item = parameters.items.items.get_item_type_by_name(item_name)?;
    let player;

    if let Some(player_name) = player_name {
        player = parameters
            .players
            .get_player_entity_from_name(player_name, &mut parameters.entities.entities)?;
    } else if let Some(executor) = parameters.executor.clone() {
        player = parameters
            .players
            .get_player_entity_from_name(executor.as_str(), &mut parameters.entities.entities)?;
    } else {
        return Err(anyhow!(
            "No player specified when the executor is a server console"
        ));
    };

    let mut inventory = parameters
        .entities
        .entities
        .ecs
        .get::<&mut Inventory>(*player)?
        .clone();

    inventory.give_item(
        ItemStack::new(item.get_id(), 1),
        (0.0, 0.0),
        &mut parameters.items.items,
        &mut parameters.entities.entities,
        parameters.event_manager,
    )?;

    *parameters
        .entities
        .entities
        .ecs
        .get::<&mut Inventory>(*player)? = inventory;

    Ok(("Gave item to player").into())
}
