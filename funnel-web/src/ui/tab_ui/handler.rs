use eframe::egui::Ui;
use std::collections::HashMap;

use crate::ui::{DateHandler, Overview, UserTable};
use crate::{EventBus, TabState};

#[derive(Default)]
pub struct TabHandler {
    pub current_guild: i64,
    pub overview: HashMap<i64, Overview>,
    pub user_table: HashMap<i64, UserTable>,
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, event_bus: &mut EventBus) {
        let mut show_ui = |data: Option<&mut dyn ShowUI>| {
            if let Some(item) = data {
                item.show_ui(ui, event_bus);
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading("Under Construction");
                });
            }
        };

        match state {
            TabState::Overview => show_ui(
                self.overview
                    .get_mut(&self.current_guild)
                    .map(|o| o as &mut dyn ShowUI),
            ),
            TabState::UserTable => show_ui(
                self.user_table
                    .get_mut(&self.current_guild)
                    .map(|u| u as &mut dyn ShowUI),
            ),
            _ => {
                ui.vertical_centered(|ui| {
                    ui.heading("Under Construction");
                });
            }
        }
    }

    pub fn set_data(&mut self, id: i64) {
        self.overview.insert(id, Overview::default());
        self.user_table.insert(id, UserTable::default());
    }

    pub fn set_current_guild(&mut self, id: i64) {
        self.current_guild = id;
    }

    pub fn set_date_handler(&mut self, guild_id: i64, handler: DateHandler) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
        self.user_table
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
    }
}

pub trait ShowUI {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus);
}
