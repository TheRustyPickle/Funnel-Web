use strum_macros::{Display, EnumIter};

#[derive(Default, Eq, PartialEq, Display, EnumIter, serde::Deserialize, serde::Serialize)]
pub enum TabState {
    #[default]
    Overview,
    #[strum(to_string = "User Table")]
    UserTable,
    Chart,
    #[strum(to_string = "Common Words")]
    CommonWords,
}
