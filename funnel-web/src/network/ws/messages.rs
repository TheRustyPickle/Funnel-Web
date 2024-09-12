use funnel_shared::{ErrorType, Request, Response, WsResponse};
use log::{error, info};

use crate::{AppStatus, MainWindow};

pub fn handle_ws_message(window: &mut MainWindow, response: WsResponse) -> Option<Request> {
    if response.status.is_error() {
        handle_errors(window, response.get_error());
        return None;
    }
    match response.response {
        Response::AuthenticationSuccess => {
            info!("Client successfully authenticated. Getting guild list");
            window.panels.set_app_status(AppStatus::Fetching);
            window.send_ws(Request::guilds());
        }
        Response::Guilds(guilds) => {
            window.panels.set_guild_channels(guilds);
            window.panels.set_app_status(AppStatus::Idle);
        }
        Response::Messages(messages) => {}
        Response::Error(_) => unreachable!(),
    }
    None
}

fn handle_errors(window: &mut MainWindow, error: ErrorType) {
    match error {
        ErrorType::AuthenticationFailed(reason) => {
            error!("Authentication attempt with the server failed. Reason: {reason}",);
            window.password.failed_connection();
            window.panels.set_app_status(AppStatus::FailedAuth(reason));
        }
        ErrorType::ClientNotAuthenticated => {
            error!("Client is not authenticated by the server.");
            window.password.failed_connection();
            window
                .panels
                .set_app_status(AppStatus::FailedAuth(String::from(
                    "Client is not authenticated",
                )));
        }
        ErrorType::UnknowError(reason) => {
            error!("Unexpected error. Reason: {reason}")
            // TODO: Perhaps maintain an enum for work process => restart that
            // Fetch Guild => Fetch Message => ??
        }
    }
}
