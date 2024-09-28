use eframe::egui::{Button, Key, TextEdit, Ui, Vec2};

use crate::core::MainWindow;
use crate::{AppEvent, EventBus};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct PasswordStatus {
    pub pass: String,
    show_pass: bool,
    pass_authenticated: bool,
    authenticating: bool,
    temp_pass: String,
    textbox_size: f32,
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

    fn add_info_text(&self, ui: &mut Ui) {
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            let remaining = ui.available_width() - self.textbox_size;
            ui.add_space(remaining / 2.0);
            ui.label("📝 Note: The server side of this project is not live anywhere so it is not possible to pass this step right now");
        });
    }

    pub fn show_pass_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Enter Password");
        });
        ui.add_space(10.0);

        let enter_pressed = ui.ctx().input(|i| i.key_pressed(Key::Enter));

        ui.horizontal(|ui| {
            let remaining = ui.available_width() - self.textbox_size;
            ui.add_space(remaining / 2.0);
            let max_width = ui.available_width();
            ui.label("Password:");
            ui.add_space(5.0);

            // Either 600 size or 15% below the max size available
            let textedit_size = if ui.available_width() < 600.0 {
                let x_15 = ui.available_width() * 15.0 / 100.0;
                Vec2::new(ui.available_width() - x_15, 20.0)
            } else {
                Vec2::new(600.0, 20.0)
            };

            let text_edit = TextEdit::singleline(&mut self.pass)
                .password(!self.show_pass)
                .hint_text("Password")
                .return_key(None);
            let text_edit_box = ui
                .add_sized(textedit_size, text_edit)
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
            let space_taken = max_width - ui.available_width();
            self.textbox_size = space_taken;
        });

        ui.add_space(10.0);
        if self.add_submit_button(ui) {
            self.authenticating = true;
            event_bus.publish(AppEvent::StartWsConnection)
        }
        self.add_info_text(ui);
    }
}

impl MainWindow {}
