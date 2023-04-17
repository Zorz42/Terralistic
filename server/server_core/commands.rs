use crate::server::server_core::items;
use crate::server::server_core::players;
use crate::server::server_ui::ServerState;

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
                    arguments: arguments.get(1..).unwrap_or(&[]).to_vec(),
                });
            }
        }
        Err(Some(String::from("Command not found")))
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
    let _item_name = parameters.arguments.first();
    let player_name = parameters.arguments.get(1);
    //let item;
    //let player;

    return if let Some(_player_name) = player_name {
        Ok(String::from(
            "this isn't implemented yet because you cannot get the player by name",
        )) //get from player_name TODO
    } else if let Some(_executor) = parameters.executor {
        Ok(String::from(
            "this isn't implemented yet because you cannot get the player by name",
        )) //get from executor TODO
    } else {
        Err(Some(String::from(
            "A player name must be specified if the command isn't executed by a player",
        )))
    };

    /*if let Some(item_name) = item_name { //unreachable code, uncomment after the above thing is fixed
        item = parameters.items.items.get_item_type_by_name(item_name)
    } else {
        return Err(None);
    }

    if let Ok(item) = item {
        //give code
    } else {
        return Err(Some(String::from("Item not found")));
    }

    Ok(String::from("Item given"))*/
}
