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
    UserChart,
    #[strum(to_string = "Common Words")]
    CommonWords,
}

impl TabState {
    #[must_use]
    pub fn last_value() -> Self {
        TabState::CommonWords
    }

    #[must_use]
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

#[derive(Debug, PartialEq, Eq)]
pub enum AppEvent {
    DateChanged,
    CompareDate,
    StartWebsocket,
    UpdateDate(NaiveDate, i64),
    OverviewNeedsReload(i64),
    UserTableNeedsReload(i64),
    ChannelTableNeedsReload(i64),
    WordTableNeedsReload(i64),
    MessageChartNeedsReload(i64),
    UserChartNeedsReload(i64),
    CellsCopied,
    GuildChanged,
    StopCompareOverview,
    MessageChartTypeChanged(i64),
    UserChartTypeChanged(i64),
    SelectedChannelsChanged,
    LogOut,
}

#[derive(Default, Display)]
pub enum AppStatus {
    #[default]
    Idle,
    #[strum(to_string = "Connecting to the server")]
    ConnectingToWs,
    #[strum(to_string = "Failed to connect to the websocket server. Reason: {0}")]
    FailedWs(String),
    #[strum(to_string = "Fetching data from the server")]
    Fetching,
    #[strum(to_string = "Copied selected cells to clipboard")]
    CellsCopied,
    #[strum(to_string = "Waiting to login to discord..")]
    LoggingIn,
    #[strum(
        to_string = "This account is not in any guild with data or with manage channel permission"
    )]
    NoValidGuild,
    #[strum(to_string = "Failed to authenticate Discord. Please try again")]
    FailedAuth,
    #[strum(to_string = "Unexpected error found. Reason: {0}")]
    UnexpectedError(String),
    #[strum(to_string = "Logged out of Discord")]
    LoggedOut,
    #[strum(to_string = "Attempting to log out of Discord")]
    AttemptLogOut,
    #[strum(to_string = "Faiiled to log out of Discord. Reason: {0}")]
    FailedLogOut(String),
}

impl AppStatus {
    #[must_use]
    pub fn show_spinner(&self) -> bool {
        match self {
            AppStatus::ConnectingToWs
            | AppStatus::Fetching
            | AppStatus::LoggingIn
            | AppStatus::AttemptLogOut => true,
            AppStatus::Idle
            | AppStatus::FailedWs(_)
            | AppStatus::CellsCopied
            | AppStatus::NoValidGuild
            | AppStatus::FailedAuth
            | AppStatus::UnexpectedError(_)
            | AppStatus::LoggedOut
            | AppStatus::FailedLogOut(_) => false,
        }
    }
}

#[derive(EnumIter, Display, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Copy)]
pub enum UserColumn {
    #[default]
    Name,
    Username,
    #[strum(to_string = "User ID")]
    UserID,
    #[strum(to_string = "Total Message")]
    TotalMessage,
    #[strum(to_string = "Deleted Message")]
    DeletedMessage,
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
    #[strum(to_string = "Unique Channels")]
    UniqueChannels,
}

#[derive(EnumIter, Display, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Copy)]
pub enum ChannelColumn {
    #[default]
    Name,
    #[strum(to_string = "Channel ID")]
    ID,
    #[strum(to_string = "Total Message")]
    TotalMessage,
    #[strum(to_string = "Deleted Message")]
    DeletedMessage,
    #[strum(to_string = "First Message Seen")]
    FirstMessage,
    #[strum(to_string = "Last Message Time")]
    LastMessage,
    #[strum(to_string = "Unique Users")]
    UniqueUsers,
}

#[derive(EnumIter, Display, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Copy)]
pub enum WordColumn {
    #[default]
    Phrase,
    Hits,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Display, EnumIter)]
pub enum ChartType {
    Hourly,
    #[default]
    Daily,
    Weekly,
    Monthly,
}
