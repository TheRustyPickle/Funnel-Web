use ewebsock::WsEvent;
use log::{error, info};

use crate::core::MainWindow;

impl MainWindow {
    pub fn check_ws_receiver(&mut self) {
        if self.ws_receiver.is_some() {
            for _ in 0..50 {
                if let Some(event) = self.ws_receiver.as_ref().unwrap().try_recv() {
                    match event {
                        WsEvent::Closed => {
                            info!("Connection to WS has been closed");
                            self.remove_channels();
                            self.password.failed_connection();
                            break;
                        }
                        WsEvent::Error(e) => {
                            error!("Error in ws. Reason: {e}");
                        }
                        WsEvent::Opened => {
                            info!("Connection to WS has been opened");
                            self.send_ws(format!("/password {}", self.password.temp_pass()));
                            self.password.set_authenticated();
                            self.password.clear_pass();
                        }
                        WsEvent::Message(message) => {
                            info!("Got a new event. Details: {message:#?}");
                        }
                    }
                }
            }
        }
    }
}
