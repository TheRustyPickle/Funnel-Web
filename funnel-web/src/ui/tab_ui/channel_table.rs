use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use eframe::egui::ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use eframe::egui::{Align, Layout, Response, RichText, SelectableLabel, Ui};
use egui_extras::Column;
use egui_selectable_table::{
    ColumnOperations, ColumnOrdering, SelectableRow, SelectableTable, SortOrder,
};
use funnel_shared::{Channel, MessageWithUser, PAGE_VALUE};
use std::cmp::Ordering;
use strum::IntoEnumIterator;

use crate::core::ChannelColumn;
use crate::ui::{DateHandler, ShowUI, TabHandler};
use crate::{AppEvent, EventBus};

#[derive(Default)]
pub struct Config {
    copy_selected: bool,
}

impl ColumnOperations<ChannelRowData, ChannelColumn, Config> for ChannelColumn {
    fn column_text(&self, row: &ChannelRowData) -> String {
        match self {
            ChannelColumn::Name => row.name.to_string(),
            ChannelColumn::ID => row.id.to_string(),
            ChannelColumn::TotalMessage => row.total_message.to_string(),
            ChannelColumn::DeletedMessage => row.deleted_message.to_string(),
            ChannelColumn::FirstMessage => row.first_message.to_string(),
            ChannelColumn::LastMessage => row.last_message.to_string(),
            ChannelColumn::UniqueUsers => row.unique_users.len().to_string(),
        }
    }
    fn create_header(
        &self,
        ui: &mut Ui,
        sort_order: Option<SortOrder>,
        _table: &mut SelectableTable<ChannelRowData, ChannelColumn, Config>,
    ) -> Option<Response> {
        let mut label_text = self.to_string();
        let hover_text = match self {
            ChannelColumn::Name => "The name of the channel. Click to sort by name".to_string(),
            ChannelColumn::ID => "The channel ID. Click to sort by user ID".to_string(),
            ChannelColumn::TotalMessage => {
                "Total messages sent to this channel. Click to sort by total message".to_string()
            }
            ChannelColumn::DeletedMessage => {
                "Total deleted messages in this channel. Click to sort by deleted message"
                    .to_string()
            }
            ChannelColumn::FirstMessage => {
                "The date and time the first message seen in this channel".to_string()
            }
            ChannelColumn::LastMessage => {
                "The date and time the last message that was seen in this channel".to_string()
            }
            ChannelColumn::UniqueUsers => {
                "The number of unique users seen in this channel. Click to sort by unique users"
                    .to_string()
            }
        };

        let is_selected = if let Some(direction) = sort_order {
            match direction {
                SortOrder::Ascending => label_text += " ↓",
                SortOrder::Descending => label_text += " ↑",
            }
            true
        } else {
            false
        };

        let label_text = RichText::new(label_text).strong();

        let response = ui
            .add_sized(
                ui.available_size(),
                SelectableLabel::new(is_selected, label_text),
            )
            .on_hover_text(hover_text);
        Some(response)
    }

    fn create_table_row(
        &self,
        ui: &mut Ui,
        row: &SelectableRow<ChannelRowData, ChannelColumn>,
        column_selected: bool,
        table: &mut SelectableTable<ChannelRowData, ChannelColumn, Config>,
    ) -> Response {
        let row_data = &row.row_data;
        let mut show_tooltip = false;
        let row_text = match self {
            ChannelColumn::Name => {
                show_tooltip = true;
                row_data.name.clone()
            }
            ChannelColumn::ID => row_data.id.to_string(),
            ChannelColumn::TotalMessage => row_data.total_message.to_string(),
            ChannelColumn::DeletedMessage => row_data.deleted_message.to_string(),
            ChannelColumn::FirstMessage => row_data.first_message.to_string(),
            ChannelColumn::LastMessage => row_data.last_message.to_string(),
            ChannelColumn::UniqueUsers => row_data.unique_users.len().to_string(),
        };
        let is_selected = column_selected;

        let mut label = ui.add_sized(
            ui.available_size(),
            SelectableLabel::new(is_selected, &row_text),
        );

        if show_tooltip {
            label = label.on_hover_text(row_text);
        };

        label.context_menu(|ui| {
            if ui.button("Copy selected rows").clicked() {
                table.config.copy_selected = true;
                ui.close_menu();
            };
        });
        label
    }
}

