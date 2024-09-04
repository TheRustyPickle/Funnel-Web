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
