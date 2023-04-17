use std::ops::Deref;
use crate::server::server_ui::ServerState;

//struct that contains the command name and the function that will be executed when the command is called
pub struct Command {
    pub call_name: String,
    pub name: String,
    pub description: String,
    pub function: fn(&CommandManager, &mut ServerState),
}

//contains all the commands
pub struct CommandManager {
    pub commands: Vec<Command>
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new()
        }
    }

    //adds a command to the command manager
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    //executes a command
    pub fn execute_command(&self, command: &str, state: &mut ServerState) {
        for c in &self.commands {
            if c.call_name == command {
                (c.function)(&self, state);
            }
        }
    }
}

//help command
pub fn help(command_manager: &CommandManager, mut state: &mut ServerState) {
    println!("Commands:");
    for c in &command_manager.commands {
        println!("{} - {}", c.call_name, c.description);
    }
}

pub fn stop(command_manager: &CommandManager, mut state: &mut ServerState) {
    *state = ServerState::Stopping;
}
