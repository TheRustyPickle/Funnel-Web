use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};
use core::ops::RangeInclusive;
use eframe::egui::ahash::{HashMap, HashMapExt, HashSet};
use eframe::egui::Ui;
use egui_plot::{AxisHints, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use funnel_shared::{MessageWithUser, PAGE_VALUE};
use indexmap::IndexMap;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

use crate::ui::{AnimatedMenuLabel, DateHandler, ShowUI, TabHandler};
use crate::{AppEvent, ChartType, EventBus};

pub struct UserChart {
    chart_type: ChartType,
    chart_data: BTreeMap<String, IndexMap<NaiveDateTime, i64>>,
    chart_values: HashMap<String, bool>,
    chart_labels: Vec<Vec<(String, String)>>,
    /// Timestamp key with value HashMap with key channel id and value as data point
    hourly_data: BTreeMap<NaiveDateTime, HashMap<i64, HashSet<i64>>>,
    daily_data: BTreeMap<NaiveDateTime, HashMap<i64, HashSet<i64>>>,
    weekly_data: BTreeMap<NaiveDateTime, HashMap<i64, HashSet<i64>>>,
    monthly_data: BTreeMap<NaiveDateTime, HashMap<i64, HashSet<i64>>>,
    last_hour: Option<NaiveDateTime>,
    last_day: Option<NaiveDateTime>,
    last_week: Option<NaiveDateTime>,
    last_month: Option<NaiveDateTime>,
    date_handler: DateHandler,
    reload_count: u64,
}

impl Default for UserChart {
    fn default() -> Self {
        let chart_type = ChartType::Daily;
        let mut chart_data = BTreeMap::new();

        chart_data.insert("Active Users".to_string(), IndexMap::default());

        Self {
            chart_type,
            chart_data,
            chart_values: HashMap::default(),
            chart_labels: Vec::new(),
            hourly_data: BTreeMap::default(),
            daily_data: BTreeMap::new(),
            weekly_data: BTreeMap::new(),
            monthly_data: BTreeMap::new(),
            last_hour: None,
            last_day: None,
            last_week: None,
            last_month: None,
            date_handler: Default::default(),
            reload_count: 0,
        }
    }
}

impl ShowUI for UserChart {
    fn show_ui(&mut self, ui: &mut Ui, guild_id: i64, event_bus: &mut EventBus) {
        let hover_position = ui.make_persistent_id("user_chart_chart_hover");
        let selected_position = ui.make_persistent_id("user_chart_chart_selected");

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
                    event_bus.publish(AppEvent::UserChartTypeChanged(guild_id));
                }
            }
        });
        ui.add_space(5.0);

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
        Plot::new("user_chart")
            .legend(Legend::default().background_alpha(0.0))
            .auto_bounds([true; 2].into())
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

impl UserChart {
    fn get_target_data(&mut self) -> &BTreeMap<NaiveDateTime, HashMap<i64, HashSet<i64>>> {
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
        let user_id = message.sender.user_id;

        let timestamp = if let Some(d) = message.message.delete_timestamp {
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

        target_entry.insert(user_id);

        let daily_entry = self.daily_data.entry(daily_time).or_default();
        let target_entry = daily_entry.entry(channel_id).or_default();

        target_entry.insert(user_id);

        let weekly_entry = self.weekly_data.entry(weekly_time).or_default();
        let target_entry = weekly_entry.entry(channel_id).or_default();

        target_entry.insert(user_id);

        let monthly_entry = self.monthly_data.entry(monthly_time).or_default();
        let target_entry = monthly_entry.entry(channel_id).or_default();

        target_entry.insert(user_id);

        self.chart_values.entry(username).or_default();

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::MessageChartNeedsReload(guild_id));
        }
    }

    fn reload_chart(&mut self) {
        self.reload_count = 0;
        self.chart_labels.clear();

        let target_values: HashSet<String> = self.chart_data.keys().cloned().collect();
        let mut final_data: BTreeMap<String, IndexMap<NaiveDateTime, i64>> = BTreeMap::new();
        let chart_data = self.get_target_data();

        let do_active_users = target_values.contains("Active Users");

        for (date, data) in chart_data {
            let mut total_users: i64 = 0;
            let mut other_users: HashMap<String, i64> = HashMap::new();

            for (_channel, user_ids) in data {
                // TODO: Filter out channels here

                total_users += user_ids.len() as i64;

                let data: HashMap<String, i64> = user_ids
                    .iter()
                    .filter(|v| target_values.contains(&v.to_string()))
                    .map(|num| (num.to_string(), 1))
                    .collect();

                other_users.extend(data);
            }
            if do_active_users {
                final_data
                    .entry("Active Users".to_string())
                    .or_default()
                    .insert(*date, total_users);
            }

            for (user, count) in other_users {
                final_data.entry(user).or_default().insert(*date, count);
            }
        }
        self.chart_data = final_data;
    }

    pub fn set_date_handler(&mut self, handler: DateHandler) {
        self.date_handler = handler;
    }
}

impl TabHandler {
    pub fn handle_message_user_chart(
        &mut self,
        message: &MessageWithUser,
        event_bus: &mut EventBus,
    ) {
        let guild_id = message.message.guild_id;
        self.user_chart
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus)
    }

    pub fn reload_user_chart(&mut self, guild_id: i64) {
        self.user_chart.get_mut(&guild_id).unwrap().reload_chart();
    }
}
