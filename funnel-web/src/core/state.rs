use chrono::NaiveDate;
use strum_macros::{Display, EnumIter};

#[derive(Default, Eq, PartialEq, Display, EnumIter, Clone, Copy)]
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
    PasswordSubmitted,
    PasswordFailed(String),
    StartWebsocket(String),
    TableUpdateDate(NaiveDate, i64),
    CellsCopied,
    GuildChanged,
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
    #[strum(to_string = "Copied selected cells to clipboard")]
    CellsCopied,
}

impl AppStatus {
    pub fn show_spinner(&self) -> bool {
        match self {
            AppStatus::CheckingAuth | AppStatus::Fetching => true,
            AppStatus::Idle
            | AppStatus::FailedAuth(_)
            | AppStatus::FailedWs(_)
            | AppStatus::CellsCopied => false,
        }
    }
}

#[derive(EnumIter, Display, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Copy)]
pub enum ColumnName {
    #[default]
    Name,
    Username,
    #[strum(to_string = "User ID")]
    UserID,
    #[strum(to_string = "Total Message")]
    TotalMessage,
    #[strum(to_string = "Total Word")]
    TotalWord,
    #[strum(to_string = "Total Char")]
    TotalChar,
    #[strum(to_string = "Average Word")]
    AverageWord,
    #[strum(to_string = "Average Char")]
    AverageChar,
    #[strum(to_string = "First Message Seen")]
    FirstMessageSeen,
    #[strum(to_string = "Last Message Seen")]
    LastMessageSeen,
}

#[derive(Default)]
pub enum RequestStatus {
    #[default]
    None,
    Pending,
    Gotten(String),
    Failed(String),
}
