use eframe::egui::{Button, ScrollArea, TopBottomPanel, Ui, Vec2};

use crate::{get_change_log, AppEvent, EventBus};

#[derive(Default)]
pub struct Connection {
    connected: bool,
    connecting: bool,
    text_size: f32,
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
            let submit_button = Button::new("Start Connection").min_size(Vec2::new(100.0, 40.0));
            if ui.add_enabled(!self.connecting, submit_button).clicked() {
                clicked = true;
            }
        });
        clicked
    }

    fn add_info_text(&mut self, ui: &mut Ui) {
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if self.text_size != 0.0 {
                let max_size = ui.available_width();
                let space_taken = self.text_size;
                let remaining = max_size - space_taken;
                let remaining = ui.painter().round_to_pixel_center(remaining);
                ui.add_space(remaining / 2.0);
            }
            let available_width = ui.available_width();
            ui.label("📝 Note: The server side of this project is not live anywhere so it is not possible to pass this step right now");
            let space_consumed = available_width - ui.available_width();
            self.text_size = space_consumed;
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
                ScrollArea::vertical()
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        for log in change_logs {
                            log.to_ui(ui);
                        }
                    })
            });
    }
}
