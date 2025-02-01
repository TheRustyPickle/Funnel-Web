use ewebsock::Options;
use funnel_shared::Request;
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
                AppEvent::DateChanged => {
                    let guild_id = self.panels.selected_guild();
                    let date_handler = self.panels.current_date_handler();
                    self.tabs.set_date_handler(guild_id, date_handler);
                    self.event_bus
                        .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::WordTableNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::MessageChartNeedsReload(guild_id));
                    self.event_bus
                        .publish_if_needed(AppEvent::UserChartNeedsReload(guild_id));
                }
                AppEvent::CompareDate => {
                    let guild_id = self.panels.selected_guild();
                    self.tabs.compare_overview(guild_id);
                }
                AppEvent::StartWebsocket => {
                    if !self.has_channels() {
                        info!("Starting connection to the websocket");
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
                    } else {
                        info!("websocket connection already exists. Not creating a new one");
                        let no_login = self.connection.no_login();
                        if no_login {
                            self.send_ws(Request::start_no_login());
                        } else {
                            self.send_ws(Request::start());
                        }
                    }
                }
                AppEvent::UpdateDate(date, guild_id) => {
                    let date_handler = self.panels.date_update(date, guild_id);
                    self.tabs.set_date_handler(guild_id, date_handler);
                }
                AppEvent::CellsCopied => self.panels.set_app_status(AppStatus::CellsCopied),
                AppEvent::GuildChanged => {
                    let guild_id = self.panels.selected_guild();
                    self.tabs.set_current_guild(guild_id);

                    let guild_channels = self.panels.current_guild_channels();
                    self.tabs.set_channels(guild_channels);

                    let selected_channels = self.panels.current_selected_channels();
                    self.tabs.set_selected_channels(selected_channels);

                    self.fetch_guild_data();
                }
                AppEvent::StopCompareOverview => {
                    let guild_id = self.panels.selected_guild();
                    self.tabs.stop_compare_overview(guild_id)
                }
                AppEvent::OverviewNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::Overview(guild_id));
                }
                AppEvent::UserTableNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::UserTable(guild_id));
                }
                AppEvent::ChannelTableNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::ChannelTable(guild_id));
                }
                AppEvent::WordTableNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::WordTable(guild_id));
                }
                AppEvent::MessageChartNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::MessageChart(guild_id));
                }
                AppEvent::UserChartNeedsReload(guild_id) => {
                    self.tabs
                        .add_reload(guild_id, ReloadTab::UserChart(guild_id));
                }
                AppEvent::MessageChartTypeChanged(guild_id) => {
                    self.tabs.reload_message_chart(guild_id);
                }
                AppEvent::UserChartTypeChanged(guild_id) => {
                    self.tabs.reload_user_chart(guild_id);
                }
                AppEvent::SelectedChannelsChanged => {
                    let current_guild = self.panels.selected_guild();
                    let selected_channels = self.panels.current_selected_channels();
                    self.tabs.set_selected_channels(selected_channels);

                    self.event_bus
                        .publish_if_needed(AppEvent::OverviewNeedsReload(current_guild));
                    self.event_bus
                        .publish_if_needed(AppEvent::UserTableNeedsReload(current_guild));
                    self.event_bus
                        .publish_if_needed(AppEvent::MessageChartNeedsReload(current_guild));
                    self.event_bus
                        .publish_if_needed(AppEvent::UserChartNeedsReload(current_guild));
                    self.event_bus
                        .publish_if_needed(AppEvent::WordTableNeedsReload(current_guild));
                }
                AppEvent::LogOut => {
                    self.panels.set_app_status(AppStatus::AttemptLogOut);
                    self.send_ws(Request::LogOut);
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
