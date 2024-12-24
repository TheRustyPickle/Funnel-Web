use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberCount {
    pub count_timestamp: i64,
    pub guild_id: i64,
    pub total_members: i64,
}
