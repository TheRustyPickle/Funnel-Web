use ewebsock::Options;
use gloo_worker::{HandlerId, WorkerScope};
use log::{error, info};

use crate::core::MainWindow;
use crate::network::get_ws_code;
use crate::web_worker::{WebWorker, WorkerMessage};
use crate::AppStatus;

const WS_URL: &str = "wss://127.0.0.1:8081/ws";

pub async fn handler_worker_comms(
    message: WorkerMessage,
    scope: WorkerScope<WebWorker>,
    id: HandlerId,
) {
    match message {
        WorkerMessage::StartConnection(password) => {
            let result = get_ws_code(password).await;
            let pass = if let Err(e) = result {
                scope.respond(id, WorkerMessage::AuthError(e.to_string()));
                return;
            } else {
                result.unwrap()
            };
            scope.respond(id, WorkerMessage::WsPassword(pass));
        }
        WorkerMessage::WsPassword(_) | WorkerMessage::AuthError(_) => {}
    };
}

impl MainWindow {
    pub fn handle_main_comms(&mut self, message: WorkerMessage) -> Option<WorkerMessage> {
        match message {
            WorkerMessage::WsPassword(pass) => {
                let options = Options::default();
                let result = ewebsock::connect(WS_URL, options);
                match result {
                    Ok((sender, receiver)) => {
                        self.set_channels(sender, receiver);
                        self.password.set_temp_pass(pass);
                    }
                    Err(e) => {
                        info!("Failed to connect to WS. Reason: {e}");
                        self.password.failed_connection();
                        self.panels.set_app_status(AppStatus::FailedWs(e));
                    }
                }
            }
            WorkerMessage::AuthError(error) => {
                error!("Failed to authenticate. Reason: {error}");
                self.password.failed_connection();
                self.panels.set_app_status(AppStatus::FailedAuth(error));
            }
            WorkerMessage::StartConnection(_) => {
                error!("This message should not be received by this part");
            }
        }
        None
    }
}
