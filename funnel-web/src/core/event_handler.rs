use egui::Context;

use crate::core::MainWindow;
use crate::web_worker::WorkerMessage;
use crate::AppEvent;

impl MainWindow {
    pub fn check_event(&mut self, ctx: &Context) {
        while !self.events.is_empty() {
            let event = self.events.pop_front().unwrap();

            match event {
                AppEvent::DateChanged => {
                    // TODO:
                }
                AppEvent::StartWsConnection => {
                    let password = self.password.pass.clone();
                    self.send(WorkerMessage::StartConnection(password));
                }
            }
        }
        ctx.request_repaint();
    }
}
