#[derive(Clone, Default)]
pub struct FetchStatus {
    messages: bool,
    counts: bool,
    activities: bool,
    messages_page: u64,
    counts_page: u64,
    activities_page: u64,
}

impl FetchStatus {
    pub fn messages_done(&mut self) {
        self.messages = true;
    }

    pub fn counts_done(&mut self) {
        self.counts = true;
    }

    pub fn activities_done(&mut self) {
        self.activities = true;
    }

    #[must_use]
    pub fn all_done(&self) -> bool {
        self.messages && self.counts && self.activities
    }

    #[must_use]
    pub fn messages(&self) -> bool {
        self.messages
    }

    #[must_use]
    pub fn counts(&self) -> bool {
        self.counts
    }

    #[must_use]
    pub fn activities(&self) -> bool {
        self.activities
    }

    pub fn set_messages_page(&mut self, page: u64) {
        self.messages_page = page;
    }

    pub fn set_counts_page(&mut self, page: u64) {
        self.counts_page = page;
    }

    pub fn set_activities_page(&mut self, page: u64) {
        self.activities_page = page;
    }

    fn partial_messages(&self) -> bool {
        !self.messages() && self.messages_page != 0
    }

    fn partial_counts(&self) -> bool {
        !self.counts() && self.counts_page != 0
    }

    fn partial_activities(&self) -> bool {
        !self.activities() && self.activities_page != 0
    }

    #[must_use]
    pub fn no_partial(&self) -> bool {
        !self.partial_messages() && !self.partial_counts() && !self.partial_activities()
    }
}
