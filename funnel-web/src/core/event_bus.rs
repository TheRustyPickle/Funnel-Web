use eframe::egui::Context;
use std::collections::VecDeque;

use crate::core::MainWindow;
use crate::web_worker::WorkerMessage;
use crate::{AppEvent, AppStatus};

impl MainWindow {
    pub fn check_event(&mut self, ctx: &Context) {
        loop {
            let Some(event) = self.event_bus.get() else {
                break;
            };

            match event {
                // When date changed from the top of the UI
                AppEvent::DateChanged => {
                    let guild_id = self.panels.selected_guild();
                    let date_handler = self.panels.current_date_handler();
                    self.tabs.set_date_handler(guild_id, date_handler);
                    self.tabs.recreate_rows(guild_id);
                }
                AppEvent::CompareDate => {}
                AppEvent::CompareVisibility => {
                    // self.tabs.set_overview_compare(self.panels.show_compared());
                }
                // Pressed on submit
                AppEvent::StartWsConnection => {
                    let password = self.password.pass.clone();
                    self.send(WorkerMessage::StartConnection(password));
                    self.panels.set_app_status(AppStatus::CheckingAuth);
                }
                // Messages were gotten from the server and table is asking to update the earliest
                // to the latest date with at least 1 message
                AppEvent::TableUpdateDate(date, guild_id) => {
                    let date_handler = self.panels.date_update(date, guild_id);
                    self.tabs.set_date_handler(guild_id, date_handler);
                    self.tabs.recreate_rows(guild_id);
                }
                AppEvent::CellsCopied => self.panels.set_app_status(AppStatus::CellsCopied),
                AppEvent::GuildChanged => {
                    self.tabs.set_current_guild(self.panels.selected_guild());
                }
            }
        }
        ctx.request_repaint();
    }
}

#[derive(Default)]
pub struct EventBus {
    events: VecDeque<AppEvent>,
}

impl EventBus {
    pub fn publish(&mut self, event: AppEvent) {
        self.events.push_back(event);
    }

    pub fn get(&mut self) -> Option<AppEvent> {
        self.events.pop_front()
    }
}
