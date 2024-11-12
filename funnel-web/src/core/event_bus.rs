use eframe::egui::Context;
use ewebsock::Options;
use log::{error, info};
use std::collections::VecDeque;

use crate::core::MainWindow;
use crate::{AppEvent, AppStatus};

const WS_URL: &str = "wss://127.0.0.1:8081/ws";

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
                    self.tabs.recreate_rows(guild_id, &mut self.event_bus);
                }
                AppEvent::CompareDate => {
                    // let guild_id = self.panels.selected_guild();
                }
                // Pressed on submit
                AppEvent::PasswordSubmitted => {
                    info!("Password submitted");
                    self.panels.set_app_status(AppStatus::CheckingAuth);
                }
                AppEvent::PasswordFailed(error) => {
                    error!("Failed to authenticate. Reason: {error}");
                    self.password.failed_connection();
                    self.panels.set_app_status(AppStatus::FailedAuth(error));
                }
                AppEvent::StartWebsocket(pass) => {
                    let options = Options::default();
                    let result = ewebsock::connect(WS_URL, options);
                    match result {
                        Ok((sender, receiver)) => {
                            self.set_channels(sender, receiver);
                            self.password.set_temp_pass(pass);
                        }
                        Err(e) => {
                            info!("Failed to connect to WS. Reason: {e}");
                            self.password.failed_connection();
                            self.panels.set_app_status(AppStatus::FailedWs(e));
                        }
                    }
                }

                // Messages were gotten from the server and table is asking to update the earliest
                // to the latest date with at least 1 message
                AppEvent::TableUpdateDate(date, guild_id) => {
                    let date_handler = self.panels.date_update(date, guild_id);
                    self.tabs.set_date_handler(guild_id, date_handler);
                    self.tabs.recreate_rows(guild_id, &mut self.event_bus);
                }
                AppEvent::CellsCopied => self.panels.set_app_status(AppStatus::CellsCopied),
                AppEvent::GuildChanged => {
                    self.tabs.set_current_guild(self.panels.selected_guild());
                }
                AppEvent::TableReloaded(guild_id) => self.tabs.reload_overview(guild_id),
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
