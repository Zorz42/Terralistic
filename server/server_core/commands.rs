use crate::server::server_ui::ServerState;

/**
 * This struct contains all parameters that are needed to execute any command
 * It is used so that when a new argument is needed to be added to a command, it can be added here and all commands will be updated
 */
pub struct CommandParameters<'a> {
    pub command_manager: &'a CommandManager,
    pub state: &'a mut ServerState,
}

//struct that contains the command name and the function that will be executed when the command is called
pub struct Command {
    pub call_name: String,
    pub name: String,
    pub description: String,
    pub function: fn(CommandParameters) -> String,
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
                let feedback = (c.function)(CommandParameters {
                    command_manager: self,
                    state
                });
                return feedback;
            }
        }
        String::from("Command not found")
    }
}

//help command
pub fn help(parameters: CommandParameters) -> String {
    let mut string = String::new();
    string.push_str("Commands:\n");
    for c in &parameters.command_manager.commands {
        string.push_str(&format!("{} - {}\n", c.call_name, c.description));
    }
    string
}

pub fn stop(parameters: CommandParameters) -> String {
    *parameters.state = ServerState::Stopping;
    String::from("Stopping server...")
}
