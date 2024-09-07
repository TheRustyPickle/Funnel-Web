use chrono::{Days, Months};
use egui::Ui;
use egui_extras::DatePickerButton;
use std::collections::VecDeque;
use strum::IntoEnumIterator;

use crate::core::NavigationType;
use crate::ui::DatePickerHandler;
use crate::AppEvent;

#[derive(Default)]
pub struct DateNavigator {
    nav_type: NavigationType,
    handler: DatePickerHandler,
}

impl DateNavigator {
    pub fn show_ui(&mut self, ui: &mut Ui, events: &mut VecDeque<AppEvent>) {
        ui.label("From:");
        ui.add(DatePickerButton::new(self.handler.from()).id_source("1"));
        ui.label("To:");
        ui.add(DatePickerButton::new(self.handler.to()).id_source("2"));

        if ui.button("Reset").clicked() {
            events.push_back(AppEvent::DateChanged);
            self.handler.reset_dates();
        }

        ui.separator();

        for val in NavigationType::iter() {
            let val_string = val.to_string();
            ui.selectable_value(self.nav_type(), val, val_string);
        }

        ui.separator();

        // TODO: add shortcut
        let previous_hover = format!(
            "Go back by 1 {} from the current date. Shortcut key: CTRL + H",
            self.nav_name()
        );
        let next_hover = format!(
            "Go next by 1 {} from the current date. Shortcut key: CTRL + L",
            self.nav_name()
        );

        if ui
            .button(format!("Previous {}", self.nav_name()))
            .on_hover_text(previous_hover)
            .clicked()
        {
            self.go_previous();
        };

        if ui
            .button(format!("Next {}", self.nav_name()))
            .on_hover_text(next_hover)
            .clicked()
        {
            self.go_next();
        };

        if self.handler.check_date_change() {
            events.push_back(AppEvent::DateChanged);
        }
    }
    /// Handler and mutable
    pub fn handler(&mut self) -> &mut DatePickerHandler {
        &mut self.handler
    }

    /// Handler and not mutable
    pub fn handler_i(&self) -> &DatePickerHandler {
        &self.handler
    }

    pub fn nav_type(&mut self) -> &mut NavigationType {
        &mut self.nav_type
    }

    pub fn nav_name(&self) -> String {
        self.nav_type.to_string()
    }

    pub fn go_next(&mut self) {
        match self.nav_type {
            NavigationType::Day => self.next_day(),
            NavigationType::Week => self.next_week(),
            NavigationType::Month => self.next_month(),
            NavigationType::Year => self.next_year(),
        }
    }

    pub fn go_previous(&mut self) {
        match self.nav_type {
            NavigationType::Day => self.previous_day(),
            NavigationType::Week => self.previous_week(),
            NavigationType::Month => self.previous_month(),
            NavigationType::Year => self.previous_year(),
        }
    }

    fn next_day(&mut self) {
        let from_date = self.handler().from;
        let mut to_date = self.handler().to;

        if from_date != to_date {
            *self.handler().from() = to_date;
            return;
        }

        to_date = to_date.checked_add_days(Days::new(1)).unwrap();

        *self.handler().from() = to_date;
        *self.handler().to() = to_date;
    }

    fn previous_day(&mut self) {
        let mut from_date = self.handler().from;
        let to_date = self.handler().to;

        if from_date != to_date {
            *self.handler().to() = from_date;
            return;
        }

        from_date = from_date.checked_sub_days(Days::new(1)).unwrap();

        *self.handler().from() = from_date;
        *self.handler().to() = from_date;
    }

    fn next_week(&mut self) {
        let from_date = self.handler().from;
        let mut to_date = self.handler().to;

        let target_date = to_date.checked_sub_days(Days::new(6)).unwrap();

        if from_date != target_date {
            *self.handler().from() = target_date;
            return;
        }

        to_date = to_date.checked_add_days(Days::new(6)).unwrap();

        *self.handler().from() = to_date.checked_sub_days(Days::new(6)).unwrap();
        *self.handler().to() = to_date;
    }

    fn previous_week(&mut self) {
        let mut from_date = self.handler().from;
        let to_date = self.handler().to;

        let target_date = from_date.checked_add_days(Days::new(6)).unwrap();

        if to_date != target_date {
            *self.handler().to() = target_date;
            return;
        }

        from_date = from_date.checked_sub_days(Days::new(6)).unwrap();

        *self.handler().from() = from_date;
        *self.handler().to() = from_date.checked_add_days(Days::new(6)).unwrap();
    }

    fn next_month(&mut self) {
        let from_date = self.handler().from;
        let mut to_date = self.handler().to;

        let target_date = to_date.checked_sub_months(Months::new(1)).unwrap();

        if from_date != target_date {
            *self.handler().from() = target_date;
            return;
        }

        to_date = to_date.checked_add_months(Months::new(1)).unwrap();

        *self.handler().from() = to_date.checked_sub_months(Months::new(1)).unwrap();
        *self.handler().to() = to_date;
    }

    fn previous_month(&mut self) {
        let mut from_date = self.handler().from;
        let to_date = self.handler().to;

        let target_date = from_date.checked_add_months(Months::new(1)).unwrap();

        if to_date != target_date {
            *self.handler().to() = target_date;
            return;
        }

        from_date = from_date.checked_sub_months(Months::new(1)).unwrap();

        *self.handler().from() = from_date;
        *self.handler().to() = from_date.checked_add_months(Months::new(1)).unwrap();
    }

    fn next_year(&mut self) {
        let from_date = self.handler().from;
        let mut to_date = self.handler().to;

        let target_date = to_date.checked_sub_months(Months::new(12)).unwrap();

        if from_date != target_date {
            *self.handler().from() = target_date;
            return;
        }

        to_date = to_date.checked_add_months(Months::new(12)).unwrap();

        *self.handler().from() = to_date.checked_sub_months(Months::new(12)).unwrap();
        *self.handler().to() = to_date;
    }

    fn previous_year(&mut self) {
        let mut from_date = self.handler().from;
        let to_date = self.handler().to;

        let target_date = from_date.checked_add_months(Months::new(12)).unwrap();

        if to_date != target_date {
            *self.handler().to() = target_date;
            return;
        }

        from_date = from_date.checked_sub_months(Months::new(12)).unwrap();

        *self.handler().from() = from_date;
        *self.handler().to() = from_date.checked_add_months(Months::new(12)).unwrap();
    }
}
