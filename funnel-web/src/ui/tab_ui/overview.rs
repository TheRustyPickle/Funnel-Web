use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};
use core::ops::RangeInclusive;
use eframe::egui::Ui;
use egui_plot::{AxisHints, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use funnel_shared::{Channel, MemberCount, MessageWithUser, PAGE_VALUE};
use indexmap::IndexMap;
use log::info;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::core::{compare_number, to_header};
use crate::ui::{AnimatedMenuLabel, Card, DateHandler, DateNavigator, ShowUI, TabHandler};
use crate::{AppEvent, ChartType, EventBus};

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
pub struct MemberJoins {
    show_full_chart: bool,
    chart_type: ChartType,
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

#[derive(Default)]
pub struct Overview {
    chart_labels: Vec<(NaiveDateTime, i64)>,
    member_joins: MemberJoins,
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
        if !self.member_joins.show_full_chart {
            self.show_compare_buttons(ui, event_bus);
            self.show_card_ui(ui);
            ui.add_space(10.0);
        }
        self.show_member_chart(ui);
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

    fn show_member_chart(&mut self, ui: &mut Ui) {
        let hover_position = ui.make_persistent_id("overivew_chart_hover");
        let selected_position = ui.make_persistent_id("overview_chart_selected");

        ui.horizontal(|ui| {
            for val in ChartType::iter() {
                let val_string = val.to_string();
                let selected = self.member_joins.chart_type == val;

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
                    self.member_joins.chart_type = val;
                    self.chart_labels.clear();
                }
            }
            ui.separator();

            if ui
                .selectable_label(self.member_joins.show_full_chart, "Hide Other UI")
                .clicked()
            {
                self.member_joins.show_full_chart = !self.member_joins.show_full_chart;
            }
        });
        ui.add_space(5.0);

        let target_data = match self.member_joins.chart_type {
            ChartType::Hourly => &self.member_joins.hourly,
            ChartType::Daily => &self.member_joins.daily,
            ChartType::Weekly => &self.member_joins.weekly,
            ChartType::Monthly => &self.member_joins.monthly,
        };

        let mut ongoing_x = 0.0;

        let plot_points: PlotPoints = if !self.chart_labels.is_empty() {
            target_data
                .values()
                .map(|count| {
                    let x = ongoing_x;
                    let y = *count as f64;
                    ongoing_x += 1.0;
                    [x, y]
                })
                .collect()
        } else {
            target_data
                .iter()
                .map(|(date, count)| {
                    let x = ongoing_x;
                    let y = *count as f64;
                    self.chart_labels.push((*date, *count));
                    ongoing_x += 1.0;
                    [x, y]
                })
                .collect()
        };

        let labels = self.chart_labels.clone();
        let date_axis = move |mark: GridMark, _range: &RangeInclusive<f64>| {
            let index = mark.value.round() as usize;
            if let Some((date, _)) = labels.get(index) {
                date.format("%y-%m-%d").to_string()
            } else {
                String::new()
            }
        };

        let x_axis = AxisHints::new_x().formatter(date_axis);

        let line = Line::new(plot_points).name("Total Members");

        let hover_label = move |_s: &str, val: &PlotPoint| {
            let x_val = val.x.round() as i64;

            if let Some((date, count)) = self.chart_labels.get(x_val as usize) {
                let date_label = match self.member_joins.chart_type {
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
                format!(
                    "{}\nY = {:.0}\nTotal Members = {}",
                    date_label, val.y, count
                )
            } else {
                format!("X = {:.0}\nY = {:.0}", val.x, val.y)
            }
        };

        Plot::new("member_join")
            .legend(Legend::default().background_alpha(0.0))
            .auto_bounds([true; 2].into())
            .custom_x_axes(vec![x_axis])
            .clamp_grid(true)
            .label_formatter(hover_label)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
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

        // If last day is January 10, current day is January 5, add january 6 to 9 with 0 value
        // Apply the same for all of them
        if let Some(last_hour) = self.member_joins.last_hour {
            let missing_hour = (hourly_time - last_hour).num_hours();
            let hourly_member = *self
                .member_joins
                .hourly
                .get(&last_hour)
                .expect("Member count should have existed");

            let mut ongoing_hour = last_hour;
            for _ in 0..missing_hour {
                let to_add = ongoing_hour.checked_add_signed(Duration::hours(1)).unwrap();
                self.member_joins
                    .hourly
                    .entry(to_add)
                    .or_insert(hourly_member);
                ongoing_hour = to_add;
            }
        }

        self.member_joins.last_hour = Some(hourly_time);

        if let Some(last_day) = self.member_joins.last_day {
            let missing_day = (daily_time - last_day).num_days();

            let daily_member = *self
                .member_joins
                .daily
                .get(&last_day)
                .expect("Member count should have existed");

            let mut ongoing_day = last_day;
            for _ in 0..missing_day {
                let to_add = ongoing_day.checked_add_signed(Duration::days(1)).unwrap();
                self.member_joins
                    .daily
                    .entry(to_add)
                    .or_insert(daily_member);
                ongoing_day = to_add;
            }
        }

        self.member_joins.last_day = Some(daily_time);

        if let Some(last_week) = self.member_joins.last_week {
            let missing_week = (weekly_time - last_week).num_weeks();

            let weekly_member = *self
                .member_joins
                .weekly
                .get(&last_week)
                .expect("Member count should have existed");

            let mut ongoing_week = last_week;
            for _ in 0..missing_week {
                let to_add = ongoing_week.checked_add_signed(Duration::weeks(1)).unwrap();
                self.member_joins
                    .weekly
                    .entry(to_add)
                    .or_insert(weekly_member);
                ongoing_week = to_add;
            }
        }

        self.member_joins.last_week = Some(weekly_time);

        if let Some(last_month) = self.member_joins.last_month {
            let mut ongoing_month = last_month;

            let monthly_member = *self
                .member_joins
                .monthly
                .get(&last_month)
                .expect("Member count should have existed");

            while monthly_time > ongoing_month {
                let to_add = ongoing_month
                    .checked_add_signed(Duration::days(45))
                    .unwrap()
                    .with_day(1)
                    .unwrap();
                self.member_joins
                    .monthly
                    .entry(to_add)
                    .or_insert(monthly_member);
                ongoing_month = to_add;
            }
        }
        self.member_joins.last_month = Some(monthly_time);

        self.member_joins.hourly.insert(hourly_time, total_members);
        self.member_joins.daily.insert(daily_time, total_members);
        self.member_joins.weekly.insert(weekly_time, total_members);
        self.member_joins
            .monthly
            .insert(monthly_time, total_members);

        // Below or equal to the To date = final member count of the selected period
        if daily_time.date() <= self.date_handler.to {
            self.data.member_count = total_members as u32;
        }
    }

    fn find_member_count(&self, date: NaiveDate) -> u32 {
        if self.member_joins.daily.is_empty() {
            return 0;
        }
        let first_date = self.member_joins.daily.get_index(0).unwrap();

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
            .member_joins
            .daily
            .get_index(self.member_joins.daily.len() - 1)
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
            if let Some(count) = self.member_joins.daily.get(&ongoing_date) {
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
}
