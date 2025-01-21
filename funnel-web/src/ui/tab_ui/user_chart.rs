use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};
use core::ops::RangeInclusive;
use eframe::egui::ahash::{HashMap, HashMapExt};
use eframe::egui::Ui;
use egui_plot::{AxisHints, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use funnel_shared::{Channel, MemberActivity, MemberCount, MessageWithUser, PAGE_VALUE};
use indexmap::IndexMap;
use log::info;
use strum::IntoEnumIterator;

use crate::core::to_header;
use crate::ui::{AnimatedMenuLabel, Card, DateHandler, DateNavigator, ShowUI, TabHandler};
use crate::{AppEvent, CardData, CardType, ChartType, EventBus};

pub struct UserChart {}

impl ShowUI for UserChart {
    fn show_ui(&mut self, ui: &mut Ui, guild_id: i64, event_bus: &mut EventBus) {}
}