impl ColumnOrdering<ChannelRowData> for ChannelColumn {
    fn order_by(&self, row_1: &ChannelRowData, row_2: &ChannelRowData) -> Ordering {
        match self {
            ChannelColumn::Name => row_1.name.cmp(&row_2.name),
            ChannelColumn::ID => row_1.id.cmp(&row_2.id),
            ChannelColumn::TotalMessage => row_1.total_message.cmp(&row_2.total_message),
            ChannelColumn::DeletedMessage => row_1.deleted_message.cmp(&row_2.deleted_message),
            ChannelColumn::FirstMessage => row_1.first_message.cmp(&row_2.first_message),
            ChannelColumn::LastMessage => row_1.last_message.cmp(&row_2.last_message),
            ChannelColumn::UniqueUsers => row_1.unique_users.len().cmp(&row_2.unique_users.len()),
        }
    }
}

#[derive(Clone, Debug)]
struct ChannelRowData {
    name: String,
    id: i64,
    total_message: u32,
    deleted_message: u32,
    first_message: NaiveDateTime,
    last_message: NaiveDateTime,
    unique_users: HashSet<i64>,
}

impl ChannelRowData {
    fn new(name: &str, id: i64, date: NaiveDateTime) -> Self {
        Self {
            name: name.to_string(),
            id,
            total_message: 0,
            deleted_message: 0,
            first_message: date,
            last_message: date,
            unique_users: HashSet::new(),
        }
    }

    fn add_user(&mut self, user_id: i64) {
        self.unique_users.insert(user_id);
    }

    fn extend_user(&mut self, list: &HashSet<i64>) {
        self.unique_users.extend(list);
    }

    /// Increment total message count by 1
    fn increment_total_message(&mut self) {
        self.total_message += 1;
    }

    /// Increment deleted message count by 1
    fn increment_deleted_message(&mut self) {
        self.deleted_message += 1;
    }

    fn increase_deleted_by(&mut self, amount: u32) {
        self.deleted_message += amount;
    }

    /// Increment total message count by `amount`
    fn increase_message_by(&mut self, amount: u32) {
        self.total_message += amount;
    }

    /// Update the date this user was first seen in the chat
    fn set_first_message(&mut self, date: NaiveDateTime) {
        self.first_message = date;
    }

    /// Update the date this user was last seen in the chat
    fn set_last_message(&mut self, date: NaiveDateTime) {
        self.last_message = date;
    }
}

pub struct ChannelTable {
    channel_data: HashMap<NaiveDate, HashMap<i64, ChannelRowData>>,
    table: SelectableTable<ChannelRowData, ChannelColumn, Config>,
    /// Read only currently selected dates in the UI
    date_handler: DateHandler,
    reload_count: u64,
    channel_map: HashMap<i64, String>,
}

impl Default for ChannelTable {
    fn default() -> Self {
        let table = SelectableTable::new(ChannelColumn::iter().collect())
            .auto_scroll()
            .horizontal_scroll()
            .serial_column();
        Self {
            channel_data: HashMap::new(),
            table,
            date_handler: DateHandler::default(),
            reload_count: 0,
            channel_map: HashMap::new(),
        }
    }
}

