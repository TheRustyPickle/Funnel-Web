use egui::{Align, Button, Context, Grid, Key, Layout, TextEdit, Ui, Vec2};
use log::info;
use std::collections::VecDeque;

use crate::core::{get_new_x, MainWindow};
use crate::AppEvent;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct PasswordStatus {
    pub pass: String,
    show_pass: bool,
    pass_authenticated: bool,
    authenticating: bool,
    temp_pass: String,
}

impl PasswordStatus {
    pub fn pass_authenticated(&self) -> bool {
        self.pass_authenticated
    }

    pub fn is_authenticating(&self) -> bool {
        self.authenticating
    }

    pub fn set_authenticated(&mut self) {
        self.pass_authenticated = true;
    }

    pub fn failed_connection(&mut self) {
        self.pass_authenticated = false;
        self.authenticating = false;
        self.temp_pass.clear();
    }

    pub fn clear_pass(&mut self) {
        self.temp_pass.clear();
        self.pass.clear();
    }

    pub fn set_temp_pass(&mut self, pass: String) {
        self.temp_pass = pass
    }

    pub fn temp_pass(&self) -> String {
        self.temp_pass.clone()
    }

    fn add_submit_button(&self, ui: &mut Ui) -> bool {
        let mut clicked = false;
        ui.vertical_centered(|ui| {
            let submit_button = Button::new("Submit").min_size(Vec2::new(80.0, 40.0));
            if ui
                .add_enabled(!self.authenticating, submit_button)
                .clicked()
            {
                clicked = true;
            }
        });
        clicked
    }

    pub fn show_pass_ui(&mut self, ui: &mut Ui, events: &mut VecDeque<AppEvent>) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Enter Password");
        });
        ui.add_space(10.0);
        let (x_10, _) = get_new_x(ui, 10.0);

        let enter_pressed = ui.ctx().input(|i| i.key_pressed(Key::Enter));

        Grid::new("Pass Grid")
            .num_columns(2)
            .spacing([5.0, 10.0])
            .show(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Password:");
                    ui.add_space(x_10);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let (_, new_x) = get_new_x(ui, 10.0);
                    let new_size = Vec2::new(new_x, 20.0);

                    let text_edit = TextEdit::singleline(&mut self.pass)
                        .password(!self.show_pass)
                        .hint_text("Password")
                        .return_key(None);

                    let text_edit_box = ui
                        .add_sized(new_size, text_edit)
                        .on_hover_text("Enter the password to access the application");

                    if text_edit_box.has_focus() && enter_pressed && !self.authenticating {
                        self.authenticating = true;
                        events.push_back(AppEvent::StartWsConnection);
                    }

                    if ui
                        .selectable_label(self.show_pass, "👁")
                        .on_hover_text("Show/Hide password")
                        .clicked()
                    {
                        self.show_pass = !self.show_pass
                    };
                });
            });

        ui.add_space(10.0);
        if self.add_submit_button(ui) {
            self.authenticating = true;
            events.push_back(AppEvent::StartWsConnection)
        }
    }
}

impl MainWindow {}
