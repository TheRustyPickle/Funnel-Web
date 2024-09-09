use egui::Ui;
use std::collections::VecDeque;

use crate::{AppEvent, TabState};

#[derive(Default)]
pub struct TabHandler {
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, events: &mut VecDeque<AppEvent>) {
        match state {
            _ => {
                ui.heading("Under Construction");
            }
        }
    }
}
