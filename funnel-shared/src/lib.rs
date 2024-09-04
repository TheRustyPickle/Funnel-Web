pub mod guild_channel;

use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::guild_channel::GuildWithChannels;

#[derive(Serialize, Deserialize)]
pub enum ResultData {
    Guilds(Vec<GuildWithChannels>),
    AuthenticationSuccess,
    AuthenticationFailed,
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Success { current_page: u64 },
    Error,
}

impl Status {
    pub fn success(current_page: u64) -> Self {
        Self::Success { current_page }
    }

    pub fn error() -> Self {
        Self::Error
    }
}

#[derive(Serialize, Deserialize)]
pub struct WsResponse {
    status: Status,
    result: ResultData,
}

impl WsResponse {
    pub fn error(message: String) -> Self {
        let fail_status = Status::error();
        Self {
            status: fail_status,
            result: ResultData::Error(message),
        }
    }

    pub fn guilds(status: Status, guild_data: Vec<GuildWithChannels>) -> Self {
        Self {
            status,
            result: ResultData::Guilds(guild_data),
        }
    }

    pub fn from_json(data: String) -> Result<Self, Error> {
        serde_json::from_str(&data)
    }

    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
