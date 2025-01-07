use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};
use core::ops::RangeInclusive;
use eframe::egui::Ui;
use egui_plot::{AxisHints, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use funnel_shared::{Channel, MemberActivity, MemberCount, MessageWithUser, PAGE_VALUE};
use indexmap::IndexMap;
use log::info;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::core::{compare_number, to_header};
use crate::ui::{AnimatedMenuLabel, Card, DateHandler, DateNavigator, ShowUI, TabHandler};
use crate::{AppEvent, ChartType, EventBus};

#[derive(Default)]
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

#[derive(Default, Debug)]
struct MemberChartData {
    hourly: IndexMap<NaiveDateTime, i64>,
    daily: IndexMap<NaiveDateTime, i64>,
    weekly: IndexMap<NaiveDateTime, i64>,
    monthly: IndexMap<NaiveDateTime, i64>,
    last_hour: Option<NaiveDateTime>,
    last_day: Option<NaiveDateTime>,
    last_week: Option<NaiveDateTime>,
    last_month: Option<NaiveDateTime>,
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

pub struct Overview {
    show_count: bool,
    show_joins: bool,
    show_leaves: bool,
    show_full_chart: bool,
    chart_type: ChartType,
    chart_labels: Vec<(NaiveDateTime, i64, i64, i64)>,
    chart_data: HashMap<String, MemberChartData>,
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

impl Default for Overview {
    fn default() -> Self {
        let mut chart_data = HashMap::new();
        chart_data.insert("count".to_string(), MemberChartData::default());
        chart_data.insert("joins".to_string(), MemberChartData::default());
        chart_data.insert("leaves".to_string(), MemberChartData::default());

        Self {
            show_count: true,
            show_joins: true,
            show_leaves: true,
            show_full_chart: Default::default(),
            chart_type: Default::default(),
            chart_labels: Default::default(),
            chart_data,
            activity_data: Default::default(),
            channel_map: Default::default(),
            data: Default::default(),
            compare_data: Default::default(),
            card_size: Default::default(),
            compare_nav: Default::default(),
            compare_size: Default::default(),
            date_handler: Default::default(),
            max_content: Default::default(),
            reload_count: Default::default(),
        }
    }
}

impl ShowUI for Overview {
    fn show_ui(&mut self, ui: &mut Ui, event_bus: &mut EventBus) {
        if !self.show_full_chart {
            self.show_compare_buttons(ui, event_bus);
            self.show_card_ui(ui);
            ui.add_space(10.0);
        }
        self.show_member_chart(ui);
    }
}

impl Overview {
    fn get_target_data_count(&self) -> &IndexMap<NaiveDateTime, i64> {
        match self.chart_type {
            ChartType::Hourly => &self.get_count().hourly,
            ChartType::Daily => &self.get_count().daily,
            ChartType::Weekly => &self.get_count().weekly,
            ChartType::Monthly => &self.get_count().monthly,
        }
    }

    fn get_target_data_joins(&self) -> &IndexMap<NaiveDateTime, i64> {
        match self.chart_type {
            ChartType::Hourly => &self.get_joins().hourly,
            ChartType::Daily => &self.get_joins().daily,
            ChartType::Weekly => &self.get_joins().weekly,
            ChartType::Monthly => &self.get_joins().monthly,
        }
    }
    fn get_target_data_leaves(&self) -> &IndexMap<NaiveDateTime, i64> {
        match self.chart_type {
            ChartType::Hourly => &self.get_leaves().hourly,
            ChartType::Daily => &self.get_leaves().daily,
            ChartType::Weekly => &self.get_leaves().weekly,
            ChartType::Monthly => &self.get_leaves().monthly,
        }
    }

    fn get_target_m(&mut self, count: bool, joins: bool, leaves: bool) -> &mut MemberChartData {
        if count {
            self.get_count_m()
        } else if joins {
            self.get_joins_m()
        } else if leaves {
            self.get_leaves_m()
        } else {
            unreachable!()
        }
    }

    fn get_target(&self, count: bool, joins: bool, leaves: bool) -> &MemberChartData {
        if count {
            self.get_count()
        } else if joins {
            self.get_joins()
        } else if leaves {
            self.get_leaves()
        } else {
            unreachable!()
        }
    }
    fn get_count(&self) -> &MemberChartData {
        self.chart_data.get("count").unwrap()
    }
    fn get_count_m(&mut self) -> &mut MemberChartData {
        self.chart_data.get_mut("count").unwrap()
    }

    fn get_joins(&self) -> &MemberChartData {
        self.chart_data.get("joins").unwrap()
    }

    fn get_joins_m(&mut self) -> &mut MemberChartData {
        self.chart_data.get_mut("joins").unwrap()
    }

    fn get_leaves(&self) -> &MemberChartData {
        self.chart_data.get("leaves").unwrap()
    }

    fn get_leaves_m(&mut self) -> &mut MemberChartData {
        self.chart_data.get_mut("leaves").unwrap()
    }

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

    fn show_member_chart(&mut self, ui: &mut Ui) {
        let hover_position = ui.make_persistent_id("overivew_chart_hover");
        let selected_position = ui.make_persistent_id("overview_chart_selected");

        ui.horizontal(|ui| {
            for val in ChartType::iter() {
                let val_string = val.to_string();
                let selected = self.chart_type == val;

                let resp = ui.add(AnimatedMenuLabel::new(
                    selected,
                    val_string,
                    selected_position,
                    hover_position,
                    55.0,
                    18.0,
                    None,
                    (false, false),
                ));

                if resp.clicked() {
                    self.chart_type = val;
                    self.chart_labels.clear();
                }
            }
            ui.separator();
            if ui
                .selectable_label(self.show_count, "Show Total Member")
                .clicked()
            {
                self.show_count = !self.show_count;
            }

            if ui
                .selectable_label(self.show_joins, "Show Member Joins")
                .clicked()
            {
                self.show_joins = !self.show_joins;
            }

            if ui
                .selectable_label(self.show_leaves, "Show Member Leaves")
                .clicked()
            {
                self.show_leaves = !self.show_leaves;
            }
            ui.separator();

            if ui
                .selectable_label(self.show_full_chart, "Hide Other UI")
                .clicked()
            {
                self.show_full_chart = !self.show_full_chart;
            }
        });
        ui.add_space(5.0);

        let reload_labels = self.chart_labels.is_empty();

        // NOTE: No enumerate because we don't want the index to increase if filtered out

        let mut plot_point_count = None;
        let mut plot_point_joins = None;
        let mut plot_point_leaves = None;

        let mut not_generated_yet = true;

        if self.show_count {
            let mut index = 0.0;
            let points: PlotPoints = self
                .get_target_data_count()
                .clone()
                .into_iter()
                .filter_map(|(date, count)| {
                    if !self.date_handler.within_range(date.date()) {
                        return None;
                    }
                    let x = index;
                    let y = count as f64;
                    if reload_labels {
                        self.chart_labels.push((date, count, 0, 0));
                    }
                    index += 1.0;
                    Some([x, y])
                })
                .collect();
            plot_point_count = Some(points);
            not_generated_yet = false;
        }

        if self.show_joins {
            let mut index = 0.0;
            let points: PlotPoints = self
                .get_target_data_joins()
                .clone()
                .into_iter()
                .filter_map(|(date, count)| {
                    if !self.date_handler.within_range(date.date()) {
                        return None;
                    }
                    let x = index;
                    let y = count as f64;
                    if reload_labels {
                        if not_generated_yet {
                            self.chart_labels.push((date, 0, count, 0));
                        } else if let Some(target_data) = self.chart_labels.get_mut(index as usize)
                        {
                            *target_data = (target_data.0, target_data.1, count, 0)
                        }
                    }
                    index += 1.0;
                    Some([x, y])
                })
                .collect();
            plot_point_joins = Some(points);
            not_generated_yet = false;
        }

        if self.show_leaves {
            let mut index = 0.0;
            let points: PlotPoints = self
                .get_target_data_leaves()
                .clone()
                .into_iter()
                .filter_map(|(date, count)| {
                    if !self.date_handler.within_range(date.date()) {
                        return None;
                    }
                    let x = index;
                    let y = count as f64;
                    if reload_labels {
                        if not_generated_yet {
                            self.chart_labels.push((date, 0, 0, count));
                        } else if let Some(target_data) = self.chart_labels.get_mut(index as usize)
                        {
                            *target_data = (target_data.0, target_data.1, target_data.2, count)
                        }
                    }
                    index += 1.0;
                    Some([x, y])
                })
                .collect();
            plot_point_leaves = Some(points);
        }

        let labels = self.chart_labels.clone();
        let date_axis = move |mark: GridMark, _range: &RangeInclusive<f64>| {
            let index = mark.value.round() as usize;
            if let Some((date, _, _, _)) = labels.get(index) {
                date.format("%y-%m-%d").to_string()
            } else {
                String::new()
            }
        };

        let x_axis = AxisHints::new_x().formatter(date_axis);

        let hover_label = move |_s: &str, val: &PlotPoint| {
            let x_val = val.x.round() as i64;

            if let Some((date, count, joins, leaves)) = self.chart_labels.get(x_val as usize) {
                let date_label = match self.chart_type {
                    ChartType::Hourly => date.to_string(),
                    ChartType::Daily => date.format("%y-%m-%d").to_string(),
                    ChartType::Weekly => {
                        let other_date = date.checked_add_signed(Duration::weeks(1)).unwrap();
                        format!(
                            "{} - {}",
                            date.format("%y-%m-%d"),
                            other_date.format("%y-%m-%d")
                        )
                    }
                    ChartType::Monthly => {
                        let other_date = date.checked_add_months(Months::new(1)).unwrap();
                        format!(
                            "{} - {}",
                            date.format("%y-%m-%d"),
                            other_date.format("%y-%m-%d")
                        )
                    }
                };
                let mut hover_text = format!("{}\nY = {:.0}", date_label, val.y);
                if self.show_count {
                    hover_text += &format!("\nTotal Members = {}", count);
                }
                if self.show_joins {
                    hover_text += &format!("\nJoins = {}", joins);
                }
                if self.show_leaves {
                    hover_text += &format!("\nLeaves = {}", leaves);
                }
                hover_text
            } else {
                format!("X = {:.0}\nY = {:.0}", val.x, val.y)
            }
        };

        Plot::new("member_count")
            .legend(Legend::default().background_alpha(0.0))
            .auto_bounds([true; 2].into())
            .custom_x_axes(vec![x_axis])
            .clamp_grid(true)
            .label_formatter(hover_label)
            .show(ui, |plot_ui| {
                let mut lines = Vec::new();
                if let Some(count) = plot_point_count {
                    let line_count = Line::new(count).name("Total Members");
                    lines.push(line_count);
                }
                if let Some(joins) = plot_point_joins {
                    let line_joins = Line::new(joins).name("Joins");
                    lines.push(line_joins);
                }
                if let Some(leaves) = plot_point_leaves {
                    let line_leaves = Line::new(leaves).name("Leaves");
                    lines.push(line_leaves);
                }
                for line in lines {
                    plot_ui.line(line);
                }
            });
    }

    fn show_card_ui(&mut self, ui: &mut Ui) {
        let total_message_id = ui.make_persistent_id("overview_total_message");
        let unique_user_id = ui.make_persistent_id("overview_unique_user");
        let member_count_id = ui.make_persistent_id("overivew_member_count");

        let compare_total_message = ui.make_persistent_id("overview_compare_message");
        let compare_unique_user = ui.make_persistent_id("overview_compare_user");
        let compare_member_count = ui.make_persistent_id("overview_compare_member_count");

        let space_3_item = ui.make_persistent_id("card_space_3");
        let space_2_item = ui.make_persistent_id("card_space_2");

        ui.vertical(|ui| {
            let has_compare = self.compare_data.is_some();

            if !has_compare {
                ui.ctx().animate_value_with_time(compare_total_message, 0.0, 0.0);
                ui.ctx().animate_value_with_time(compare_unique_user, 0.0, 0.0);
                ui.ctx().animate_value_with_time(compare_member_count, 0.0, 0.0);
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
                ui.ctx().animate_value_with_time(member_count_id, 0.0, 0.0);
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

                let mut header_text = "Member Count".to_string();
                let content_text = ui.ctx().animate_value_with_time(member_count_id, self.data.member_count as f32, 0.5) as u32;
                let mut hover_text =
                    "The final member count at the end of the selected date".to_string();

                if has_compare {
                    let comparing_with = self.compare_data.as_ref().unwrap().member_count;
                    let difference = compare_number(ui, comparing_with, content_text, compare_member_count);
                    header_text += &format!(" {difference}");
                    hover_text += &format!(
                        "\nThe final member count at the end of the selected compare date: {comparing_with}"
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
                )).on_hover_text(hover_text);
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
    }

    fn handle_message(&mut self, message: MessageWithUser, event_bus: &mut EventBus) {
        let username = &message.sender.username;
        let channel_id = message.message.channel_id;
        let guild_id = message.message.guild_id;

        let mut deleted_message = false;

        let timestamp = if let Some(delete_timestamp) = message.message.delete_timestamp {
            deleted_message = true;
            delete_timestamp
        } else {
            message.message.message_timestamp
        };

        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        let local_time = datetime.with_timezone(&Local).naive_local();
        let local_date = local_time.date();
        let activity = ActivityData::new(username.to_string());

        let entry = self.activity_data.entry(local_date).or_default();

        let target_entry = entry.entry(username.to_string()).or_insert(activity);

        if deleted_message {
            target_entry.deleted_message += 1;
        } else {
            let count_entry = target_entry.message_count.entry(channel_id).or_default();
            *count_entry += 1;
        }

        self.reload_count += 1;

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::OverviewNeedsReload(guild_id));
        }
    }

    fn reload_overview(&mut self) {
        self.chart_labels.clear();

        let mut channel_message_count = HashMap::new();
        let mut member_message_count = HashMap::new();
        let mut total_message = 0;
        let mut deleted_message = 0;
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
                    deleted_message += activity.deleted_message;
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

        let to_date = self.date_handler.to;

        let overview = OverviewData {
            total_message,
            deleted_message,
            member_count: self.find_member_count(to_date),
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
            .unwrap_or((String::from("Not Found"), 0));

        let channel_name = if let Some(name) = self.channel_map.get(&most_active_channel.0) {
            name.to_string()
        } else {
            "Not Found".to_string()
        };

        let compare_to_date = self.compare_nav.handler_i().to;

        let overview = OverviewData {
            total_message,
            deleted_message: 0,
            member_count: self.find_member_count(compare_to_date),
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

    fn handle_member_data(&mut self, count: MemberCount) {
        let total_members = count.total_members;
        let timestamp = count.count_timestamp;
        let (hourly_time, daily_time, weekly_time, monthly_time) =
            self.add_missing_date(timestamp, true, false, false);

        self.get_count_m().hourly.insert(hourly_time, total_members);
        self.get_count_m().daily.insert(daily_time, total_members);
        self.get_count_m().weekly.insert(weekly_time, total_members);
        self.get_count_m()
            .monthly
            .insert(monthly_time, total_members);

        // Below or equal to the To date = final member count of the selected period
        if daily_time.date() <= self.date_handler.to {
            self.data.member_count = total_members as u32;
        }
    }

    fn find_member_count(&self, date: NaiveDate) -> u32 {
        if self.get_count().daily.is_empty() {
            return 0;
        }
        let first_date = self.get_count().daily.get_index(0).unwrap();

        let mut ongoing_date = first_date
            .0
            .with_day(date.day())
            .unwrap()
            .with_month(date.month())
            .unwrap()
            .with_year(date.year())
            .unwrap();

        info!("Finding member count for date {ongoing_date}");

        let mut member_count = 0;

        let last_date = self
            .get_count()
            .daily
            .get_index(self.get_count().daily.len() - 1)
            .unwrap();

        // The first data that we have occurred after the date that is selected
        if date < first_date.0.date() {
            return 0;
        }

        // Last date is smaller than current selected date thus that's the final member count
        if last_date.0.date() <= date {
            return *last_date.1 as u32;
        }

        // Go from the date we need the data of to the first date that is available
        while &ongoing_date >= first_date.0 {
            if let Some(count) = self.get_count().daily.get(&ongoing_date) {
                member_count = *count as u32;
                break;
            } else {
                info!("Didn't find on {ongoing_date}");
                ongoing_date = ongoing_date.checked_sub_signed(Duration::days(1)).unwrap();
            }
        }

        info!("Found member count: {member_count}");

        member_count
    }

    fn handle_member_activity(&mut self, activity: MemberActivity) {
        let timestamp = activity.activity_timestamp;
        let is_join = activity.join_activity;

        let (hourly_time, daily_time, weekly_time, monthly_time) =
            self.add_missing_date(timestamp, false, is_join, !is_join);

        if is_join {
            let target_val = self.get_joins_m().hourly.entry(hourly_time).or_default();
            *target_val += 1;

            let target_val = self.get_joins_m().daily.entry(daily_time).or_default();
            *target_val += 1;

            let to_use = *target_val;
            if daily_time.date() <= self.date_handler.to {
                self.data.member_joins = to_use as u32;
            }

            let target_val = self.get_joins_m().weekly.entry(weekly_time).or_default();
            *target_val += 1;

            let target_val = self.get_joins_m().monthly.entry(monthly_time).or_default();
            *target_val += 1;
        } else {
            let target_val = self.get_leaves_m().hourly.entry(hourly_time).or_default();
            *target_val += 1;

            let target_val = self.get_leaves_m().daily.entry(daily_time).or_default();
            *target_val += 1;

            let to_use = *target_val;
            if daily_time.date() <= self.date_handler.to {
                self.data.member_left = to_use as u32;
            }

            let target_val = self.get_leaves_m().weekly.entry(weekly_time).or_default();
            *target_val += 1;

            let target_val = self.get_leaves_m().monthly.entry(monthly_time).or_default();
            *target_val += 1;
        }
    }

    fn add_missing_date(
        &mut self,
        timestamp: i64,
        count: bool,
        joins: bool,
        leaves: bool,
    ) -> (NaiveDateTime, NaiveDateTime, NaiveDateTime, NaiveDateTime) {
        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        let local_time = datetime.with_timezone(&Local).naive_local();

        let hourly_time = local_time.with_minute(0).unwrap().with_second(0).unwrap();
        let daily_time = local_time
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let monthly_time = local_time
            .with_second(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_day(1)
            .unwrap();

        // We only care about the week number for this. Set it as Monday to keep a common ground
        let week_day_name = Weekday::Mon;
        let week_num = local_time.iso_week().week();
        let time_year = local_time.iso_week().year();

        let weekly_time = NaiveDate::from_isoywd_opt(time_year, week_num, week_day_name)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        if let Some(last_hour) = self.get_target(count, joins, leaves).last_hour {
            let missing_hour = (hourly_time - last_hour).num_hours();
            let hourly_member = *self
                .get_target(count, joins, leaves)
                .hourly
                .get(&last_hour)
                .expect("Member count should have existed");

            let mut ongoing_hour = last_hour;
            for _ in 0..missing_hour {
                let to_add = ongoing_hour.checked_add_signed(Duration::hours(1)).unwrap();
                self.get_count_m()
                    .hourly
                    .entry(to_add)
                    .or_insert(hourly_member);
                ongoing_hour = to_add;
            }
        }
        self.get_target_m(count, joins, leaves).last_hour = Some(hourly_time);

        // NOTE: All index maps gets an entry instead of direct insert. For count data where target is
        // that if there are missing day, the missing day will have the data of the lats found
        // data. For joins and leaves, we only want spikes on the day there are relevant data, old
        // data is not persisted. Before any member activities are gotten, the dates found for
        // counts are replicated for join and leave index maps so all entries are ignored unless
        // there are newer joins than the count current date.

        if let Some(last_day) = self.get_target(count, joins, leaves).last_day {
            let missing_day = (daily_time - last_day).num_days();

            let daily_member = *self
                .get_target(count, joins, leaves)
                .daily
                .get(&last_day)
                .expect("Member count should have existed");

            let mut ongoing_day = last_day;
            for _ in 0..missing_day {
                let to_add = ongoing_day.checked_add_signed(Duration::days(1)).unwrap();
                self.get_target_m(count, joins, leaves)
                    .daily
                    .entry(to_add)
                    .or_insert(daily_member);
                ongoing_day = to_add;
            }
        }

        self.get_target_m(count, joins, leaves).last_day = Some(daily_time);

        if let Some(last_week) = self.get_target(count, joins, leaves).last_week {
            let missing_week = (weekly_time - last_week).num_weeks();

            let weekly_member = *self
                .get_target(count, joins, leaves)
                .weekly
                .get(&last_week)
                .expect("Member count should have existed");

            let mut ongoing_week = last_week;
            for _ in 0..missing_week {
                let to_add = ongoing_week.checked_add_signed(Duration::weeks(1)).unwrap();
                self.get_target_m(count, joins, leaves)
                    .weekly
                    .entry(to_add)
                    .or_insert(weekly_member);
                ongoing_week = to_add;
            }
        }

        self.get_target_m(count, joins, leaves).last_week = Some(weekly_time);

        if let Some(last_month) = self.get_target(count, joins, leaves).last_month {
            let mut ongoing_month = last_month;

            let monthly_member = *self
                .get_target(count, joins, leaves)
                .monthly
                .get(&last_month)
                .expect("Member count should have existed");

            while monthly_time > ongoing_month {
                let to_add = ongoing_month
                    .checked_add_signed(Duration::days(45))
                    .unwrap()
                    .with_day(1)
                    .unwrap();
                self.get_target_m(count, joins, leaves)
                    .monthly
                    .entry(to_add)
                    .or_insert(monthly_member);
                ongoing_month = to_add;
            }
        }
        self.get_target_m(count, joins, leaves).last_month = Some(monthly_time);

        (hourly_time, daily_time, weekly_time, monthly_time)
    }

    fn fill_member_activity(&mut self) {
        for data in self.get_count().hourly.clone().into_iter() {
            self.get_joins_m().hourly.insert(data.0, 0);
            self.get_leaves_m().hourly.insert(data.0, 0);
        }

        for data in self.get_count().daily.clone().into_iter() {
            self.get_joins_m().daily.insert(data.0, 0);
            self.get_leaves_m().daily.insert(data.0, 0);
        }

        for data in self.get_count().weekly.clone().into_iter() {
            self.get_joins_m().weekly.insert(data.0, 0);
            self.get_leaves_m().weekly.insert(data.0, 0);
        }

        for data in self.get_count().monthly.clone().into_iter() {
            self.get_joins_m().monthly.insert(data.0, 0);
            self.get_leaves_m().monthly.insert(data.0, 0);
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

    pub fn handle_member_count(&mut self, guild_id: i64, count: MemberCount) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .handle_member_data(count);
    }

    pub fn handle_member_activity(&mut self, guild_id: i64, activity: MemberActivity) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .handle_member_activity(activity);
    }

    pub fn clear_chart_labels(&mut self, guild_id: i64) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .chart_labels
            .clear();
    }

    pub fn fill_member_activity(&mut self, guild_id: i64) {
        self.overview
            .get_mut(&guild_id)
            .unwrap()
            .fill_member_activity();
    }
}
