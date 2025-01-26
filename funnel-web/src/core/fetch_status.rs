#[derive(Clone, Default)]
pub struct FetchStatus {
    messages: bool,
    counts: bool,
    activities: bool,
}

impl FetchStatus {
    pub fn messages_done(&mut self) {
        self.messages = true;
    }

    pub fn counts_done(&mut self) {
        self.counts = true;
    }

    pub fn activities_done(&mut self) {
        self.activities = true
    }

    pub fn all_done(&self) -> bool {
        self.messages && self.counts && self.activities
    }

    pub fn messages(&self) -> bool {
        self.messages
    }

    pub fn counts(&self) -> bool {
        self.counts
    }

    pub fn activities(&self) -> bool {
        self.activities
    }
}
