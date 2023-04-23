#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq)]
pub enum UiMessageType {
    ServerState(ServerState),
    SrvToUiConsoleMessage(ConsoleMessageType),
    UiToSrvConsoleMessage(String),
    MsptUpdate((u64, u64)),
    PlayerEvent(PlayerEventType),
}

#[derive(Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq)]
pub enum ServerState {
    Nothing,
    Starting,
    InitMods,
    LoadingWorld,
    GeneratingWorld,
    Running,
    Stopping,
    Stopped,
}

#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq)]
pub enum PlayerEventType {
    Join((String, std::net::SocketAddr)),
    Leave(std::net::SocketAddr),
    //kick and ban will be added later
}

#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq)]
pub enum ConsoleMessageType {
    Info(String),
    Warning(String),
    Error(String),
}
