#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::serialization;
    use crate::server::server_ui::{ConsoleMessageType, PlayerEventType, ServerState, UiMessageType};

    #[test]
    fn test_server_state_round_trip() {
        for state in [
            ServerState::Nothing,
            ServerState::Starting,
            ServerState::InitMods,
            ServerState::LoadingWorld,
            ServerState::GeneratingWorld,
            ServerState::Running,
            ServerState::Stopping,
            ServerState::Stopped,
        ] {
            let bytes = serialization::serialize(&state).unwrap();
            assert!(serialization::deserialize::<ServerState>(&bytes).unwrap() == state);
        }
    }

    /// The states are distinct, which matters because the ui compares against them to
    /// decide when to close.
    #[test]
    fn test_server_states_are_distinct() {
        assert!(ServerState::Running != ServerState::Stopping);
        assert!(ServerState::Stopping != ServerState::Stopped);
        assert!(ServerState::Nothing != ServerState::Starting);
    }

    #[test]
    fn test_console_message_round_trip() {
        for message in [
            ConsoleMessageType::Info("info".to_owned()),
            ConsoleMessageType::Warning("warning".to_owned()),
            ConsoleMessageType::Error("error".to_owned()),
        ] {
            let bytes = serialization::serialize(&message).unwrap();
            assert!(serialization::deserialize::<ConsoleMessageType>(&bytes).unwrap() == message);
        }
    }

    /// The same text at different severities is not the same message.
    #[test]
    fn test_console_severity_is_part_of_the_value() {
        assert!(ConsoleMessageType::Info("x".to_owned()) != ConsoleMessageType::Warning("x".to_owned()));
    }

    #[test]
    fn test_player_event_round_trip() {
        let address = "127.0.0.1:49153".parse().unwrap();

        for event in [PlayerEventType::Join(("jakob".to_owned(), address)), PlayerEventType::Leave(address)] {
            let bytes = serialization::serialize(&event).unwrap();
            assert!(serialization::deserialize::<PlayerEventType>(&bytes).unwrap() == event);
        }
    }

    #[test]
    fn test_ui_message_round_trip() {
        let messages = [
            UiMessageType::ServerState(ServerState::Running),
            UiMessageType::SrvToUiConsoleMessage(ConsoleMessageType::Info("hello".to_owned())),
            UiMessageType::UiToSrvConsoleMessage("/help".to_owned()),
            UiMessageType::MsptUpdate((Some(1.5), 2.5)),
        ];

        for message in messages {
            let bytes = serialization::serialize(&message).unwrap();
            assert!(serialization::deserialize::<UiMessageType>(&bytes).unwrap() == message);
        }
    }

    /// The mspt update carries an optional server time, since a tick may not have run.
    #[test]
    fn test_mspt_update_handles_a_missing_server_time() {
        let message = UiMessageType::MsptUpdate((None, 3.0));
        let bytes = serialization::serialize(&message).unwrap();
        assert!(serialization::deserialize::<UiMessageType>(&bytes).unwrap() == message);
    }
}
