use crate::libraries::events::EventManager;
use crate::server::server_core::{entities, players};
use crate::server::server_core::{items, send_to_ui};
use crate::server::server_ui::{ConsoleMessageType, ServerState, UiMessageType};
use crate::shared::inventory::Inventory;
use crate::shared::items::ItemStack;
use std::sync::mpsc::{Receiver, Sender};
use core::fmt::Write;

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
    pub function: fn(&mut CommandParameters) -> Result<String, Option<String>>,
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
    pub fn execute_command(
        &self,
        command: &str,
        state: &mut ServerState,
        executor: Option<String>,
        players: &mut players::ServerPlayers,
        items: &mut items::ServerItems,
        entities: &mut entities::ServerEntities,
        event_manager: &mut EventManager,
    ) -> Result<String, Option<String>> {
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
        Err(Some(String::from("Command not found")))
    }

    //executes all commands
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
                Err(val) => val.map_or_else(
                    || {
                        println!("Invalid use. see /help for more info");
                        ConsoleMessageType::Warning(
                            "Invalid use. see /help for more info".to_owned(),
                        )
                    },
                    |message| {
                        println!("{message}");
                        ConsoleMessageType::Warning(message)
                    },
                ),
            };
            send_to_ui(
                UiMessageType::SrvToUiConsoleMessage(feedback),
                ui_message_sender,
            );
        }
    }
}

//help command
#[allow(clippy::unnecessary_wraps)]//all command functions must return the same type
pub fn help(parameters: &mut CommandParameters) -> Result<String, Option<String>> {
    let mut string = String::new();
    string.push_str("Commands:\n");
    for c in &parameters.command_manager.commands {
        let res = writeln!(string, "{} - {}", c.call_name, c.description);
        if res.is_err() {
            return Err(Some(String::from("failed to write the help message")))
        }
    }
    Ok(string)
}

#[allow(clippy::unnecessary_wraps)]//all command functions must return the same type
pub fn stop(parameters: &mut CommandParameters) -> Result<String, Option<String>> {
    *parameters.state = ServerState::Stopping;
    Ok(String::from("Stopping server..."))
}

//this command gives an item to the player
pub fn give(parameters: &mut CommandParameters) -> Result<String, Option<String>> {
    //TODO: simplify errors
    let item_name = parameters.arguments.first();
    let player_name = parameters.arguments.get(1);
    let item;
    let player;

    if let Some(player_name) = player_name {
        if let Ok(player_) = parameters
            .players
            .get_player_entity_from_name(player_name, &mut parameters.entities.entities)
        {
            player = player_;
        } else {
            return Err(Some(String::from("Player not found")));
        }
    } else if let Some(executor) = parameters.executor.clone() {
        if let Ok(player_) = parameters
            .players
            .get_player_entity_from_name(executor.as_str(), &mut parameters.entities.entities)
        {
            player = player_;
        } else {
            return Err(Some(String::from("Player not found")));
        }
    } else {
        return Err(Some(String::from(
            "A player name must be specified if the command isn't executed by a player",
        )));
    };

    if let Some(item_name) = item_name {
        item = parameters.items.items.get_item_type_by_name(item_name);
    } else {
        return Err(None);
    }

    if let Ok(item) = item {
        let mut inventory;
        if let Ok(inventory_) = parameters
            .entities
            .entities
            .ecs
            .get::<&mut Inventory>(*player)
        {
            inventory = inventory_.clone();
        } else {
            return Err(None); //shouldn't ever happen
        }
        let res = inventory
            .give_item(
                ItemStack::new(item.get_id(), 1),
                (0.0, 0.0),
                &mut parameters.items.items,
                &mut parameters.entities.entities,
                parameters.event_manager,
            );

        if res.is_err() {
            return Err(Some(String::from("Failed to give item")))
        }

        if let Ok(mut inventory_) = parameters
            .entities
            .entities
            .ecs
            .get::<&mut Inventory>(*player)
        {
            *inventory_ = inventory;
        }
    } else {
        return Err(Some(String::from("Item not found")));
    }

    Ok(String::from("Item given"))
}
