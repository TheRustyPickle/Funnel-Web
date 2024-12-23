use chrono::NaiveDate;

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct DateHandler {
    /// The From Date currently selected in the UI
    pub from: NaiveDate,
    /// The To Date currently selected in the UI
    pub to: NaiveDate,
    /// The last From date selected before the current From date
    last_from: Option<NaiveDate>,
    /// The last To date selected before the current To date
    last_to: Option<NaiveDate>,
    /// The oldest date with at least 1 data point
    start: Option<NaiveDate>,
    /// The newest date with at least 1 data point
    end: Option<NaiveDate>,
}

impl DateHandler {
    pub fn from(&mut self) -> &mut NaiveDate {
        &mut self.from
    }
    pub fn to(&mut self) -> &mut NaiveDate {
        &mut self.to
    }
    /// Verify whether the current From and To dates have changed
    pub fn check_date_change(&mut self) -> bool {
        if let Some(d) = self.last_from {
            if d != self.from {
                if self.from > self.to {
                    self.from = self.to;
                }

                self.last_from = Some(self.from);
                return true;
            }
        }
        if let Some(d) = self.last_to {
            if d != self.to {
                if self.to < self.from {
                    self.to = self.from;
                }

                self.last_to = Some(self.to);
                return true;
            }
        }
        false
    }

    /// Reset dates to the oldest and the newest value
    pub fn reset_dates(&mut self) {
        self.from = self.start.unwrap_or_default();
        self.to = self.end.unwrap_or_default();
        self.last_from = Some(self.from);
        self.last_to = Some(self.to);
    }

    /// Compare the given date with the current Start and End date
    /// to find the oldest and the newest date
    pub fn update_dates(&mut self, date: NaiveDate) -> bool {
        let mut needs_update = false;
        if self.start.map_or(true, |current| current > date) {
            self.from = date;
            self.start = Some(date);
            self.last_from = Some(date);
            needs_update = true;
        }

        if self.end.map_or(true, |current_date| current_date < date) {
            self.to = date;
            self.end = Some(date);
            self.last_to = Some(date);
            needs_update = true;
        }
        needs_update
    }

    /// Whether the given date is within the current From and To range
    pub fn within_range(&self, date: NaiveDate) -> bool {
        date >= self.from && date <= self.to
    }

    /// Whether the given date is before the current To range
    pub fn before_to_range(&self, date: NaiveDate) -> bool {
        date < self.to
    }
}
