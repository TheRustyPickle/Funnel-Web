use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: i64,
    pub guild_name: String,
    pub guild_icon: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: i64,
    pub guild_id: i64,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GuildWithChannels {
    pub guild: Guild,
    pub channels: Vec<Channel>,
}

impl GuildWithChannels {
    pub fn new(guild: Guild, channels: Vec<Channel>) -> Self {
        Self { guild, channels }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResultData {
    Guilds(Vec<GuildWithChannels>),
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Success { has_more: bool, current_page: u64 },
    Error,
}

impl Status {
    pub fn success(has_more: bool, current_page: u64) -> Self {
        Self::Success {
            has_more,
            current_page,
        }
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

    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
