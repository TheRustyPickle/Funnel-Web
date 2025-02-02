use serde::{Deserialize, Serialize};
use serde_json::Error;

pub const PAGE_VALUE: u64 = 5000;

use crate::{GuildWithChannels, MemberActivity, MemberCount, MessageWithUser, UserDetails};

#[derive(Serialize, Deserialize)]
pub enum Response {
    UserDetails(UserDetails),
    SessionID(String),
    Guilds(Vec<GuildWithChannels>),
    ConnectionSuccess(u64),
    Messages {
        guild_id: i64,
        messages: Vec<MessageWithUser>,
    },
    MemberCounts {
        guild_id: i64,
        counts: Vec<MemberCount>,
    },
    MemberActivities {
        guild_id: i64,
        activities: Vec<MemberActivity>,
    },
    LoggedOut,
    Error(ErrorType),
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Success { current_page: u64 },
    Error,
}

impl Status {
    #[must_use]
    pub fn success(current_page: u64) -> Self {
        Self::Success { current_page }
    }

    #[must_use]
    pub fn error() -> Self {
        Self::Error
    }

    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Status::Error)
    }

    #[must_use]
    pub fn page(&self) -> u64 {
        if let Status::Success { current_page } = self {
            return *current_page;
        }
        panic!("Should not be here");
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ErrorType {
    ClientNotConnected,
    FailedAuthentication,
    NoValidGuild,
    FailedSaveSession(String),
    FailedLogOut(String),
    InvalidSession,
    UnknowError(String),
}

#[derive(Serialize, Deserialize)]
pub struct WsResponse {
    pub status: Status,
    pub response: Response,
}

impl WsResponse {
    #[must_use]
    pub fn error_unknown(message: String) -> Self {
        let fail_status = Status::error();
        Self {
            status: fail_status,
            response: Response::Error(ErrorType::UnknowError(message)),
        }
    }

    #[must_use]
    pub fn guilds(guild_data: Vec<GuildWithChannels>) -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::Guilds(guild_data),
        }
    }

    #[must_use]
    pub fn connection_success(conn_id: u64) -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::ConnectionSuccess(conn_id),
        }
    }

    #[must_use]
    pub fn not_connected() -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::ClientNotConnected),
        }
    }

    #[must_use]
    pub fn failed_authentication() -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::FailedAuthentication),
        }
    }

    #[must_use]
    pub fn no_valid_guild() -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::NoValidGuild),
        }
    }

    #[must_use]
    pub fn messages(guild_id: i64, messages: Vec<MessageWithUser>, page: u64) -> Self {
        let status = Status::success(page);
        Self {
            status,
            response: Response::Messages { guild_id, messages },
        }
    }

    #[must_use]
    pub fn member_counts(guild_id: i64, counts: Vec<MemberCount>, page: u64) -> Self {
        let status = Status::success(page);
        Self {
            status,
            response: Response::MemberCounts { guild_id, counts },
        }
    }

    #[must_use]
    pub fn member_activities(guild_id: i64, activities: Vec<MemberActivity>, page: u64) -> Self {
        let status = Status::success(page);
        Self {
            status,
            response: Response::MemberActivities {
                guild_id,
                activities,
            },
        }
    }

    #[must_use]
    pub fn user_details(user_details: UserDetails) -> Self {
        let status = Status::success(1);
        Self {
            status,
            response: Response::UserDetails(user_details),
        }
    }

    #[must_use]
    pub fn session(id: String) -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::SessionID(id),
        }
    }

    #[must_use]
    pub fn failed_session_save(reason: String) -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::FailedSaveSession(reason)),
        }
    }

    #[must_use]
    pub fn failed_log_out(reason: String) -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::FailedLogOut(reason)),
        }
    }

    #[must_use]
    pub fn logged_out() -> Self {
        let status = Status::success(0);
        Self {
            status,
            response: Response::LoggedOut,
        }
    }

    #[must_use]
    pub fn invalid_session() -> Self {
        let status = Status::error();
        Self {
            status,
            response: Response::Error(ErrorType::InvalidSession),
        }
    }

    pub fn from_json(data: &str) -> Result<Self, Error> {
        serde_json::from_str(data)
    }

    #[must_use]
    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    #[must_use]
    pub fn get_error(&self) -> ErrorType {
        if let Response::Error(e_type) = &self.response {
            e_type.clone()
        } else {
            panic!()
        }
    }
}
