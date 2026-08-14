#![allow(clippy::unwrap_used, clippy::panic)] // a helper that gives up should fail the test loudly
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::libraries::scripting::ScriptHost;
    use crate::server::server_core::commands::CommandManager;
    use crate::server::server_core::networking::{BindAddress, ServerNetworking};
    use crate::shared::MOD_FUNCTION_PREFIX;

    /// Runs a command against a `CommandManager` that has no mods loaded.
    fn execute(command: &str) -> anyhow::Result<String> {
        let commands = CommandManager::new();
        let mut mods = ScriptHost::new(Vec::new(), MOD_FUNCTION_PREFIX);
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

    /// A server that cannot bind its port has to say so.
    ///
    /// The bind happens on the networking thread, so its error used to go nowhere anyone
    /// looked: `init` returns before the bind is even attempted, and the thread's `Result`
    /// is only joined once something else notices the thread has finished. A server whose
    /// port was taken therefore looked like it had started, printed no listening line, and
    /// accepted nobody for as long as it ran.
    #[test]
    fn test_a_server_that_cannot_bind_reports_it() {
        // hold the port for real, so the bind underneath has nothing to take
        let held = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = held.local_addr().unwrap().port();

        let mut net = ServerNetworking::new(port, BindAddress::Loopback);
        let mut events = EventManager::new();
        net.init();

        let error = wait_for_error(&mut net, &mut events);
        assert!(error.contains(&port.to_string()), "the error should name the port it could not bind: {error}");
        assert!(!net.is_listening(), "a server that never bound must not claim to be listening");
    }

    /// Steps networking until it surfaces the dead thread's error, so the test fails with
    /// that error rather than hanging if the reporting ever breaks again.
    fn wait_for_error(net: &mut ServerNetworking, events: &mut EventManager) -> String {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            if let Err(e) = net.update(events) {
                return e.to_string();
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("networking never reported that it could not bind");
    }
}
