use egui::{Align, Button, Grid, Key, Layout, TextEdit, Ui, Vec2};

use crate::core::{get_new_x, MainWindow};
use crate::{AppEvent, EventBus};

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

    fn add_info_text(&self, ui: &mut Ui, spacing: f32) {
        ui.add_space(20.0);
        Grid::new("Info Grid")
            .num_columns(2)
            .spacing([5.0, 10.0])
            .show(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("📝: The server side of this project is not live anywhere right now so it is not possible to pass this step right now");
                    ui.add_space(spacing);
                });
            });
    }

    pub fn show_pass_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
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
                        event_bus.publish(AppEvent::StartWsConnection);
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
            event_bus.publish(AppEvent::StartWsConnection)
        }
        self.add_info_text(ui, x_10);
    }
}

impl MainWindow {}
