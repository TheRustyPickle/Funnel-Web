use egui::{Align, Button, Grid, Layout, TextEdit, Ui, Vec2};

use crate::initializer::MainWindow;
use crate::utils::get_new_x;

#[derive(Default)]
pub struct PasswordStatus {
    pass: String,
    show_pass: bool,
}

impl MainWindow {
    pub fn show_pass_ui(&mut self, ui: &mut Ui) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.heading("Enter Password");
        });
        ui.add_space(10.0);
        let (x_10, _) = get_new_x(ui);

        Grid::new("Pass Grid")
            .num_columns(2)
            .spacing([5.0, 10.0])
            .show(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Password:");
                    ui.add_space(x_10);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let (_, new_x) = get_new_x(ui);
                    let new_size = Vec2::new(new_x, 20.0);

                    let text_edit = TextEdit::singleline(&mut self.password.pass)
                        .password(!self.password.show_pass)
                        .hint_text("Password");
                    ui.add_sized(new_size, text_edit)
                        .on_hover_text("Enter the password to access the application");

                    if ui
                        .selectable_label(self.password.show_pass, "👁")
                        .on_hover_text("Show/Hide password")
                        .clicked()
                    {
                        self.password.show_pass = !self.password.show_pass
                    };
                });
            });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.add_sized(Vec2::new(80.0, 40.0), Button::new("Submit"));
        });
    }
}
