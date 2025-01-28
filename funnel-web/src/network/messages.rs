use eframe::egui::{Context, OpenUrl};
use funnel_shared::{ErrorType, Request, Response, WsResponse, PAGE_VALUE};
use log::{error, info};

use crate::{AppEvent, AppStatus, MainWindow};

// const LOGIN_URL: &str = "https://discord.com/oauth2/authorize?client_id=1324028221066576017&response_type=code&redirect_uri=https%3A%2F%2Ffunnel-jyz9.shuttle.app%2Fauth%2Fredirect%2F&scope=guilds+identify";
const LOGIN_URL: &str = "https://discord.com/oauth2/authorize?client_id=1324028221066576017&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fauth%2Fredirect%2F&scope=guilds+identify";

pub fn handle_ws_message(
    window: &mut MainWindow,
    response: WsResponse,
    ctx: &Context,
) -> Option<Request> {
    if response.status.is_error() {
        handle_errors(window, response.get_error());
        return None;
    }

    match response.response {
        Response::ConnectionSuccess(conn_id) => {
            info!("Client successfully connected. Conn id: {conn_id}");

            // TODO: Maybe maintain some kind of cache instead of force login each time
            let full_url = format!("{LOGIN_URL}&state={}", conn_id);

            let open_url = OpenUrl {
                url: full_url,
                new_tab: true,
            };

            ctx.open_url(open_url);
            window.panels.set_app_status(AppStatus::LoggingIn);
        }
        Response::Guilds(guilds) => {
            window.connection.set_connected();
            window.panels.set_app_status(AppStatus::Fetching);

            for guild in &guilds {
                let guild_id = guild.guild.guild_id;
                window.tabs.set_data(guild_id);
                window
                    .tabs
                    .set_overview_channel_map(guild.guild.guild_id, guild.channels.clone());
                window
                    .tabs
                    .set_channel_table_channel_map(guild.guild.guild_id, guild.channels.clone());
            }
            window.panels.set_guild_channels(guilds);
            window.event_bus.publish(AppEvent::GuildChanged);
        }
        Response::Messages { guild_id, messages } => {
            let current_page = response.status.page();
            window
                .panels
                .current_guild_status_m()
                .set_messages_page(current_page);

            if messages.is_empty() {
                window.panels.current_guild_status_m().messages_done();
                window.to_set_idle();

                window
                    .event_bus
                    .publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::MessageChartNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::UserChartNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::WordTableNeedsReload(guild_id));
                return None;
            }

            let do_new_page = messages.len() as u64 == PAGE_VALUE;

            if do_new_page {
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
                window
                    .tabs
                    .handle_message_message_chart(&message, &mut window.event_bus);
                window
                    .tabs
                    .handle_message_user_chart(&message, &mut window.event_bus);
                window
                    .tabs
                    .handle_message_word_table(&message, &mut window.event_bus);
            }

            if !do_new_page {
                window.panels.current_guild_status_m().messages_done();
                window.to_set_idle();

                window
                    .event_bus
                    .publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::MessageChartNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::UserChartNeedsReload(guild_id));
                window
                    .event_bus
                    .publish_if_needed(AppEvent::WordTableNeedsReload(guild_id));
            }
        }
        Response::MemberCounts { guild_id, counts } => {
            let current_page = response.status.page();
            window
                .panels
                .current_guild_status_m()
                .set_counts_page(current_page);

            if counts.is_empty() {
                window.panels.current_guild_status_m().counts_done();
                window.to_set_idle();

                return None;
            }

            let do_new_page = counts.len() as u64 == PAGE_VALUE;

            if do_new_page {
                window.send_ws(Request::get_member_counts(guild_id, current_page + 1));
            }

            for count in counts {
                window
                    .tabs
                    .handle_member_count(guild_id, count, &mut window.event_bus);
            }

            window.tabs.clear_chart_labels(guild_id);

            if !do_new_page {
                window.panels.current_guild_status_m().counts_done();
                window.to_set_idle();

                window.tabs.fill_member_activity(guild_id);

                if !window.panels.current_guild_status().activities() {
                    window.send_ws(Request::get_member_activity(guild_id, 1));
                }
            }
        }
        Response::MemberActivities {
            guild_id,
            activities,
        } => {
            let current_page = response.status.page();
            window
                .panels
                .current_guild_status_m()
                .set_activities_page(current_page);

            if activities.is_empty() {
                window.panels.current_guild_status_m().activities_done();
                window.to_set_idle();
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
                window.panels.current_guild_status_m().activities_done();
                window.to_set_idle();
            }
        }
        Response::UserDetails(user_details) => {
            window.panels.set_user_details(user_details);
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
            window.remove_channels();
            window
                .panels
                .set_app_status(AppStatus::FailedWs(String::from(
                    "Client did not connect properly to the server",
                )));
        }
        ErrorType::FailedAuthentication => {
            error!("Failed to authenticate with Discord");
            window.connection.failed_connection();
            window.remove_channels();
            window.panels.set_app_status(AppStatus::FailedAuth);
        }
        ErrorType::NoValidGuild => {
            error!("No valid guild was found with this discord account");
            window.connection.failed_connection();
            window.remove_channels();
            window.panels.set_app_status(AppStatus::NoValidGuild);
        }

        ErrorType::UnknowError(reason) => {
            error!("Unexpected error. Reason: {reason}");
            window.connection.failed_connection();
            window.remove_channels();
            window
                .panels
                .set_app_status(AppStatus::UnexpectedError(reason));
        }
    }
}
