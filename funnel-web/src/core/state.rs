use chrono::NaiveDate;
use strum_macros::{Display, EnumIter};

// NOTE: Overview: Show a chart with total member movement, total left movement and total joined movement
// User Table: Already done. What other columns can be we add?
// Channel table: Show different message data based on a channel
// Message Chart: Show chart on total message, deleted message, alongside option to add individual
// user. Check statbot for more inspiration
// User chart: Show total active user chart daily hourly, weekly monthly
// Common words: Use the filtered word list to get count for 2 or more words combinations. Allow
// the user to change combination list as necessary

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

#[derive(Debug, PartialEq, Eq)]
pub enum AppEvent {
    DateChanged,
    CompareDate,
    StartWebsocket,
    TableUpdateDate(NaiveDate, i64),
    TableNeedsReload(i64),
    OverviewNeedsReload(i64),
    CellsCopied,
    GuildChanged,
    StopCompareOverview,
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
}

impl AppStatus {
    pub fn show_spinner(&self) -> bool {
        match self {
            AppStatus::ConnectingToWs | AppStatus::Fetching => true,
            AppStatus::Idle | AppStatus::FailedWs(_) | AppStatus::CellsCopied => false,
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

#[derive(Default, Eq, PartialEq, Display, EnumIter)]
pub enum ChartType {
    Hourly,
    #[default]
    Daily,
    Weekly,
    Monthly,
}
