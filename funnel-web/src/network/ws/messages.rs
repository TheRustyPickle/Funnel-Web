use funnel_shared::{ErrorType, Request, Response, WsResponse, PAGE_VALUE};
use log::{error, info};

use crate::{AppEvent, AppStatus, MainWindow};

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
            // TODO: Do not request stuff for all guilds. Mark 1 guild as selected => Get data for
            // that only. Once a new guild is selected, fetch data for it as required
            for guild in &guilds {
                let guild_id = guild.guild.guild_id;
                window.send_ws(Request::get_messages(guild_id, 1));
                window.send_ws(Request::get_member_counts(guild_id, 1));
                window.tabs.set_data(guild_id);
                window
                    .tabs
                    .set_channel_map(guild.guild.guild_id, guild.channels.clone())
            }
            window.panels.set_guild_channels(guilds);
            window
                .tabs
                .set_current_guild(window.panels.selected_guild());
        }
        Response::Messages { guild_id, messages } => {
            if messages.is_empty() {
                window
                    .event_bus
                    .publish_if_needed(AppEvent::TableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
                return None;
            }

            let do_new_page = messages.len() as u64 == PAGE_VALUE;

            if do_new_page {
                let current_page = response.status.page();
                window.send_ws(Request::get_messages(guild_id, current_page + 1));
            }
            for message in messages {
                window
                    .tabs
                    .handle_message_user_table(message.clone(), &mut window.event_bus);
                window
                    .tabs
                    .handle_message_overview(message, &mut window.event_bus);
            }

            if !do_new_page {
                window
                    .event_bus
                    .publish_if_needed(AppEvent::TableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
            }
        }
        Response::MemberCounts { guild_id, counts } => {
            info!(
                "Got member count data for {guild_id} with data count {}",
                counts.len()
            );
        }
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
