use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};
use eframe::egui::ahash::{HashMap, HashMapExt, HashSet};
use eframe::egui::{CentralPanel, Id, Modal, ScrollArea, TopBottomPanel, Ui};
use egui_plot::{AxisHints, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use funnel_shared::{Channel, MessageWithUser, PAGE_VALUE};
use indexmap::IndexMap;
use strum::IntoEnumIterator;

use crate::ui::{AnimatedMenuLabel, DateHandler, ShowUI, TabHandler};
use crate::{AppEvent, ChartType, EventBus};

#[derive(Default)]
struct ChartPointData {
    user: String,
    count: u32,
    deleted: bool,
}

pub struct MessageChart {
    chart_type: ChartType,
    chart_data: BTreeMap<String, IndexMap<NaiveDateTime, i64>>,
    chart_values: BTreeMap<String, bool>,
    chart_labels: Vec<Vec<(String, String)>>,
    hourly_data: BTreeMap<NaiveDateTime, HashMap<i64, Vec<ChartPointData>>>,
    daily_data: BTreeMap<NaiveDateTime, HashMap<i64, Vec<ChartPointData>>>,
    weekly_data: BTreeMap<NaiveDateTime, HashMap<i64, Vec<ChartPointData>>>,
    monthly_data: BTreeMap<NaiveDateTime, HashMap<i64, Vec<ChartPointData>>>,
    last_hour: Option<NaiveDateTime>,
    last_day: Option<NaiveDateTime>,
    last_week: Option<NaiveDateTime>,
    last_month: Option<NaiveDateTime>,
    date_handler: DateHandler,
    reload_count: u64,
    open_modal: bool,
    channels: Vec<Channel>,
    selected_channels: HashSet<usize>,
}

impl Default for MessageChart {
    fn default() -> Self {
        let chart_type = ChartType::Daily;
        let mut chart_data = BTreeMap::new();
        let mut chart_values = BTreeMap::new();

        chart_data.insert("All Messages".to_string(), IndexMap::default());
        chart_data.insert("Deleted Messages".to_string(), IndexMap::default());

        chart_values.insert("All Messages".to_string(), true);
        chart_values.insert("Deleted Messages".to_string(), true);

        Self {
            chart_type,
            chart_data,
            chart_values,
            chart_labels: Vec::new(),
            hourly_data: BTreeMap::default(),
            daily_data: BTreeMap::new(),
            weekly_data: BTreeMap::new(),
            monthly_data: BTreeMap::new(),
            last_hour: None,
            last_day: None,
            last_week: None,
            last_month: None,
            date_handler: DateHandler::default(),
            reload_count: 0,
            open_modal: false,
            channels: Vec::default(),
            selected_channels: HashSet::default(),
        }
    }
}

impl ShowUI for MessageChart {
    fn show_ui(&mut self, ui: &mut Ui, guild_id: i64, event_bus: &mut EventBus) {
        let hover_position = ui.make_persistent_id("message_chart_chart_hover");
        let selected_position = ui.make_persistent_id("message_chart_chart_selected");

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

                if resp.clicked() && self.chart_type != val {
                    self.chart_type = val;
                    event_bus.publish(AppEvent::MessageChartTypeChanged(guild_id));
                }
            }
            ui.separator();

            if ui.button("Customize View").clicked() {
                self.open_modal = true;
            }
        });

        ui.add_space(5.0);

        if self.open_modal {
            self.show_popup(ui);
        }

        let start_datetime = self.date_handler.from.and_hms_opt(0, 0, 0).unwrap();
        let reload_labels = self.chart_labels.is_empty();
        let total_label_values = self.chart_data.len();

        let mut generated_labels = false;

        let mut all_lines = Vec::new();
        for (val_index, (val, data)) in self.chart_data.iter().enumerate() {
            let start_index = data.get_index_of(&start_datetime).unwrap_or(0);
            let mut index = 0.0;

            let points: PlotPoints = data
                .clone()
                .into_iter()
                .skip(start_index)
                .take_while(|(date, _)| date.date() <= self.date_handler.to)
                .filter_map(|(date, count)| {
                    if !self.date_handler.within_range(date.date()) {
                        return None;
                    }
                    let y = count as f64;
                    let x = index;

                    if reload_labels {
                        if generated_labels {
                            if let Some(target_data) = self.chart_labels.get_mut(index as usize) {
                                target_data[val_index + 1].0 = val.to_string();
                                target_data[val_index + 1].1 = count.to_string();
                            }
                        } else {
                            let mut to_insert = Vec::new();

                            to_insert.push((date.to_string(), String::new()));

                            for _ in 0..total_label_values {
                                to_insert.push((String::new(), String::new()));
                            }

                            to_insert[val_index + 1].0 = val.to_string();
                            to_insert[val_index + 1].1 = count.to_string();
                            self.chart_labels.push(to_insert);
                        }
                    }

                    index += 1.0;
                    Some([x, y])
                })
                .collect();
            let line = Line::new(points).name(val.to_string());

            all_lines.push(line);
            generated_labels = true;
        }

        let labels = self.chart_labels.clone();
        let date_axis = move |mark: GridMark, _range: &RangeInclusive<f64>| {
            let index = mark.value.round() as usize;
            if let Some(data) = labels.get(index) {
                let date_string = &data[0].0;

                let date = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S").unwrap();

                date.format("%y-%m-%d").to_string()
            } else {
                String::new()
            }
        };

        let x_axis = AxisHints::new_x().formatter(date_axis);

        let hover_label = move |_s: &str, val: &PlotPoint| {
            let x_val = val.x.round() as i64;

            if let Some(hover_data) = self.chart_labels.get(x_val as usize) {
                let date_string = &hover_data[0].0;

                let date = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S").unwrap();

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
                for data in hover_data.iter().skip(1) {
                    hover_text.push_str(&format!("\n{}: {}", data.0, data.1));
                }

                hover_text
            } else {
                format!("X = {:.0}\nY = {:.0}", val.x, val.y)
            }
        };
        Plot::new("message_chart")
            .legend(Legend::default().background_alpha(0.0))
            .auto_bounds([true; 2])
            .custom_x_axes(vec![x_axis])
            .clamp_grid(true)
            .label_formatter(hover_label)
            .show(ui, |plot_ui| {
                for line in all_lines {
                    plot_ui.line(line);
                }
            });
    }
}