impl ShowUI for ChannelTable {
    fn show_ui(&mut self, ui: &mut Ui, _guild_id: i64, event_bus: &mut EventBus) {
        let to_copy = self.table.config.copy_selected;
        if to_copy {
            self.table.config.copy_selected = false;
            self.table.copy_selected_cells(ui);
            event_bus.publish(AppEvent::CellsCopied);
        }

        let mut clip_added = false;

        self.table.show_ui(ui, |builder| {
            let mut table = builder
                .striped(true)
                .resizable(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .drag_to_scroll(false)
                .auto_shrink([false; 2])
                .min_scrolled_height(0.0);

            for _ in ChannelColumn::iter() {
                let mut column = Column::initial(100.0);
                if !clip_added {
                    column = column.clip(true);
                    clip_added = true;
                }
                table = table.column(column);
            }
            table
        });
    }
}

impl ChannelTable {
    fn handle_message(&mut self, message: &MessageWithUser, event_bus: &mut EventBus) {
        self.reload_count += 1;

        let user_id = message.sender.user_id;

        let guild_id = message.message.guild_id;
        let channel_id = message.message.channel_id;

        let name = self.channel_map.get(&channel_id).unwrap();

        let mut deleted_message = false;

        let timestamp = if let Some(time) = message.message.delete_timestamp {
            deleted_message = true;
            time
        } else {
            message.message.message_timestamp
        };

        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        let local_time = datetime.with_timezone(&Local).naive_local();
        let local_date = local_time.date();

        let channel_row = ChannelRowData::new(name, channel_id, local_time);

        let entry = self.channel_data.entry(local_date).or_default();
        entry.entry(channel_id).or_insert(channel_row);

        let target_data = self.channel_data.get_mut(&local_date).unwrap();
        let user_row_data = target_data.get_mut(&channel_id).unwrap();

        if user_row_data.first_message > local_time {
            user_row_data.set_first_message(local_time);
        }

        if user_row_data.last_message < local_time {
            user_row_data.set_last_message(local_time);
        }

        // Channel table has a copy of the handler only. Modifying here doesn't impact the UI. Check
        // here if update is necessary in the main UI, if yes, send an event for processing
        let needs_update = self.date_handler.update_dates(local_date);
        if needs_update {
            event_bus.publish(AppEvent::UpdateDate(local_date, guild_id));
        }

        if deleted_message {
            user_row_data.increment_deleted_message();
        } else {
            user_row_data.increment_total_message();
        }

        user_row_data.add_user(user_id);

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::ChannelTableNeedsReload(guild_id));
        }
    }

    /// Create the rows that will be shown in the UI.
    fn create_rows(&mut self) {
        self.reload_count = 0;
        self.table.clear_all_rows();

        let mut id_map = HashMap::new();

        // Go by all the data that are within the range and join them together
        for (date, data) in &self.channel_data {
            if !self.date_handler.within_range(*date) {
                continue;
            }

            for (id, row) in data {
                if let Some(row_id) = id_map.get(id) {
                    self.table.add_modify_row(|rows| {
                        let target_row = rows.get_mut(row_id).unwrap();
                        let user_row_data = &mut target_row.row_data;

                        if user_row_data.first_message > row.first_message {
                            user_row_data.set_first_message(row.first_message);
                        }

                        if user_row_data.last_message < row.last_message {
                            user_row_data.set_last_message(row.last_message);
                        }

                        let total_message = row.total_message;
                        let deleted_message = row.deleted_message;
                        let user_list = &row.unique_users;

                        user_row_data.increase_message_by(total_message);
                        user_row_data.increase_deleted_by(deleted_message);
                        user_row_data.extend_user(user_list);
                        None
                    });
                } else {
                    let new_id = self.table.add_modify_row(|_| Some(row.clone()));
                    id_map.insert(row.id, new_id.unwrap());
                }
            }
        }
        self.table.recreate_rows();
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
    pub fn handle_message_channel_table(
        &mut self,
        message: &MessageWithUser,
        event_bus: &mut EventBus,
    ) {
        let guild_id = message.message.guild_id;
        self.channel_table
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus);
    }

    pub fn channel_table_recreate_rows(&mut self, guild_id: i64) {
        self.channel_table.get_mut(&guild_id).unwrap().create_rows();
    }

    pub fn set_channel_table_channel_map(&mut self, guild_id: i64, channels: Vec<Channel>) {
        self.channel_table
            .get_mut(&guild_id)
            .unwrap()
            .set_channel_id_map(channels);
    }
}
