use chrono::{DateTime, NaiveDate};
use eframe::egui::{TopBottomPanel, Ui};
use funnel_shared::{Channel, MessageWithUser};
use std::collections::HashMap;

use crate::core::to_header;
use crate::ui::{Card, DateHandler, DateNavigator, ShowUI, TabHandler};
use crate::EventBus;

#[derive(Default, Debug)]
pub struct ActivityData {
    // Channel ID + Messagse Count
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
}

impl ShowUI for Overview {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        self.show_bottom_bar(ui, event_bus);

        let space_3_item = ui.make_persistent_id("card_space_3");
        let space_2_item = ui.make_persistent_id("card_space_2");
        ui.vertical(|ui| {
            // TODO: Modify X size based on the largest text?
            let x_size = 250.0;
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
            }

            ui.horizontal(|ui| {
                ui.add_space(space_3);
                let remaining_width = ui.available_width();
                ui.add(Card::new(
                    to_header("Total Message"),
                    to_header(self.data.total_message),
                    x_size,
                    y_size,
                ));
                let space_taken = remaining_width - ui.available_width();
                self.card_size = space_taken;

                ui.add(Card::new(
                    to_header("Deleted Message"),
                    to_header(self.data.deleted_message),
                    x_size,
                    y_size,
                ));

                ui.add(Card::new(
                    to_header("Unique User"),
                    to_header(self.data.unique_user),
                    x_size,
                    y_size,
                ));
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
                ui.add_space(space_2);
                ui.add(Card::new(
                    to_header("Most Active Member"),
                    to_header(&self.data.most_active_member),
                    x_size,
                    y_size,
                ));

                ui.add(Card::new(
                    to_header("Most Active Channel"),
                    to_header(&self.data.most_active_channel),
                    x_size,
                    y_size,
                ));
            })
        });
        ui.add_space(50.0);
    }
}
// TODO: Allow comparing these numbers with a different date, show up or down %
impl Overview {
    fn show_bottom_bar(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        TopBottomPanel::bottom("bottom_panel_comparison")
            .show_separator_line(true)
            .show_animated_inside(ui, true, |ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let spacing_size = ui.available_width() - self.compare_size;
                    if spacing_size > 0.0 {
                        ui.add_space(spacing_size / 2.0);
                    };
                    let max_width = ui.available_width();
                    self.compare_nav.show_ui_compare(ui, event_bus);
                    let consumed = max_width - ui.available_width();
                    self.compare_size = consumed;
                });
            });
    }

    fn handle_message(&mut self, message: MessageWithUser) {
        let username = &message.sender.username;
        let channel_id = message.message.channel_id;

        let timestamp = &message.message.message_timestamp;

        let datetime = DateTime::from_timestamp(*timestamp, 0).unwrap();
        let local_time = datetime.naive_local();
        let local_date = local_time.date();
        let activity = ActivityData::new(username.to_string());

        let entry = self.activity_data.entry(local_date).or_default();

        let target_entry = entry.entry(username.to_string()).or_insert(activity);

        let count_entry = target_entry.message_count.entry(channel_id).or_default();
        *count_entry += 1;
    }

    fn reload_overview(&mut self) {
        let mut channel_message_count = HashMap::new();
        let mut member_message_count = HashMap::new();
        let mut total_message = 0;

        // Filter `activity_data` in a single iterator chain.
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

    pub fn set_date_handler(&mut self, handler: DateHandler) {
        self.date_handler = handler;
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
    pub fn handle_message_overview(&mut self, message: MessageWithUser) {
        let guild_id = message.message.guild_id;
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message)
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
}