impl MessageChart {
    fn show_popup(&mut self, ui: &mut Ui) {
        let response = Modal::new(Id::new("customize_view")).show(ui.ctx(), |ui| {
            ui.set_width(300.0);
            ui.set_height(300.0);
            TopBottomPanel::top("customize_top_view").show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Customize View");
                });
            });

            TopBottomPanel::bottom(Id::new("customize_bottom_view")).show_inside(ui, |ui| {
                ui.add_space(5.0);
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Confirm").clicked() {
                        self.open_modal = false;
                    }
                })
            });

            CentralPanel::default().show_inside(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let all_keys = self.chart_values.keys().cloned().collect::<Vec<_>>();
                    for val in all_keys {
                        ui.horizontal(|ui| {
                            ui.checkbox(self.chart_values.get_mut(&val).unwrap(), val);
                            ui.allocate_space(ui.available_size());
                        });
                    }
                });
            });
        });

        if response.should_close() {
            self.open_modal = false;
        }

        if !self.open_modal {
            for (key, val) in self.chart_values.clone() {
                if val {
                    self.chart_data.entry(key).or_default();
                } else {
                    self.chart_data.remove(&key);
                }
            }

            self.reload_chart();
        }
    }

    fn get_target_data(&mut self) -> &BTreeMap<NaiveDateTime, HashMap<i64, Vec<ChartPointData>>> {
        match self.chart_type {
            ChartType::Hourly => &mut self.hourly_data,
            ChartType::Daily => &mut self.daily_data,
            ChartType::Weekly => &mut self.weekly_data,
            ChartType::Monthly => &mut self.monthly_data,
        }
    }
    fn handle_message(&mut self, message: &MessageWithUser, event_bus: &mut EventBus) {
        self.reload_count += 1;

        let guild_id = message.message.guild_id;
        let channel_id = message.message.channel_id;
        let username = message.sender.username.to_string();
        let mut deleted = false;

        let timestamp = if let Some(d) = message.message.delete_timestamp {
            deleted = true;
            d
        } else {
            message.message.message_timestamp
        };

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

        if let Some(last_hour) = self.last_hour {
            let missing_hour = (hourly_time - last_hour).num_hours();
            let mut ongoing_hour = last_hour;

            for _ in 0..missing_hour {
                let to_add = ongoing_hour.checked_add_signed(Duration::hours(1)).unwrap();
                self.hourly_data.entry(to_add).or_default();
                ongoing_hour = to_add;
            }
        }
        self.last_hour = Some(hourly_time);

        if let Some(last_day) = self.last_day {
            let missing_day = (daily_time - last_day).num_days();

            let mut ongoing_day = last_day;
            for _ in 0..missing_day {
                let to_add = ongoing_day.checked_add_signed(Duration::days(1)).unwrap();
                self.daily_data.entry(to_add).or_default();
                ongoing_day = to_add;
            }
        }

        self.last_day = Some(daily_time);

        if let Some(last_week) = self.last_week {
            let missing_week = (weekly_time - last_week).num_weeks();

            let mut ongoing_week = last_week;
            for _ in 0..missing_week {
                let to_add = ongoing_week.checked_add_signed(Duration::weeks(1)).unwrap();
                self.weekly_data.entry(to_add).or_default();
                ongoing_week = to_add;
            }
        }

        self.last_week = Some(weekly_time);

        if let Some(last_month) = self.last_month {
            let mut ongoing_month = last_month;

            while monthly_time > ongoing_month {
                let to_add = ongoing_month
                    .checked_add_months(Months::new(1))
                    .unwrap()
                    .with_day(1)
                    .unwrap();
                self.monthly_data.entry(to_add).or_default();
                ongoing_month = to_add;
            }
        }

        self.last_month = Some(monthly_time);

        let hourly_entry = self.hourly_data.entry(hourly_time).or_default();
        let target_entry = hourly_entry.entry(channel_id).or_default();

        if deleted {
            target_entry.push(ChartPointData {
                user: username.clone(),
                count: 1,
                deleted: true,
            });
        } else {
            let mut not_found = true;
            for point in target_entry.iter_mut() {
                if point.user == username && !point.deleted {
                    point.count += 1;
                    not_found = false;
                    break;
                }
            }

            if not_found {
                target_entry.push(ChartPointData {
                    user: username.clone(),
                    count: 1,
                    deleted: false,
                });
            }
        }

        let daily_entry = self.daily_data.entry(daily_time).or_default();
        let target_entry = daily_entry.entry(channel_id).or_default();

        if deleted {
            target_entry.push(ChartPointData {
                user: username.clone(),
                count: 1,
                deleted: true,
            });
        } else {
            let mut not_found = true;
            for point in target_entry.iter_mut() {
                if point.user == username && !point.deleted {
                    point.count += 1;
                    not_found = false;
                    break;
                }
            }

            if not_found {
                target_entry.push(ChartPointData {
                    user: username.clone(),
                    count: 1,
                    deleted: false,
                });
            }
        }

        let weekly_entry = self.weekly_data.entry(weekly_time).or_default();
        let target_entry = weekly_entry.entry(channel_id).or_default();

        if deleted {
            target_entry.push(ChartPointData {
                user: username.clone(),
                count: 1,
                deleted: true,
            });
        } else {
            let mut not_found = true;
            for point in target_entry.iter_mut() {
                if point.user == username && !point.deleted {
                    point.count += 1;
                    not_found = false;
                    break;
                }
            }

            if not_found {
                target_entry.push(ChartPointData {
                    user: username.clone(),
                    count: 1,
                    deleted: false,
                });
            }
        }

        let monthly_entry = self.monthly_data.entry(monthly_time).or_default();
        let target_entry = monthly_entry.entry(channel_id).or_default();

        if deleted {
            target_entry.push(ChartPointData {
                user: username.clone(),
                count: 1,
                deleted: true,
            });
        } else {
            let mut not_found = true;
            for point in target_entry.iter_mut() {
                if point.user == username && !point.deleted {
                    point.count += 1;
                    not_found = false;
                    break;
                }
            }

            if not_found {
                target_entry.push(ChartPointData {
                    user: username.clone(),
                    count: 1,
                    deleted: false,
                });
            }
        }

        self.chart_values.entry(username).or_default();

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::MessageChartNeedsReload(guild_id));
        }
    }

    fn reload_chart(&mut self) {
        self.reload_count = 0;
        self.chart_labels.clear();

        let mut selected_channels = HashSet::default();

        if self.selected_channels.is_empty() {
            for channel in &self.channels {
                selected_channels.insert(channel.channel_id);
            }
        } else {
            for index in &self.selected_channels {
                if index == &0_usize {
                    for channel in &self.channels {
                        selected_channels.insert(channel.channel_id);
                    }
                    break;
                } else {
                    let channel_id = self.channels.get(*index - 1).unwrap().channel_id;
                    selected_channels.insert(channel_id);
                }
            }
        }

        let target_values: HashSet<String> = self.chart_data.keys().cloned().collect();
        let mut final_data: BTreeMap<String, IndexMap<NaiveDateTime, i64>> = BTreeMap::new();
        let chart_data = self.get_target_data();

        let do_total_message = target_values.contains("All Messages");
        let do_deleted_message = target_values.contains("Deleted Messages");

        for (date, data) in chart_data {
            let mut total_message: i64 = 0;
            let mut deleted_message: i64 = 0;
            let mut other_messages: HashMap<String, u32> = HashMap::new();

            for val in &target_values {
                if val == "All Messages" || val == "Deleted Messages" {
                    continue;
                }
                other_messages.insert(val.to_string(), 0);
            }

            for (channel, points) in data {
                if selected_channels.contains(channel) {
                    for point in points {
                        if point.deleted && do_deleted_message {
                            deleted_message += i64::from(point.count);
                        } else if !point.deleted && do_total_message {
                            total_message += i64::from(point.count);
                        }

                        if target_values.contains(&point.user) {
                            *other_messages.get_mut(&point.user).unwrap() += point.count;
                        }
                    }
                }
            }
            if do_deleted_message {
                final_data
                    .entry("Deleted Messages".to_string())
                    .or_default()
                    .insert(*date, deleted_message);
            }

            if do_total_message {
                final_data
                    .entry("All Messages".to_string())
                    .or_default()
                    .insert(*date, total_message);
            }

            for (user, count) in other_messages {
                let entry = final_data.entry(user).or_default();
                entry.insert(*date, i64::from(count));
            }
        }
        self.chart_data = final_data;
    }

    pub fn set_date_handler(&mut self, handler: DateHandler) {
        self.date_handler = handler;
    }

    pub fn set_channels(&mut self, channels: Vec<Channel>) {
        self.channels = channels;
    }
    pub fn set_selected_channels(&mut self, selected: HashSet<usize>) {
        self.selected_channels = selected;
    }
}

impl TabHandler {
    pub fn handle_message_message_chart(
        &mut self,
        message: &MessageWithUser,
        event_bus: &mut EventBus,
    ) {
        let guild_id = message.message.guild_id;
        self.message_chart
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus);
    }

    pub fn reload_message_chart(&mut self, guild_id: i64) {
        self.message_chart
            .get_mut(&guild_id)
            .unwrap()
            .reload_chart();
    }
}
