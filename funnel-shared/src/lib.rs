pub mod guild_channel;

pub use guild_channel::*;

use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub enum ResultData {
    Guilds(Vec<GuildWithChannels>),
    AuthenticationSuccess,
    Error(ErrorType),
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
pub enum ErrorType {
    AuthenticationFailed(String),
    ClientNotAuthenticated,
    UnknowError(String),
}

#[derive(Serialize, Deserialize)]
pub struct WsResponse {
    status: Status,
    result: ResultData,
}

impl WsResponse {
    pub fn error_unknown(message: String) -> Self {
        let fail_status = Status::error();
        Self {
            status: fail_status,
            result: ResultData::Error(ErrorType::UnknowError(message)),
        }
    }

    pub fn guilds(guild_data: Vec<GuildWithChannels>) -> Self {
        let status = Status::Success { current_page: 0 };
        Self {
            status,
            result: ResultData::Guilds(guild_data),
        }
    }

    pub fn authentication_success() -> Self {
        let status = Status::Success { current_page: 0 };
        Self {
            status,
            result: ResultData::AuthenticationSuccess,
        }
    }

    pub fn authentication_failed(message: String) -> Self {
        let status = Status::error();
        Self {
            status,
            result: ResultData::Error(ErrorType::AuthenticationFailed(message)),
        }
    }

    pub fn not_authenticated() -> Self {
        let status = Status::error();
        Self {
            status,
            result: ResultData::Error(ErrorType::ClientNotAuthenticated),
        }
    }

    pub fn from_json(data: String) -> Result<Self, Error> {
        serde_json::from_str(&data)
    }

    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
