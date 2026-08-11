#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::server::server_core::commands::CommandManager;
    use crate::shared::mod_manager::ModManager;

    /// Runs a command against a `CommandManager` that has no mods loaded.
    fn execute(command: &str) -> anyhow::Result<String> {
        let commands = CommandManager::new();
        let mut mods = ModManager::new(Vec::new());
        commands.execute_command(command, &mut mods, None)
    }

    /// A chat message of just "/" is stripped to an empty string before it reaches
    /// `execute_command`. That used to panic on `Vec::remove(0)`, which took the whole
    /// server down and was reachable by any connected player from the stock client.
    #[test]
    fn test_empty_command_does_not_panic() {
        execute("").unwrap_err();
    }

    /// Same path, but the message was "/ " or similar: `split_whitespace` still yields
    /// no tokens.
    #[test]
    fn test_whitespace_only_command_does_not_panic() {
        execute("   ").unwrap_err();
        execute("\t").unwrap_err();
    }

    /// An unknown command is reported back to the caller rather than being an error.
    #[test]
    fn test_unknown_command_is_reported() {
        let result = execute("definitely_not_a_command").unwrap();
        assert!(result.contains("definitely_not_a_command"), "unexpected output: {result}");
    }

    /// The builtin help command works without any mods loaded.
    #[test]
    fn test_help_command() {
        let result = execute("help").unwrap();
        assert!(result.contains("/help"), "unexpected output: {result}");
    }

    /// Help rejects more than one argument instead of panicking or silently ignoring.
    #[test]
    fn test_help_with_too_many_arguments() {
        execute("help one two").unwrap_err();
    }
}
