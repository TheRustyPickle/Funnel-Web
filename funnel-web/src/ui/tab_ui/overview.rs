use egui::{Align, Grid, Layout, Ui};

use crate::get_new_x;

#[derive(Default)]
pub struct Overview {}

impl Overview {
    pub fn show_ui(&mut self, ui: &mut Ui) {
        ui.add_space(50.0);
        let (space_size, _) = get_new_x(ui, 15.0);
        Grid::new("Pass Grid")
            .num_columns(2)
            .spacing([5.0, 10.0])
            .show(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Total Message:");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Users With Message:");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Deleted Message");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Member Count:");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Member Joined");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Member Left");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Most Active Members");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Most Active Channels:");
                    ui.add_space(space_size);
                });

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("0");
                });
                ui.end_row();
            });
    }
}
