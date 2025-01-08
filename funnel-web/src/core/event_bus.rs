use ewebsock::Options;
use log::{error, info};
use std::collections::VecDeque;

use crate::core::MainWindow;
use crate::ui::ReloadTab;
use crate::{AppEvent, AppStatus};

const WS_URL: &str = "wss://funnel-jyz9.shuttle.app/ws";
// const WS_URL: &str = "ws://localhost:8000/ws";

impl MainWindow {
    pub fn check_event(&mut self) {
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
                    self.event_bus
                        .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::TableNeedsReload(guild_id));
                }
                AppEvent::CompareDate => {
                    let guild_id = self.panels.selected_guild();
                    self.tabs.compare_overview(guild_id);
                }
                AppEvent::StartWebsocket => {
                    info!("Start connection to the websocket");
                    self.panels.set_app_status(AppStatus::ConnectingToWs);
                    let options = Options::default();
                    let result = ewebsock::connect(WS_URL, options);
                    match result {
                        Ok((sender, receiver)) => {
                            self.set_channels(sender, receiver);
                        }
                        Err(e) => {
                            error!("Failed to connect to WS. Reason: {e}");
                            self.connection.failed_connection();
                            self.panels.set_app_status(AppStatus::FailedWs(e));
                        }
                    }
                }
                // Messages were gotten from the server and one of the tab is asking to update the earliest
                // to the latest date with at least 1 message
                AppEvent::UpdateDate(date, guild_id) => {
                    let date_handler = self.panels.date_update(date, guild_id);
                    self.tabs.set_date_handler(guild_id, date_handler);
                }
                AppEvent::CellsCopied => self.panels.set_app_status(AppStatus::CellsCopied),
                AppEvent::GuildChanged => {
                    self.tabs.set_current_guild(self.panels.selected_guild());
                }
                AppEvent::StopCompareOverview => {
                    let guild_id = self.panels.selected_guild();
                    self.tabs.stop_compare_overview(guild_id)
                }
                AppEvent::TableNeedsReload(guild_id) => {
                    self.tabs.add_reload(guild_id, ReloadTab::Table);
                }
                AppEvent::OverviewNeedsReload(guild_id) => {
                    self.tabs.add_reload(guild_id, ReloadTab::Overview);
                }
            }
        }
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

    pub fn publish_if_needed(&mut self, event: AppEvent) {
        if !self.events.contains(&event) {
            self.events.push_back(event);
        }
    }
}
