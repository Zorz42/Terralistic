use crate::server::server_ui::ServerState;

//struct that contains the command name and the function that will be executed when the command is called
pub struct Command {
    pub call_name: String,
    pub name: String,
    pub description: String,
    pub function: fn(&CommandManager, &mut ServerState) -> String,
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
    pub fn execute_command(&self, command: &str, state: &mut ServerState) -> String {
        for c in &self.commands {
            if c.call_name == command {
                let feedback = (c.function)(&self, state);
                return feedback;
            }
        }
        String::from("Command not found")
    }
}

//help command
pub fn help(command_manager: &CommandManager, _state: &mut ServerState) -> String {
    let mut string = String::new();
    string.push_str("Commands:\n");
    for c in &command_manager.commands {
        string.push_str(&format!("{} - {}\n", c.call_name, c.description));
    }
    string
}

pub fn stop(_command_manager: &CommandManager, state: &mut ServerState) -> String {
    *state = ServerState::Stopping;
    String::from("Stopping server...")
}
