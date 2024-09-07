use std::collections::VecDeque;
use egui::Ui;

use crate::{ui::Overview, AppEvent, TabState};

#[derive(Default)]
pub struct TabHandler {
    overview: Overview,
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, events: &mut VecDeque<AppEvent>) {
        match state {
            TabState::Overview => self.overview.show_ui(ui),
            _ => {
                ui.heading("Under Construction");
            }
        }
    }
}
