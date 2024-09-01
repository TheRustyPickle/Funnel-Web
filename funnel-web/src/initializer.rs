use eframe::{App, Frame};
use egui::{
    global_dark_light_mode_switch, menu, Align, CentralPanel, Context, Image, Layout, ScrollArea,
    SidePanel, Spinner, TopBottomPanel,
};
use egui_extras::{install_image_loaders, DatePickerButton};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::ui::{PanelStatus, PasswordStatus};

pub struct MainWindow {
    pub password: PasswordStatus,
    pub panels: PanelStatus,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Default for MainWindow {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver,
            password: PasswordStatus::default(),
            panels: PanelStatus::default(),
        }
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.show_panels(ctx);
    }
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.3);
        install_image_loaders(&cc.egui_ctx);
        Default::default()
    }
}
