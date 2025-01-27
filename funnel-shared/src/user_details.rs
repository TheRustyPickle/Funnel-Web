use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserDetails {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

impl UserDetails {
    pub fn full_username(&self) -> String {
        if self.username != "0" {
            format!("{}#{}", self.username, self.discriminator)
        } else {
            self.username.clone()
        }
    }
}
