use eframe::egui::ahash::HashMap;
use eframe::egui::Ui;

use crate::ui::{DateHandler, Overview, UserTable};
use crate::{EventBus, TabState};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ReloadTab {
    Overview,
    Table,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PendingReload {
    guild_id: i64,
    reload_type: ReloadTab,
}

#[derive(Default)]
pub struct TabHandler {
    pub current_guild: i64,
    pub overview: HashMap<i64, Overview>,
    pub user_table: HashMap<i64, UserTable>,
    pub pending_reloads: Vec<PendingReload>,
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, event_bus: &mut EventBus) {
        self.process_pending_reloads(&state);
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

    pub fn add_reload(&mut self, guild_id: i64, reload_type: ReloadTab) {
        let existing_reload = self
            .pending_reloads
            .iter()
            .any(|reload| reload.reload_type == reload_type);

        if !existing_reload {
            self.pending_reloads.push(PendingReload {
                guild_id,
                reload_type,
            });
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

    pub fn process_pending_reloads(&mut self, state: &TabState) {
        let mut to_remove_indices = Vec::new();

        for (index, pending_reload) in self.pending_reloads.clone().iter().enumerate() {
            if pending_reload.guild_id != self.current_guild {
                continue;
            }
            match pending_reload.reload_type {
                ReloadTab::Overview => {
                    if TabState::Overview == *state {
                        self.reload_overview(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
                ReloadTab::Table => {
                    if TabState::UserTable == *state {
                        self.recreate_rows(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
            }
        }

        // Remove items in reverse order to avoid index shifting
        for &index in to_remove_indices.iter().rev() {
            self.pending_reloads.remove(index);
        }
    }
}

pub trait ShowUI {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus);
}
