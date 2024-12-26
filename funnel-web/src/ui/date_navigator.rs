use chrono::{Days, Months};
use eframe::egui::{Button, Key, Ui};
use egui_extras::DatePickerButton;
use strum::IntoEnumIterator;

use crate::core::NavigationType;
use crate::ui::{AnimatedMenuLabel, DateHandler};
use crate::{AppEvent, EventBus};

#[derive(Default)]
pub struct DateNavigator {
    nav_type: NavigationType,
    handler: DateHandler,
}

impl DateNavigator {
    pub fn show_ui(&mut self, ui: &mut Ui, connected: bool, event_bus: &mut EventBus) {
        ui.label("From:");
        ui.add_enabled(
            connected,
            DatePickerButton::new(self.handler.from()).id_salt("1"),
        );
        ui.label("To:");
        ui.add_enabled(
            connected,
            DatePickerButton::new(self.handler.to()).id_salt("2"),
        );

        if ui.add_enabled(connected, Button::new("Reset")).clicked() {
            event_bus.publish(AppEvent::DateChanged);
            self.handler.reset_dates();
        }

        ui.separator();

        let hover_position = ui.make_persistent_id("navigation_hover");
        let selected_position = ui.make_persistent_id("navigation_selected");
        for val in NavigationType::iter() {
            let val_string = val.to_string();
            let selected = self.nav_type == val;

            let resp = ui.add(AnimatedMenuLabel::new(
                selected,
                val_string,
                selected_position,
                hover_position,
                43.0,
                18.0,
                None,
                (false, false),
            ));

            if resp.clicked() {
                self.nav_type = val;
            }
        }

        ui.separator();

        let mut shift_pressed = false;
        let mut h_pressed = false;
        let mut l_pressed = false;

        if connected {
            shift_pressed = ui.ctx().input(|i| i.modifiers.shift);
            h_pressed = ui.ctx().input(|i| i.key_pressed(Key::H));
            l_pressed = ui.ctx().input(|i| i.key_pressed(Key::L));
        }

        let previous_hover = format!(
            "Go back by 1 {} from the current date. Shortcut key: SHIFT + H",
            self.nav_name()
        );
        let next_hover = format!(
            "Go next by 1 {} from the current date. Shortcut key: SHIFT + L",
            self.nav_name()
        );

        if ui
            .add_enabled(
                connected,
                Button::new(format!("Previous {}", self.nav_name())),
            )
            .on_hover_text(previous_hover)
            .clicked()
            || shift_pressed && h_pressed
        {
            self.go_previous();
        };

        if ui
            .add_enabled(connected, Button::new(format!("Next {}", self.nav_name())))
            .on_hover_text(next_hover)
            .clicked()
            || shift_pressed && l_pressed
        {
            self.go_next();
        };

        if self.handler.check_date_change() {
            event_bus.publish(AppEvent::DateChanged);
        }
    }

    pub fn show_ui_compare(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        ui.label("From:");
        ui.add(DatePickerButton::new(self.handler.from()).id_salt("3"));
        ui.label("To:");
        ui.add(DatePickerButton::new(self.handler.to()).id_salt("4"));

        if ui
            .add(Button::new("Compare"))
            .on_hover_text("Compare data from within this period with the current overview data")
            .clicked()
        {
            event_bus.publish(AppEvent::CompareDate);
        }

        if ui
            .add(Button::new("Reset Compare"))
            .on_hover_text("Stop comparing the overview data")
            .clicked()
        {
            event_bus.publish(AppEvent::StopCompareOverview);
        }

        ui.separator();

        let hover_position = ui.make_persistent_id("compare_navigation_hover");
        let selected_position = ui.make_persistent_id("compare_navigation_selected");
        for val in NavigationType::iter() {
            let val_string = val.to_string();
            let selected = self.nav_type == val;

            let resp = ui.add(AnimatedMenuLabel::new(
                selected,
                val_string,
                selected_position,
                hover_position,
                43.0,
                18.0,
                None,
                (false, false),
            ));

            if resp.clicked() {
                self.nav_type = val;
            }
        }

        ui.separator();

        let previous_hover = format!("Go back by 1 {} from the current date.", self.nav_name());
        let next_hover = format!("Go next by 1 {} from the current date.", self.nav_name());

        if ui
            .add(Button::new(format!("Previous {}", self.nav_name())))
            .on_hover_text(previous_hover)
            .clicked()
        {
            self.go_previous();
        };

        if ui
            .add(Button::new(format!("Next {}", self.nav_name())))
            .on_hover_text(next_hover)
            .clicked()
        {
            self.go_next();
        };
    }
    /// Handler and mutable
    pub fn handler(&mut self) -> &mut DateHandler {
        &mut self.handler
    }

    /// Handler and not mutable
    pub fn handler_i(&self) -> DateHandler {
        self.handler
    }

    // pub fn nav_type(&mut self) -> &mut NavigationType {
    //     &mut self.nav_type
    // }

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
