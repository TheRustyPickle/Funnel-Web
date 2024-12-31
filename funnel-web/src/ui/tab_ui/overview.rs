use chrono::{DateTime, NaiveDate};
use eframe::egui::Ui;
use funnel_shared::{Channel, MessageWithUser, PAGE_VALUE};
use std::collections::HashMap;

use crate::core::{compare_number, to_header};
use crate::ui::{Card, DateHandler, DateNavigator, ShowUI, TabHandler};
use crate::{AppEvent, EventBus};

#[derive(Default, Debug)]
pub struct ActivityData {
    // Channel ID + Message Count
    message_count: HashMap<i64, u32>,
    deleted_message: u32,
    name: String,
}

impl ActivityData {
    fn new(name: String) -> Self {
        Self {
            message_count: HashMap::default(),
            deleted_message: 0,
            name,
        }
    }
}

#[derive(Default)]
pub struct OverviewData {
    total_message: u32,
    deleted_message: u32,
    unique_user: u32,
    member_count: u32,
    member_joins: u32,
    member_left: u32,
    most_active_member: String,
    most_active_channel: String,
}

#[derive(Default)]
pub struct Overview {
    activity_data: HashMap<NaiveDate, HashMap<String, ActivityData>>,
    channel_map: HashMap<i64, String>,
    data: OverviewData,
    compare_data: Option<OverviewData>,
    card_size: f32,
    compare_nav: DateNavigator,
    compare_size: f32,
    date_handler: DateHandler,
    max_content: usize,
    reload_count: u64,
}

impl ShowUI for Overview {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        self.show_compare_buttons(ui, event_bus);

        let total_message_id = ui.make_persistent_id("overview_total_message");
        let unique_user_id = ui.make_persistent_id("overview_unique_user");
        let compare_total_message = ui.make_persistent_id("overview_compare_message");
        let compare_unique_user = ui.make_persistent_id("overview_compare_user");

        let space_3_item = ui.make_persistent_id("card_space_3");
        let space_2_item = ui.make_persistent_id("card_space_2");

