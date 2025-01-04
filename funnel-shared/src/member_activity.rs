use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberActivity {
    pub activity_timestamp: i64,
    pub guild_id: i64,
    pub join_activity: bool,
}
