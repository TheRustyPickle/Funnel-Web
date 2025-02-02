use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub guild_id: i64,
    pub guild_name: String,
    pub guild_icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub channel_id: i64,
    pub guild_id: i64,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildWithChannels {
    pub guild: Guild,
    pub channels: Vec<Channel>,
}

impl GuildWithChannels {
    #[must_use]
    pub fn new(guild: Guild, channels: Vec<Channel>) -> Self {
        Self { guild, channels }
    }
}
