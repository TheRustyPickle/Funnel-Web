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
    StartWsConnection,
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
    UserID,
    TotalMessage,
    TotalWord,
    TotalChar,
    AverageWord,
    AverageChar,
    FirstMessageSeen,
    LastMessageSeen,
}

impl ColumnName {
    pub fn get_next(&self) -> Self {
        match self {
            ColumnName::Name => ColumnName::Username,
            ColumnName::Username => ColumnName::UserID,
            ColumnName::UserID => ColumnName::TotalMessage,
            ColumnName::TotalMessage => ColumnName::TotalWord,
            ColumnName::TotalWord => ColumnName::TotalChar,
            ColumnName::TotalChar => ColumnName::AverageWord,
            ColumnName::AverageWord => ColumnName::AverageChar,
            ColumnName::AverageChar => ColumnName::FirstMessageSeen,
            ColumnName::FirstMessageSeen => ColumnName::LastMessageSeen,
            ColumnName::LastMessageSeen => ColumnName::Name,
        }
    }

    pub fn get_previous(&self) -> Self {
        match self {
            ColumnName::Name => ColumnName::LastMessageSeen,
            ColumnName::Username => ColumnName::Name,
            ColumnName::UserID => ColumnName::Username,
            ColumnName::TotalMessage => ColumnName::UserID,
            ColumnName::TotalWord => ColumnName::TotalMessage,
            ColumnName::TotalChar => ColumnName::TotalWord,
            ColumnName::AverageWord => ColumnName::TotalChar,
            ColumnName::AverageChar => ColumnName::AverageWord,
            ColumnName::FirstMessageSeen => ColumnName::AverageChar,
            ColumnName::LastMessageSeen => ColumnName::FirstMessageSeen,
        }
    }

    pub fn from_num(num: i32) -> Self {
        match num {
            0 => ColumnName::Name,
            1 => ColumnName::Username,
            2 => ColumnName::UserID,
            3 => ColumnName::TotalMessage,
            4 => ColumnName::TotalWord,
            5 => ColumnName::TotalChar,
            6 => ColumnName::AverageWord,
            7 => ColumnName::AverageChar,
            8 => ColumnName::FirstMessageSeen,
            9 => ColumnName::LastMessageSeen,
            _ => unreachable!("Invalid enum variant for number {}", num),
        }
    }

    pub fn get_last() -> Self {
        ColumnName::LastMessageSeen
    }
}

#[derive(Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}
