use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub message_timestamp: i64,
    pub sender_id: i64,
    pub message_content: Option<String>,
    pub stripped_content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub global_name: Option<String>,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageWithUser {
    pub message: Message,
    pub sender: User,
}

impl MessageWithUser {
    pub fn new(message: Message, sender: User) -> Self {
        Self { message, sender }
    }
}
