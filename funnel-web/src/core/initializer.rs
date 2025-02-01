use eframe::{egui, App, Frame};
use egui::{Context, ThemePreference};
use egui_extras::install_image_loaders;
use ewebsock::WsMessage;
use ewebsock::{WsReceiver, WsSender};
use funnel_shared::Request;
use log::{error, info};

use crate::core::add_font;
use crate::ui::{Connection, PanelStatus, TabHandler};
use crate::{AppStatus, EventBus};

pub const JET: &[u8] = include_bytes!("../../../fonts/jetbrains_nerd_propo_regular.ttf");
pub const CHANGE: &[u8] = include_bytes!("../../../CHANGELOG.md");

pub struct MainWindow {
    pub connection: Connection,
    pub panels: PanelStatus,
    pub tabs: TabHandler,
    pub event_bus: EventBus,
    pub ws_sender: Option<WsSender>,
    pub ws_receiver: Option<WsReceiver>,
    pub conn_id: u64,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.check_event();
        self.check_ws_receiver(ctx);
        self.show_panels(ctx);
        ctx.request_repaint();
    }
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.1);
        cc.egui_ctx.set_theme(ThemePreference::Light);

        install_image_loaders(&cc.egui_ctx);

        let ctx = cc.egui_ctx.clone();
        add_font(&ctx);

        Self {
            connection: Connection::default(),
            panels: PanelStatus::default(),
            tabs: TabHandler::default(),
            event_bus: EventBus::default(),
            ws_sender: None,
            ws_receiver: None,
            conn_id: 0,
        }
    }

    pub fn set_channels(&mut self, sender: WsSender, receiver: WsReceiver) {
        self.ws_sender = Some(sender);
        self.ws_receiver = Some(receiver);
    }

    pub fn remove_channels(&mut self) {
        self.ws_sender = None;
        self.ws_receiver = None;
    }

    pub fn send_ws(&mut self, message: Request) {
        if let Some(sender) = self.ws_sender.as_mut() {
            sender.send(WsMessage::Text(message.to_json()));
        } else {
            error!(
                "Attempted to send a message to the websocket without a connection. {message:#?}"
            );
        }
    }

    pub fn to_set_idle(&mut self) {
        let all_done = self.panels.current_guild_status().all_done();

        if all_done {
            self.panels.set_app_status(AppStatus::Idle);
        }
    }

    pub fn fetch_guild_data(&mut self) {
        let guild_id = self.panels.selected_guild();
        let fetch_status = self.panels.current_guild_status_m();
        let messages_done = fetch_status.messages();
        let counts_done = fetch_status.counts();
        let activities_done = fetch_status.activities();

        let mut nothing_fetched = true;

        if !messages_done {
            nothing_fetched = false;
            self.send_ws(Request::get_messages(guild_id, 1));
        }

        if !counts_done {
            nothing_fetched = false;
            self.send_ws(Request::get_member_counts(guild_id, 1));
        }

        if counts_done && !activities_done {
            nothing_fetched = false;
            self.send_ws(Request::get_member_activity(guild_id, 1));
        }

        if nothing_fetched {
            self.panels.set_app_status(AppStatus::Idle);
        }
    }

    pub fn reset_all(&mut self) {
        info!("Resetting all data");
        *self = Self {
            connection: Connection::default(),
            panels: PanelStatus::default(),
            tabs: TabHandler::default(),
            event_bus: EventBus::default(),
            ws_sender: None,
            ws_receiver: None,
            conn_id: 0,
        }
    }

    pub fn has_channels(&self) -> bool {
        self.ws_sender.is_some() && self.ws_receiver.is_some()
    }
}
