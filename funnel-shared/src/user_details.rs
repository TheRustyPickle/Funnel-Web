use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserDetails {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

impl Default for UserDetails {
    fn default() -> Self {
        Self {
            id: String::from("0"),
            username: String::from("User"),
            discriminator: String::from("0"),
            avatar: None,
        }
    }
}

impl UserDetails {
    #[must_use]
    pub fn full_username(&self) -> String {
        if self.discriminator != "0" {
            format!("{}#{}", self.username, self.discriminator)
        } else {
            self.username.clone()
        }
    }

    #[must_use]
    pub fn avatar_link(&self) -> String {
        if let Some(hash) = self.avatar.as_ref() {
            format!("https://cdn.discordapp.com/avatars/{}/{hash}", self.id)
        } else {
            let modified_name = self.username.replace(' ', "%20");
            format!("https://api.dicebear.com/9.x/initials/png?seed={modified_name}")
        }
    }
}
