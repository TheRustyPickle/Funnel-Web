use serde::{Deserialize, Serialize};
use serde_json::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    StartConnection,
    StartConnectionNoLogin,
    GetGuildNoLogin,
    Session { id: String },
    LogOut,
    GetMessages { guild_id: i64, page: u64 },
    GetGuildMemberCount { guild_id: i64, page: u64 },
    GetGuildMemberActivity { guild_id: i64, page: u64 },
}

impl Request {
    pub fn start() -> Self {
        Request::StartConnection
    }

    pub fn start_no_login() -> Self {
        Request::StartConnectionNoLogin
    }

    pub fn guild_no_login() -> Self {
        Request::GetGuildNoLogin
    }

    pub fn session(id: String) -> Self {
        Request::Session { id }
    }

    pub fn get_messages(guild_id: i64, page: u64) -> Self {
        Request::GetMessages { guild_id, page }
    }

    pub fn get_member_counts(guild_id: i64, page: u64) -> Self {
        Request::GetGuildMemberCount { guild_id, page }
    }

    pub fn get_member_activity(guild_id: i64, page: u64) -> Self {
        Request::GetGuildMemberActivity { guild_id, page }
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json)
    }
}
