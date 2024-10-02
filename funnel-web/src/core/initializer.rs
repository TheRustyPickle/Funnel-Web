use eframe::{egui, App, Frame};
use egui::{Context, ThemePreference};
use egui_extras::install_image_loaders;
use ewebsock::WsMessage;
use ewebsock::{WsReceiver, WsSender};
use funnel_shared::Request;
use gloo_worker::{Spawnable, WorkerBridge};
use std::cell::Cell;
use std::rc::Rc;

use crate::ui::{PanelStatus, PasswordStatus, TabHandler};
use crate::web_worker::{WebWorker, WorkerMessage};
use crate::EventBus;

pub struct MainWindow {
    pub password: PasswordStatus,
    pub panels: PanelStatus,
    pub tabs: TabHandler,
    pub event_bus: EventBus,
    pub bridge: Option<WorkerBridge<WebWorker>>,
    pub data_update: Option<Rc<Cell<Option<WorkerMessage>>>>,
    pub ws_sender: Option<WsSender>,
    pub ws_receiver: Option<WsReceiver>,
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.check_event(ctx);
        let data_update = self.data_update.as_mut().unwrap();
        if let Some(message) = data_update.take() {
            if let Some(reply) = self.handle_main_comms(message) {
                self.send(reply);
            };
            ctx.request_repaint();
        };
        self.check_ws_receiver();
        self.show_panels(ctx);
    }
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.1);
        cc.egui_ctx.set_theme(ThemePreference::Light);

        install_image_loaders(&cc.egui_ctx);

        let ctx = cc.egui_ctx.clone();
        let data_update = Rc::new(Cell::new(None));
        let sender = data_update.clone();
        let bridge = <WebWorker as Spawnable>::spawner()
            .callback(move |response| {
                sender.set(Some(response));
                ctx.request_repaint();
            })
            .spawn("./dummy_worker.js");

        Self {
            password: PasswordStatus::default(),
            panels: PanelStatus::default(),
            tabs: TabHandler::default(),
            event_bus: EventBus::default(),
            bridge: Some(bridge),
            data_update: Some(data_update),
            ws_sender: None,
            ws_receiver: None,
        }
    }

    pub fn send(&mut self, message: WorkerMessage) {
        self.bridge.as_mut().unwrap().send(message);
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
