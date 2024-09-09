use serde::{Deserialize, Serialize};
use serde_json::error::Error;

#[derive(Serialize, Deserialize)]
pub enum Request {
    Authenticate(String),
    GetAllGuilds,
    GetMessages { guild_id: i64, page: u64 },
}

impl Request {
    pub fn auth(pass: String) -> Self {
        Request::Authenticate(pass)
    }

    pub fn guilds() -> Self {
        Request::GetAllGuilds
    }

    pub fn get_messages(guild_id: i64, page: u64) -> Self {
        Request::GetMessages { guild_id, page }
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json)
    }
}
