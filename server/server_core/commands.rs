use crate::libraries::events::EventManager;
use crate::server::server_core::{entities, players};
use crate::server::server_core::{items, send_to_ui};
use crate::server::server_ui::{ConsoleMessageType, ServerState, UiMessageType};
use crate::shared::items::ItemStack;
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
    pub function: fn(CommandParameters) -> Result<String, Option<String>>,
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
                return (c.function)(CommandParameters {
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
pub fn help(parameters: CommandParameters) -> Result<String, Option<String>> {
    let mut string = String::new();
    string.push_str("Commands:\n");
    for c in &parameters.command_manager.commands {
        string.push_str(&format!("{} - {}\n", c.call_name, c.description));
    }
    Ok(string)
}

pub fn stop(parameters: CommandParameters) -> Result<String, Option<String>> {
    *parameters.state = ServerState::Stopping;
    Ok(String::from("Stopping server..."))
}

//this command gives an item to the player
pub fn give(parameters: CommandParameters) -> Result<String, Option<String>> {
    let item_name = parameters.arguments.first();
    let player_name = parameters.arguments.get(1);
    let item;
    let player;

    if let Some(player_name) = player_name {
        if let Ok(player_) = parameters.players.get_player_from_name(player_name) {
            player = player_;
        } else {
            return Err(Some(String::from("Player not found")));
        }
    } else if let Some(executor) = parameters.executor {
        if let Ok(player_) = parameters.players.get_player_from_name(executor.as_str()) {
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
        item = parameters.items.items.get_item_type_by_name(item_name)
    } else {
        return Err(None);
    }

    if let Ok(item) = item {
        player
            .inventory
            .give_item(
                ItemStack::new(item.get_id(), 1),
                (0.0, 0.0),
                &mut parameters.items.items,
                &mut parameters.entities.entities,
                parameters.event_manager,
            )
            .expect("TODO: panic message");
    } else {
        return Err(Some(String::from("Item not found")));
    }

    Ok(String::from("Item given"))
}
