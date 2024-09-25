use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::{GuildWithChannels, MessageWithUser};

#[derive(Serialize, Deserialize)]
pub enum Response {
    Guilds(Vec<GuildWithChannels>),
    AuthenticationSuccess,
    Messages(Vec<MessageWithUser>),
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

    pub fn is_error(&self) -> bool {
        matches!(self, Status::Error)
    }

    pub fn page(&self) -> u64 {
        if let Status::Success { current_page } = self {
            return *current_page;
        }
        panic!("Should not be here");
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ErrorType {
    AuthenticationFailed(String),
    ClientNotAuthenticated,
    UnknowError(String),
}

#[derive(Serialize, Deserialize)]
pub struct WsResponse {
    pub status: Status,
    pub response: Response,
}

impl WsResponse {
    pub fn error_unknown(message: String) -> Self {
        let fail_status = Status::error();
        Self {
            status: fail_status,
            response: Response::Error(ErrorType::UnknowError(message)),
        }
    }

    pub fn guilds(guild_data: Vec<GuildWithChannels>) -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::Guilds(guild_data),
        }
    }

    pub fn authentication_success() -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::AuthenticationSuccess,
        }
    }

    pub fn authentication_failed(message: String) -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::AuthenticationFailed(message)),
        }
    }

    pub fn not_authenticated() -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::ClientNotAuthenticated),
        }
    }

    pub fn messages(messages: Vec<MessageWithUser>, page: u64) -> Self {
        let status = Status::success(page);
        Self {
            status,
            response: Response::Messages(messages),
        }
    }

    pub fn from_json(data: String) -> Result<Self, Error> {
        serde_json::from_str(&data)
    }

    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn get_error(&self) -> ErrorType {
        match &self.response {
            Response::Error(e_type) => e_type.clone(),
            _ => panic!(),
        }
    }
}
