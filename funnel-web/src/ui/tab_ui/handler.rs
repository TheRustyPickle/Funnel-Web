use eframe::egui::ahash::{HashMap, HashSet};
use eframe::egui::Ui;
use funnel_shared::Channel;

use crate::ui::{
    ChannelTable, DateHandler, MessageChart, Overview, UserChart, UserTable, WordTable,
};
use crate::{EventBus, TabState};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ReloadTab {
    Overview(i64),
    UserTable(i64),
    ChannelTable(i64),
    WordTable(i64),
    MessageChart(i64),
    UserChart(i64),
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
    pub channel_table: HashMap<i64, ChannelTable>,
    pub message_chart: HashMap<i64, MessageChart>,
    pub user_chart: HashMap<i64, UserChart>,
    pub word_table: HashMap<i64, WordTable>,
    pub pending_reloads: Vec<PendingReload>,
}

impl TabHandler {
    pub fn show_tab_ui(&mut self, ui: &mut Ui, state: TabState, event_bus: &mut EventBus) {
        self.process_pending_reloads(&state);
        let mut show_ui = |data: Option<&mut dyn ShowUI>| {
            if let Some(item) = data {
                item.show_ui(ui, self.current_guild, event_bus);
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
            TabState::ChannelTable => show_ui(
                self.channel_table
                    .get_mut(&self.current_guild)
                    .map(|u| u as &mut dyn ShowUI),
            ),
            TabState::MessageChart => show_ui(
                self.message_chart
                    .get_mut(&self.current_guild)
                    .map(|u| u as &mut dyn ShowUI),
            ),
            TabState::UserChart => show_ui(
                self.user_chart
                    .get_mut(&self.current_guild)
                    .map(|u| u as &mut dyn ShowUI),
            ),
            TabState::CommonWords => show_ui(
                self.word_table
                    .get_mut(&self.current_guild)
                    .map(|u| u as &mut dyn ShowUI),
            ),
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
        self.overview.entry(id).or_default();
        self.user_table.entry(id).or_default();
        self.channel_table.entry(id).or_default();
        self.message_chart.entry(id).or_default();
        self.user_chart.entry(id).or_default();
        self.word_table.entry(id).or_default();
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
        self.channel_table
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
        self.message_chart
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
        self.user_chart
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
        self.word_table
            .get_mut(&guild_id)
            .unwrap()
            .set_date_handler(handler);
    }

    pub fn set_channels(&mut self, channels: Vec<Channel>) {
        self.overview
            .get_mut(&self.current_guild)
            .unwrap()
            .set_channels(channels.clone());
        self.user_table
            .get_mut(&self.current_guild)
            .unwrap()
            .set_channels(channels.clone());
        self.message_chart
            .get_mut(&self.current_guild)
            .unwrap()
            .set_channels(channels.clone());
        self.user_chart
            .get_mut(&self.current_guild)
            .unwrap()
            .set_channels(channels.clone());
        self.word_table
            .get_mut(&self.current_guild)
            .unwrap()
            .set_channels(channels);
    }

    pub fn set_selected_channels(&mut self, selected: HashSet<usize>) {
        self.overview
            .get_mut(&self.current_guild)
            .unwrap()
            .set_selected_channels(selected.clone());
        self.user_table
            .get_mut(&self.current_guild)
            .unwrap()
            .set_selected_channels(selected.clone());
        self.message_chart
            .get_mut(&self.current_guild)
            .unwrap()
            .set_selected_channels(selected.clone());
        self.user_chart
            .get_mut(&self.current_guild)
            .unwrap()
            .set_selected_channels(selected.clone());
        self.word_table
            .get_mut(&self.current_guild)
            .unwrap()
            .set_selected_channels(selected.clone());
    }

    pub fn process_pending_reloads(&mut self, state: &TabState) {
        let mut to_remove_indices = Vec::new();

        for (index, pending_reload) in self.pending_reloads.clone().iter().enumerate() {
            if pending_reload.guild_id != self.current_guild {
                continue;
            }
            match pending_reload.reload_type {
                ReloadTab::Overview(guild_id) => {
                    if TabState::Overview == *state && guild_id == self.current_guild {
                        self.reload_overview(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
                ReloadTab::UserTable(guild_id) => {
                    if TabState::UserTable == *state && guild_id == self.current_guild {
                        self.user_table_recreate_rows(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
                ReloadTab::ChannelTable(guild_id) => {
                    if TabState::ChannelTable == *state && guild_id == self.current_guild {
                        self.channel_table_recreate_rows(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }

                ReloadTab::WordTable(guild_id) => {
                    if TabState::CommonWords == *state && guild_id == self.current_guild {
                        self.word_table_recreate_rows(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
                ReloadTab::MessageChart(guild_id) => {
                    if TabState::MessageChart == *state && guild_id == self.current_guild {
                        self.reload_message_chart(pending_reload.guild_id);
                        to_remove_indices.push(index);
                    }
                }
                ReloadTab::UserChart(guild_id) => {
                    if TabState::UserChart == *state && guild_id == self.current_guild {
                        self.reload_user_chart(pending_reload.guild_id);
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

    pub fn clear_key_data(&mut self, key: i64) {
        if self.overview.contains_key(&key) {
            self.overview.insert(key, Default::default());
        }
        if self.user_table.contains_key(&key) {
            self.user_table.insert(key, Default::default());
        }
        if self.channel_table.contains_key(&key) {
            self.channel_table.insert(key, Default::default());
        }
        if self.message_chart.contains_key(&key) {
            self.message_chart.insert(key, Default::default());
        }
        if self.user_chart.contains_key(&key) {
            self.user_chart.insert(key, Default::default());
        }
        if self.word_table.contains_key(&key) {
            self.word_table.insert(key, Default::default());
        }
    }
}

pub trait ShowUI {
    fn show_ui(&mut self, ui: &mut Ui, guild_id: i64, event_bus: &mut EventBus);
}
