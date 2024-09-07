use strum_macros::{Display, EnumIter};

#[derive(Default, Eq, PartialEq, Display, EnumIter, Clone, Copy)]
pub enum TabState {
    #[default]
    Overview,
    #[strum(to_string = "User Table")]
    UserTable,
    #[strum(to_string = "Message Chart")]
    MessageChart,
    #[strum(to_string = "Member Chart")]
    MemberChart,
    #[strum(to_string = "Common Words")]
    CommonWords,
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
    StartWsConnection,
}

#[derive(Default, Display)]
pub enum AppStatus {
    #[default]
    Idle,
}
