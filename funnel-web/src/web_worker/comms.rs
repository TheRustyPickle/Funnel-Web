use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WorkerMessage {
    StartConnection(String),
    WsPassword(String),
    ConnectionEstablished,
}
