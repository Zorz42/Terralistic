use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatPacket {
    pub message: String,
}