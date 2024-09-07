use ewebsock::Options;
use gloo_worker::{HandlerId, WorkerScope};
use log::{error, info};

use crate::core::MainWindow;
use crate::network::get_ws_code;
use crate::web_worker::{WebWorker, WorkerMessage};

const WS_URL: &str = "wss://127.0.0.1:8081/ws";

pub async fn handler_worker_comms(
    message: WorkerMessage,
    scope: WorkerScope<WebWorker>,
    id: HandlerId,
) {
    match message {
        WorkerMessage::StartConnection(password) => {
            let _ = get_ws_code(password).await;
            scope.respond(id, WorkerMessage::WsPassword("SomeCode".to_string()));
        }
        WorkerMessage::ConnectionEstablished | WorkerMessage::WsPassword(_) => {}
    };
}

impl MainWindow {
    pub fn handle_main_comms(&mut self, message: WorkerMessage) -> Option<WorkerMessage> {
        match message {
            WorkerMessage::ConnectionEstablished => {
                info!("Connected to the server successfully");
                self.password.set_authenticated();
            }
            WorkerMessage::WsPassword(pass) => {
                let options = Options::default();
                let result = ewebsock::connect(WS_URL, options);
                match result {
                    Ok((sender, receiver)) => {
                        self.set_channels(sender, receiver);
                        self.password.set_temp_pass(pass);
                    }
                    Err(e) => {
                        info!("Failed to connect, {e}");
                        self.password.failed_connection();
                    }
                }
            }
            WorkerMessage::StartConnection(_) => {
                error!("This message should not be received by this part");
            }
        }
        None
    }
}
