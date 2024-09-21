use egui::Ui;

use crate::ui::Overview;
use crate::{EventBus, TabState};

#[derive(Default)]
pub struct TabHandler {
    overview: Overview,
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, event_bus: &mut EventBus) {
        match state {
            TabState::Overview => self.overview.show_ui(ui, event_bus),
            _ => {
                ui.vertical_centered(|ui| {
                    ui.heading("Under Construction");
                });
            }
        }
    }

    pub fn set_overview_compare(&mut self, status: bool) {
        self.overview.set_bottom_panel(status);
    }
}