        ui.vertical(|ui| {
            let has_compare = self.compare_data.is_some();

            if !has_compare {
                ui.ctx().animate_value_with_time(compare_total_message, 0.0, 0.0);
                ui.ctx().animate_value_with_time(compare_unique_user, 0.0, 0.0);
            }

            let x_size = if self.max_content != usize::default() && has_compare {
                self.max_content as f32 * 12.0
            } else {
                250.0
            };
            let y_size = 70.0;

            let mut space_3 = 0.0;
            let mut space_2 = 0.0;

            if self.card_size != 0.0 {
                let max_size = ui.available_width();
                let space_taken = 3.0 * self.card_size;
                let remaining = max_size - space_taken;
                let remaining = ui.painter().round_to_pixel_center(remaining);
                let space_amount =
                    ui.ctx()
                        .animate_value_with_time(space_3_item, remaining / 2.0, 0.5);
                space_3 = space_amount;

                let space_taken = 2.0 * self.card_size;
                let remaining = max_size - space_taken;
                let remaining = ui.painter().round_to_pixel_center(remaining);
                let space_amount =
                    ui.ctx()
                        .animate_value_with_time(space_2_item, remaining / 2.0, 0.5);
                space_2 = space_amount;
            } else {
                ui.ctx().animate_value_with_time(space_3_item, 0.0, 0.0);
                ui.ctx().animate_value_with_time(space_2_item, 0.0, 0.0);
                ui.ctx().animate_value_with_time(total_message_id, 0.0, 0.0);
                ui.ctx().animate_value_with_time(unique_user_id, 0.0, 0.0);
            }

            ui.horizontal(|ui| {
                ui.add_space(space_3);
                let remaining_width = ui.available_width();
                let mut header_text = "Total Message".to_string();
                let content_text = ui.ctx().animate_value_with_time(total_message_id, self.data.total_message as f32, 1.0) as u32;
                let mut hover_text = "Total message gotten within selected time period".to_string();
                if has_compare {
                    let comparing_with = self.compare_data.as_ref().unwrap().total_message;
                    let difference = compare_number(ui,
                        comparing_with,
                        content_text,
                        compare_total_message);
                    header_text += &format!(" {difference}");
                    let header_text_len = header_text.chars().count();
                    if header_text_len > self.max_content {
                        self.max_content = header_text_len
                    }
                    hover_text +=
                        &format!("\nTotal message gotten in the compare time: {comparing_with}");
                }
                ui.add(Card::new(
                    to_header(header_text),
                    to_header(content_text),
                    x_size,
                    y_size,
                ))
                .on_hover_text(hover_text);
                let space_taken = remaining_width - ui.available_width();
                self.card_size = space_taken;

                ui.add(Card::new(
                    to_header("Deleted Message"),
                    to_header(self.data.deleted_message),
                    x_size,
                    y_size,
                ));

                let mut header_text = "Unique User".to_string();
                let content_text = ui.ctx().animate_value_with_time(unique_user_id, self.data.unique_user as f32, 0.5) as u32;
                let mut hover_text =
                    "Total unique users gotten within selected time period".to_string();

                if has_compare {
                    let comparing_with = self.compare_data.as_ref().unwrap().unique_user;
                    let difference = compare_number(ui, comparing_with, content_text, compare_unique_user);
                    header_text += &format!(" {difference}");
                    hover_text += &format!(
                        "\nTotal unique users gotten in the compare time: {comparing_with}"
                    );
                    let header_text_len = header_text.chars().count();
                    if header_text_len > self.max_content {
                        self.max_content = header_text_len
                    }
                }

                ui.add(Card::new(
                    to_header(header_text),
                    to_header(content_text),
                    x_size,
                    y_size,
                ))
                .on_hover_text(hover_text);
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add_space(space_3);
                ui.add(Card::new(
                    to_header("Member Count"),
                    to_header(self.data.member_count),
                    x_size,
                    y_size,
                ));
                ui.add(Card::new(
                    to_header("Member Joins"),
                    to_header(self.data.member_joins),
                    x_size,
                    y_size,
                ));
                ui.add(Card::new(
                    to_header("Member Left"),
                    to_header(self.data.member_left),
                    x_size,
                    y_size,
                ));
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                let mut hover_text = "The user with the most messages sent within selected time period".to_string();
                if has_compare {
                    let comparing_with = self.compare_data.as_ref().unwrap().most_active_member.to_string();
                    hover_text += &format!(
                        "\nThe user with the most messages sent in the compare time: {comparing_with}"
                    );
                }
                ui.add_space(space_2);
                ui.add(Card::new(
                    to_header("Most Active Member"),
                    to_header(&self.data.most_active_member),
                    x_size,
                    y_size,
                )).on_hover_text(hover_text);


                let mut hover_text = "The channel with the most messages sent within selected time period".to_string();
                if has_compare {
                    let comparing_with = self.compare_data.as_ref().unwrap().most_active_channel.to_string();
                    hover_text += &format!(
                        "\nThe channel with the most messages sent in the compare time: {comparing_with}"
                    );
                }

                ui.add(Card::new(
                    to_header("Most Active Channel"),
                    to_header(&self.data.most_active_channel),
                    x_size,
                    y_size,
                )).on_hover_text(hover_text);
            })
        });
        ui.add_space(50.0);
        ui.vertical_centered(|ui| {
            ui.heading("Member Movement Chart Under Construction");
        });
    }
}

