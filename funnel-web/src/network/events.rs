use ewebsock::{WsEvent, WsMessage};
use funnel_shared::{Request, WsResponse};
use log::{error, info};

use crate::core::MainWindow;
use crate::network::handle_ws_message;
use crate::AppStatus;

impl MainWindow {
    pub fn check_ws_receiver(&mut self) {
        if self.ws_receiver.is_some() {
            if let Some(event) = self.ws_receiver.as_ref().unwrap().try_recv() {
                match event {
                    WsEvent::Closed => {
                        info!("Connection to WS has been closed");
                        self.remove_channels();
                        self.connection.failed_connection();
                        self.panels.set_app_status(AppStatus::FailedWs(
                            "The websocket connection was closed".to_string(),
                        ));
                    }
                    WsEvent::Error(e) => {
                        error!("Error in ws. Reason: {e}");
                        self.panels.set_app_status(AppStatus::FailedWs(e));
                    }
                    WsEvent::Opened => {
                        info!("Connection to WS has been opened");
                        self.send_ws(Request::StartConnection);
                        self.connection.set_connected();
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
                    }
                }
            }
        }
    }
}
