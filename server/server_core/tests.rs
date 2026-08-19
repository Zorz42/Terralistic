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

    // --- the input queue ---

    use crate::server::server_core::players::InputQueue;
    use crate::shared::players::{MovingType, PlayerInput};

    fn moving(moving_type: MovingType) -> PlayerInput {
        PlayerInput { moving_type, jumping: false }
    }

    /// An input arrives stamped for a tick the server has not reached - that is the whole
    /// point of the client's lead - and must wait there rather than taking effect early.
    #[test]
    fn test_an_input_for_a_future_tick_waits_for_it() {
        let mut queue = InputQueue::default();
        queue.accept(50, moving(MovingType::MovingRight), 10);

        assert_eq!(queue.advance_to(10), moving(MovingType::Standing), "applied early");
        assert_eq!(queue.advance_to(49), moving(MovingType::Standing), "applied a tick early");
        assert_eq!(queue.advance_to(50), moving(MovingType::MovingRight), "not applied on its own tick");
    }

    /// An input is a held state, not an edit: it stays in force with no further packets,
    /// which is what makes walking across the world cost two packets rather than hundreds.
    #[test]
    fn test_an_input_stays_in_force_until_replaced() {
        let mut queue = InputQueue::default();
        queue.accept(5, moving(MovingType::MovingLeft), 0);
        queue.advance_to(5);

        for tick in 6..100 {
            assert_eq!(queue.advance_to(tick), moving(MovingType::MovingLeft), "input lapsed at tick {tick}");
        }

        queue.accept(100, moving(MovingType::Standing), 99);
        assert_eq!(queue.advance_to(100), moving(MovingType::Standing));
    }

    /// Several inputs falling in one step - a client running well ahead, or a server that
    /// stalled - collapse to the newest. The older ones are superseded, not replayed.
    #[test]
    fn test_the_newest_input_at_or_before_the_tick_wins() {
        let mut queue = InputQueue::default();
        queue.accept(10, moving(MovingType::MovingLeft), 0);
        queue.accept(11, moving(MovingType::MovingRight), 0);
        queue.accept(12, moving(MovingType::Standing), 0);
        queue.accept(30, moving(MovingType::MovingLeft), 0);

        assert_eq!(queue.advance_to(20), moving(MovingType::Standing), "should hold the newest input up to tick 20");
        assert_eq!(queue.advance_to(30), moving(MovingType::MovingLeft));
    }

    /// An input whose tick has already been simulated cannot be un-simulated, so it is
    /// applied anyway and counted. A count that climbs is how `INPUT_LEAD_TICKS` being too
    /// small for a connection shows up, rather than as unexplained correction.
    #[test]
    fn test_a_late_input_is_counted_and_still_applied() {
        let mut queue = InputQueue::default();
        queue.accept(5, moving(MovingType::MovingRight), 20);

        assert_eq!(queue.late, 1, "an input for an already-simulated tick should be counted late");
        assert_eq!(queue.advance_to(21), moving(MovingType::MovingRight), "a late input must still take effect");
    }

    #[test]
    fn test_an_input_arriving_in_time_is_not_counted_late() {
        let mut queue = InputQueue::default();
        queue.accept(30, moving(MovingType::MovingRight), 10);
        assert_eq!(queue.late, 0);
    }
}
