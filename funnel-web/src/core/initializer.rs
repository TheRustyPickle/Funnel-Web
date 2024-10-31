use eframe::{egui, App, Frame};
use egui::{Context, ThemePreference};
use egui_extras::install_image_loaders;
use ewebsock::WsMessage;
use ewebsock::{WsReceiver, WsSender};
use funnel_shared::Request;

use crate::core::add_font;
use crate::ui::{PanelStatus, Password, TabHandler};
use crate::EventBus;

pub const JET: &[u8] = include_bytes!("../../../fonts/jetbrains_nerd_propo_regular.ttf");

pub struct MainWindow {
    pub password: Password,
    pub panels: PanelStatus,
    pub tabs: TabHandler,
    pub event_bus: EventBus,
    pub ws_sender: Option<WsSender>,
    pub ws_receiver: Option<WsReceiver>,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.check_event(ctx);
        self.check_ws_receiver();
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
            password: Password::default(),
            panels: PanelStatus::default(),
            tabs: TabHandler::default(),
            event_bus: EventBus::default(),
            ws_sender: None,
            ws_receiver: None,
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
        }
    }
}
