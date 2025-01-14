use funnel_shared::{ErrorType, Request, Response, WsResponse, PAGE_VALUE};
use log::{error, info};

use crate::{AppEvent, AppStatus, MainWindow};

pub fn handle_ws_message(window: &mut MainWindow, response: WsResponse) -> Option<Request> {
    if response.status.is_error() {
        handle_errors(window, response.get_error());
        return None;
    }

    match response.response {
        Response::ConnectionSuccess => {
            info!("Client successfully connected. Getting guild list");
            window.panels.set_app_status(AppStatus::Fetching);
            window.send_ws(Request::guilds());
        }
        Response::Guilds(guilds) => {
            // TODO: Do not request stuff for all guilds. Mark one guild as selected => Get data for
            // that only. Once a new guild is selected, fetch data for it as required
            for guild in &guilds {
                let guild_id = guild.guild.guild_id;
                window.send_ws(Request::get_messages(guild_id, 1));
                window.send_ws(Request::get_member_counts(guild_id, 1));
                window.tabs.set_data(guild_id);
                window
                    .tabs
                    .set_overview_channel_map(guild.guild.guild_id, guild.channels.clone());
                window
                    .tabs
                    .set_channel_table_channel_map(guild.guild.guild_id, guild.channels.clone());
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
                    .publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
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
                    .handle_message_user_table(&message, &mut window.event_bus);
                window
                    .tabs
                    .handle_message_channel_table(&message, &mut window.event_bus);
                window
                    .tabs
                    .handle_message_overview(&message, &mut window.event_bus);
            }

            if !do_new_page {
                window
                    .event_bus
                    .publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
            }
        }
        Response::MemberCounts { guild_id, counts } => {
            if counts.is_empty() {
                return None;
            }

            let do_new_page = counts.len() as u64 == PAGE_VALUE;

            if do_new_page {
                let current_page = response.status.page();
                window.send_ws(Request::get_member_counts(guild_id, current_page + 1));
            }

            for count in counts {
                window
                    .tabs
                    .handle_member_count(guild_id, count, &mut window.event_bus);
            }

            window.tabs.clear_chart_labels(guild_id);

            if !do_new_page {
                window.tabs.fill_member_activity(guild_id);
                window.send_ws(Request::get_member_activity(guild_id, 1));
            }
        }
        Response::MemberActivities {
            guild_id,
            activities,
        } => {
            if activities.is_empty() {
                window.panels.set_app_status(AppStatus::Idle);
                return None;
            }

            let do_new_page = activities.len() as u64 == PAGE_VALUE;

            if do_new_page {
                let current_page = response.status.page();
                window.send_ws(Request::get_member_counts(guild_id, current_page + 1));
            }

            for activity in activities {
                window
                    .tabs
                    .handle_member_activity(guild_id, activity, &mut window.event_bus);
            }

            window.tabs.clear_chart_labels(guild_id);

            if !do_new_page {
                window.panels.set_app_status(AppStatus::Idle);
            }
        }
        Response::Error(_) => unreachable!(),
    }
    None
}

fn handle_errors(window: &mut MainWindow, error: ErrorType) {
    match error {
        ErrorType::ClientNotConnected => {
            error!("Client did not connect properly to the server");
            window.connection.failed_connection();
            window
                .panels
                .set_app_status(AppStatus::FailedWs(String::from(
                    "Client did not connect properly to the server",
                )));
        }
        ErrorType::UnknowError(reason) => {
            error!("Unexpected error. Reason: {reason}")
            // TODO: Perhaps maintain an enum for work process => restart that
            // Fetch Guild => Fetch Message => ??
        }
    }
}
