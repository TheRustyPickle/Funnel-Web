use eframe::egui::{Button, ScrollArea, TextEdit, TopBottomPanel, Ui, Vec2};

use crate::{get_change_log, AppEvent, EventBus};

#[derive(Default)]
pub struct Connection {
    connected: bool,
    connecting: bool,
    space_taken: f32,
}

impl Connection {
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn is_connecting(&self) -> bool {
        self.connecting
    }

    pub fn set_connected(&mut self) {
        self.connected = true;
    }

    pub fn failed_connection(&mut self) {
        self.connected = false;
        self.connecting = false;
    }

    fn add_start_button(&self, ui: &mut Ui) -> bool {
        let mut clicked = false;
        ui.vertical_centered(|ui| {
            let submit_button = Button::new("Start Connection").min_size(Vec2::new(150.0, 40.0));
            if ui
                .add_enabled(!self.connecting, submit_button)
                .on_hover_text("Start the connection to the server")
                .clicked()
            {
                clicked = true;
            }
        });
        clicked
    }

    fn add_info_text(&mut self, ui: &mut Ui) {
        ui.add_space(20.0);

        let mut text_edit_text = "https://discord.com/oauth2/authorize?client_id=1324028221066576017&permissions=66560&integration_type=0&scope=bot".to_string();

        ui.vertical_centered(|ui| {
            ui.label("Add this bot to your Discord server and run `/sync_all` to view analytics");
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            let spacing_size = ui.available_width() - self.space_taken;
            let spacing_size = ui.painter().round_to_pixel_center(spacing_size / 2.0);
            if spacing_size > 0.0 {
                ui.add_space(spacing_size);
            };

            let max_width = ui.available_width();

            // FIX: single line text edit text clipping not working. Wait for the next egui release
            // and see if it gets fixed
            let text_edit = TextEdit::multiline(&mut text_edit_text)
                .min_size(Vec2::new(300.0, 20.0))
                .clip_text(true)
                .desired_rows(1);

            ui.add(text_edit);

            let button = Button::new("Copy to clipboard").min_size(Vec2::new(30.0, 20.0));
            if ui
                .add(button)
                .on_hover_text("Copy the bot invite link to clipboard")
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = text_edit_text);
            }
            let consumed = max_width - ui.available_width();
            self.space_taken = consumed;
        });
    }

    pub fn show_start_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        ui.add_space(30.0);

        if self.add_start_button(ui) {
            event_bus.publish(AppEvent::StartWebsocket);
            self.connecting = true;
        }
        self.add_info_text(ui);

        TopBottomPanel::bottom("change_log")
            .show_separator_line(false)
            .min_height(300.0)
            .max_height(300.0)
            .show_inside(ui, |ui| {
                let change_logs = get_change_log();

                ui.vertical_centered(|ui| ui.heading("Change Logs"));
                ui.separator();
                ScrollArea::vertical().drag_to_scroll(false).show(ui, |ui| {
                    for log in change_logs {
                        log.to_ui(ui);
                    }
                })
            });
    }
}
