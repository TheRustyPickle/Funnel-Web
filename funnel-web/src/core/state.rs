use strum_macros::{Display, EnumCount, EnumIter};

#[derive(Default, Eq, PartialEq, Display, EnumIter, Clone, Copy, EnumCount)]
pub enum TabState {
    #[default]
    Overview,
    #[strum(to_string = "User Table")]
    UserTable,
    #[strum(to_string = "Channel Table")]
    ChannelTable,
    #[strum(to_string = "Message Chart")]
    MessageChart,
    #[strum(to_string = "User Chart")]
    MemberChart,
    #[strum(to_string = "Common Words")]
    CommonWords,
}

impl TabState {
    pub fn last_value() -> Self {
        TabState::CommonWords
    }

    pub fn first_value() -> Self {
        TabState::default()
    }
}

#[derive(Default, Eq, PartialEq, Display, EnumIter)]
pub enum NavigationType {
    #[default]
    Day,
    Week,
    Month,
    Year,
}

pub enum AppEvent {
    DateChanged,
    CompareDate,
    CompareVisibility,
    StartWsConnection,
}

#[derive(Default, Display)]
pub enum AppStatus {
    #[default]
    Idle,
    #[strum(to_string = "Starting authentication process")]
    CheckingAuth,
    #[strum(to_string = "Failed to authenticate. Reason: {0}")]
    FailedAuth(String),
    #[strum(to_string = "Failed to connect to the websocket server. Reason: {0}")]
    FailedWs(String),
    #[strum(to_string = "Fetching data from the server")]
    Fetching,
}

impl AppStatus {
    pub fn show_spinner(&self) -> bool {
        match self {
            AppStatus::CheckingAuth | AppStatus::Fetching => true,
            AppStatus::Idle | AppStatus::FailedAuth(_) | AppStatus::FailedWs(_) => false,
        }
    }
}