impl Overview {
    fn show_compare_buttons(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let spacing_size = ui.available_width() - self.compare_size;
            let spacing_size = ui.painter().round_to_pixel_center(spacing_size / 2.0);
            if spacing_size > 0.0 {
                ui.add_space(spacing_size);
            };
            let max_width = ui.available_width();
            self.compare_nav.show_ui_compare(ui, event_bus);
            let consumed = max_width - ui.available_width();
            self.compare_size = consumed;
        });
        ui.add_space(10.0);
    }

    fn handle_message(&mut self, message: MessageWithUser, event_bus: &mut EventBus) {
        let username = &message.sender.username;
        let channel_id = message.message.channel_id;
        let guild_id = message.message.guild_id;

        let timestamp = &message.message.message_timestamp;

        let datetime = DateTime::from_timestamp(*timestamp, 0).unwrap();
        let local_time = datetime.naive_local();
        let local_date = local_time.date();
        let activity = ActivityData::new(username.to_string());

        let entry = self.activity_data.entry(local_date).or_default();

        let target_entry = entry.entry(username.to_string()).or_insert(activity);

        let count_entry = target_entry.message_count.entry(channel_id).or_default();
        *count_entry += 1;
        self.reload_count += 1;

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
        }
    }

    fn reload_overview(&mut self) {
        let mut channel_message_count = HashMap::new();
        let mut member_message_count = HashMap::new();
        let mut total_message = 0;
        self.reload_count = 0;

        self.activity_data
            .iter()
            .filter(|(date, _)| self.date_handler.within_range(**date))
            .for_each(|(_, activities)| {
                for activity in activities.values() {
                    for (&channel_id, &count) in &activity.message_count {
                        *channel_message_count.entry(channel_id).or_insert(0) += count;
                        *member_message_count
                            .entry(activity.name.clone())
                            .or_insert(0) += count;
                        total_message += count;
                    }
                }
            });

        let unique_user = member_message_count.len() as u32;

        let most_active_channel = channel_message_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or((0, 0));

        let most_active_member = member_message_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or((String::new(), 0));

        let channel_name = if let Some(name) = self.channel_map.get(&most_active_channel.0) {
            name.to_string()
        } else {
            String::new()
        };

        let overview = OverviewData {
            total_message,
            deleted_message: 0,
            member_count: 0,
            unique_user,
            member_joins: 0,
            member_left: 0,
            most_active_member: most_active_member.0,
            most_active_channel: channel_name,
        };
        self.data = overview;
    }

    pub fn create_compare_data(&mut self) {
        self.max_content = usize::default();
        let mut channel_message_count = HashMap::new();
        let mut member_message_count = HashMap::new();
        let mut total_message = 0;

        self.activity_data
            .iter()
            .filter(|(date, _)| self.compare_nav.handler().within_range(**date))
            .for_each(|(_, activities)| {
                for activity in activities.values() {
                    for (&channel_id, &count) in &activity.message_count {
                        *channel_message_count.entry(channel_id).or_insert(0) += count;
                        *member_message_count
                            .entry(activity.name.clone())
                            .or_insert(0) += count;
                        total_message += count;
                    }
                }
            });

        let unique_user = member_message_count.len() as u32;

        let most_active_channel = channel_message_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or((0, 0));

        let most_active_member = member_message_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or((String::new(), 0));

        let channel_name = if let Some(name) = self.channel_map.get(&most_active_channel.0) {
            name.to_string()
        } else {
            String::new()
        };

        let overview = OverviewData {
            total_message,
            deleted_message: 0,
            member_count: 0,
            unique_user,
            member_joins: 0,
            member_left: 0,
            most_active_member: most_active_member.0,
            most_active_channel: channel_name,
        };
        self.compare_data = Some(overview);
    }

    pub fn set_date_handler(&mut self, handler: DateHandler) {
        self.date_handler = handler;
        *self.compare_nav.handler().from() = handler.to;
        *self.compare_nav.handler().to() = handler.to;
    }

    fn set_channel_id_map(&mut self, channel_list: Vec<Channel>) {
        for channel in channel_list {
            let channel_id = channel.channel_id;
            let channel_name = &channel.channel_name;
            self.channel_map
                .entry(channel_id)
                .or_insert(channel_name.to_string());
        }
    }
}

impl TabHandler {
    pub fn handle_message_overview(&mut self, message: MessageWithUser, event_bus: &mut EventBus) {
        let guild_id = message.message.guild_id;
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus)
    }

    pub fn set_channel_map(&mut self, guild_id: i64, channels: Vec<Channel>) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .set_channel_id_map(channels)
    }

    pub fn reload_overview(&mut self, guild_id: i64) {
        self.overview.get_mut(&guild_id).unwrap().reload_overview();
    }

    pub fn compare_overview(&mut self, guild_id: i64) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .create_compare_data();
    }

    pub fn stop_compare_overview(&mut self, guild_id: i64) {
        self.overview.get_mut(&guild_id).unwrap().compare_data = None;
    }
}
