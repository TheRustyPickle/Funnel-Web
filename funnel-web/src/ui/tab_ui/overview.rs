use egui::{TopBottomPanel, Ui};

use crate::core::to_header;
use crate::ui::{Card, DateNavigator, ShowUI};
use crate::EventBus;

#[derive(Default)]
pub struct Overview {
    total_message: u32,
    deleted_message: u32,
    unique_user: u32,
    member_count: u32,
    member_joins: u32,
    member_left: u32,
    most_active_member: String,
    most_active_channel: String,
    card_size: f32,
    show_bottom: bool,
    compare_nav: DateNavigator,
    compare_size: f32,
}

impl ShowUI for Overview {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        self.most_active_member = String::from("OneTwoThree: 100");
        self.most_active_channel = String::from("OneTwoThree: 100");

        self.show_bottom_bar(ui, event_bus);

        let space_3_item = ui.make_persistent_id("card_space_3");
        let space_2_item = ui.make_persistent_id("card_space_2");
        ui.vertical(|ui| {
            let x_size = 250.0;
            let y_size = 60.0;

            let mut space_3 = 0.0;
            let mut space_2 = 0.0;

            if self.card_size != 0.0 {
                let max_size = ui.available_width();
                let space_taken = 3.0 * self.card_size;
                let remaining = max_size - space_taken;
                let space_amount =
                    ui.ctx()
                        .animate_value_with_time(space_3_item, remaining / 2.0, 0.5);
                space_3 = space_amount;

                let space_taken = 2.0 * self.card_size;
                let remaining = max_size - space_taken;
                let space_amount =
                    ui.ctx()
                        .animate_value_with_time(space_2_item, remaining / 2.0, 0.5);
                space_2 = space_amount;
            } else {
                ui.ctx().animate_value_with_time(space_3_item, 0.0, 0.0);
                ui.ctx().animate_value_with_time(space_2_item, 0.0, 0.0);
            }

            ui.horizontal(|ui| {
                ui.add_space(space_3);
                let remaining_width = ui.available_width();
                ui.add(Card::new(
                    to_header("Total Messagse"),
                    to_header(self.total_message),
                    x_size,
                    y_size,
                ));
                let space_taken = remaining_width - ui.available_width();
                self.card_size = space_taken;

                ui.add(Card::new(
                    to_header("Deleted Message"),
                    to_header(self.deleted_message),
                    x_size,
                    y_size,
                ));

                ui.add(Card::new(
                    to_header("Unique User"),
                    to_header(self.unique_user),
                    x_size,
                    y_size,
                ));
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add_space(space_3);
                ui.add(Card::new(
                    to_header("Member Count"),
                    to_header(self.member_count),
                    x_size,
                    y_size,
                ));
                ui.add(Card::new(
                    to_header("Member Joins"),
                    to_header(self.member_joins),
                    x_size,
                    y_size,
                ));
                ui.add(Card::new(
                    to_header("Member Left"),
                    to_header(self.member_left),
                    x_size,
                    y_size,
                ));
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add_space(space_2);
                ui.add(Card::new(
                    to_header("Most Active Member"),
                    to_header(&self.most_active_member),
                    x_size,
                    y_size,
                ));

                ui.add(Card::new(
                    to_header("Most Active Channel"),
                    to_header(&self.most_active_channel),
                    x_size,
                    y_size,
                ));
            })
        });
        ui.add_space(50.0);
    }
}
// TODO: Allow comparing these numbers with a different date, show up or down %
impl Overview {
    fn show_bottom_bar(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        TopBottomPanel::bottom("bottom_panel_comparison")
            .show_separator_line(true)
            .show_animated_inside(ui, self.show_bottom, |ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let spacing_size = ui.available_width() - self.compare_size;
                    if spacing_size > 0.0 {
                        ui.add_space(spacing_size / 2.0);
                    };
                    let max_width = ui.available_width();
                    self.compare_nav.show_ui_compare(ui, event_bus);
                    let consumed = max_width - ui.available_width();
                    self.compare_size = consumed;
                });
            });
    }
}
