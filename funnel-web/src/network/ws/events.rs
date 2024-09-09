use ewebsock::{WsEvent, WsMessage};
use funnel_shared::{Request, WsResponse};
use log::{error, info};

use crate::core::MainWindow;
use crate::network::handle_ws_message;

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
                            self.send_ws(Request::auth(self.password.temp_pass()));
                            self.password.set_authenticated();
                            self.password.clear_pass();
                            self.send_ws(Request::guilds());
                        }
                        WsEvent::Message(message) => {
                            if let WsMessage::Text(text) = message {
                                self.panels.next_dot();
                                let response = WsResponse::from_json(text);

                                if let Err(e) = response {
                                    error!("Failed to serialize message. Reason: {e}");
                                    return;
                                }

                                if let Some(reply) = handle_ws_message(self, response.unwrap()) {
                                    self.send_ws(reply);
                                };
                            } else {
                                error!("Unknown response gotten from server");
                            }
                            // let response = WsResponse::from_json(message.);
                        }
                    }
                }
            }
        }
    }
}
