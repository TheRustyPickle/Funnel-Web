use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use eframe::egui::ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use eframe::egui::{Align, Layout, Response, RichText, SelectableLabel, Ui};
use egui_extras::Column;
use egui_selectable_table::{
    ColumnOperations, ColumnOrdering, SelectableRow, SelectableTable, SortOrder,
};
use funnel_shared::{MessageWithUser, PAGE_VALUE};
use std::cmp::Ordering;
use strum::IntoEnumIterator;

use crate::core::UserColumn;
use crate::ui::{DateHandler, ShowUI, TabHandler};
use crate::{AppEvent, EventBus};

#[derive(Default)]
pub struct Config {
    copy_selected: bool,
}

impl ColumnOperations<UserRowData, UserColumn, Config> for UserColumn {
    fn column_text(&self, row: &UserRowData) -> String {
        match self {
            UserColumn::Name => row.name.to_string(),
            UserColumn::Username => row.username.to_string(),
            UserColumn::UserID => row.id.to_string(),
            UserColumn::TotalMessage => row.total_message.to_string(),
            UserColumn::DeletedMessage => row.deleted_message.to_string(),
            UserColumn::TotalWord => row.total_word.to_string(),
            UserColumn::TotalChar => row.total_char.to_string(),
            UserColumn::AverageWord => row.average_word.to_string(),
            UserColumn::AverageChar => row.average_char.to_string(),
            UserColumn::FirstMessageSeen => row.first_seen.to_string(),
            UserColumn::LastMessageSeen => row.last_seen.to_string(),
            UserColumn::UniqueChannels => row.unique_channels.len().to_string(),
        }
    }
    fn create_header(
        &self,
        ui: &mut Ui,
        sort_order: Option<SortOrder>,
        _table: &mut SelectableTable<UserRowData, UserColumn, Config>,
    ) -> Option<Response> {
        let mut label_text = self.to_string();
        let hover_text = match self {
            UserColumn::Name => "Discord name of the user. Click to sort by name".to_string(),
            UserColumn::Username => {
                "Discord username of the user. Click to sort by username".to_string()
            }
            UserColumn::UserID => {
                "Discord User ID of the user. Click to sort by user ID".to_string()
            }
            UserColumn::TotalMessage => {
                "Total messages sent by the user. Click to sort by total message".to_string()
            }
            UserColumn::DeletedMessage => {
                "Total deleted message by the user. Click to sort by deleted message".to_string()
            }
            UserColumn::TotalWord => {
                "Total words in the messages. Click to sort by total words".to_string()
            }
            UserColumn::TotalChar => {
                "Total character in the messages. Click to sort by total character".to_string()
            }
            UserColumn::AverageWord => {
                "Average number of words per message. Click to sort by average words".to_string()
            }
            UserColumn::AverageChar => {
                "Average number of characters per message. Click to sort by average characters"
                    .to_string()
            }

            UserColumn::FirstMessageSeen => {
                "The date and time the first message that was sent by this user was observed"
                    .to_string()
            }
            UserColumn::LastMessageSeen => {
                "The date and time the last message that was sent by this user was observed"
                    .to_string()
            }
            UserColumn::UniqueChannels => {
                "The number of unique channels this user was seen in. Click to sort by unique channels"
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
        row: &SelectableRow<UserRowData, UserColumn>,
        column_selected: bool,
        table: &mut SelectableTable<UserRowData, UserColumn, Config>,
    ) -> Response {
        let row_data = &row.row_data;
        let mut show_tooltip = false;
        let row_text = match self {
            UserColumn::Name => {
                show_tooltip = true;
                row_data.name.clone()
            }
            UserColumn::Username => {
                show_tooltip = true;
                row_data.username.clone()
            }
            UserColumn::UserID => row_data.id.to_string(),
            UserColumn::TotalMessage => row_data.total_message.to_string(),
            UserColumn::DeletedMessage => row_data.deleted_message.to_string(),
            UserColumn::TotalWord => row_data.total_word.to_string(),
            UserColumn::TotalChar => row_data.total_char.to_string(),
            UserColumn::AverageWord => row_data.average_word.to_string(),
            UserColumn::AverageChar => row_data.average_char.to_string(),
            UserColumn::FirstMessageSeen => row_data.first_seen.to_string(),
            UserColumn::LastMessageSeen => row_data.last_seen.to_string(),
            UserColumn::UniqueChannels => row_data.unique_channels.len().to_string(),
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

impl ColumnOrdering<UserRowData> for UserColumn {
    fn order_by(&self, row_1: &UserRowData, row_2: &UserRowData) -> Ordering {
        match self {
            UserColumn::Name => row_1.name.cmp(&row_2.name),
            UserColumn::Username => row_1.username.cmp(&row_2.username),
            UserColumn::UserID => row_1.id.cmp(&row_2.id),
            UserColumn::TotalMessage => row_1.total_message.cmp(&row_2.total_message),
            UserColumn::DeletedMessage => row_1.deleted_message.cmp(&row_2.deleted_message),
            UserColumn::TotalWord => row_1.total_word.cmp(&row_2.total_word),
            UserColumn::TotalChar => row_1.total_char.cmp(&row_2.total_char),
            UserColumn::AverageWord => row_1.average_word.cmp(&row_2.average_word),
            UserColumn::AverageChar => row_1.average_char.cmp(&row_2.average_char),
            UserColumn::FirstMessageSeen => row_1.first_seen.cmp(&row_2.first_seen),
            UserColumn::LastMessageSeen => row_1.last_seen.cmp(&row_2.last_seen),
            UserColumn::UniqueChannels => row_1
                .unique_channels
                .len()
                .cmp(&row_2.unique_channels.len()),
        }
    }
}

#[derive(Clone)]
struct UserRowData {
    name: String,
    username: String,
    id: i64,
    total_message: u32,
    deleted_message: u32,
    total_word: u32,
    total_char: u32,
    average_word: u32,
    average_char: u32,
    first_seen: NaiveDateTime,
    last_seen: NaiveDateTime,
    unique_channels: HashSet<i64>,
}

impl UserRowData {
    fn new(name: &str, username: &str, id: i64, date: NaiveDateTime) -> Self {
        let username = username.to_string();

        UserRowData {
            name: name.to_string(),
            username,
            id,
            total_message: 0,
            deleted_message: 0,
            total_word: 0,
            total_char: 0,
            average_word: 0,
            average_char: 0,
            first_seen: date,
            last_seen: date,
            unique_channels: HashSet::new(),
        }
    }

    fn add_channel(&mut self, channel_id: i64) {
        self.unique_channels.insert(channel_id);
    }

    fn extend_channels(&mut self, list: &HashSet<i64>) {
        self.unique_channels.extend(list);
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

    /// Increment total word count by `word_num`
    fn increment_total_word(&mut self, word_num: u32) {
        self.total_word += word_num;
        self.average_word = self.total_word / self.total_message;
    }

    /// Increment total char count by `char_num`
    fn increment_total_char(&mut self, char_num: u32) {
        self.total_char += char_num;
        self.average_char = self.total_char / self.total_message;
    }

    /// Update the date this user was first seen in the chat
    fn set_first_seen(&mut self, date: NaiveDateTime) {
        self.first_seen = date;
    }

    /// Update the date this user was last seen in the chat
    fn set_last_seen(&mut self, date: NaiveDateTime) {
        self.last_seen = date;
    }
}

pub struct UserTable {
    /// Key: The Date where at least one message/User was found
    /// Value: A HashMap of the founded User with their user id as the key
    /// Contains all data points and UI points are recreated from here
    user_data: HashMap<NaiveDate, HashMap<i64, UserRowData>>,
    table: SelectableTable<UserRowData, UserColumn, Config>,
    /// Read only currently selected dates in the UI
    date_handler: DateHandler,
    total_message: u32,
    deleted_message: u32,
    reload_count: u64,
}

impl Default for UserTable {
    fn default() -> Self {
        let table = SelectableTable::new(UserColumn::iter().collect())
            .auto_scroll()
            .horizontal_scroll()
            .serial_column();
        Self {
            user_data: HashMap::new(),
            table,
            date_handler: DateHandler::default(),
            total_message: 0,
            deleted_message: 0,
            reload_count: 0,
        }
    }
}

impl ShowUI for UserTable {
    fn show_ui(&mut self, ui: &mut Ui, _guild_id: i64, event_bus: &mut EventBus) {
        let to_copy = self.table.config.copy_selected;
        if to_copy {
            self.table.config.copy_selected = false;
            self.table.copy_selected_cells(ui);
            event_bus.publish(AppEvent::CellsCopied);
        }

        let mut clip_added = 0;

        ui.horizontal(|ui| {
            ui.label(format!("Total Users: {}", self.get_total_user()));
            ui.separator();
            ui.label(format!("Total Message: {}", self.total_message));
            ui.separator();
            ui.label(format!("Total deleted message: {}", self.deleted_message));
        });
        ui.separator();
        ui.add_space(5.0);

        self.table.show_ui(ui, |builder| {
            let mut table = builder
                .striped(true)
                .resizable(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .drag_to_scroll(false)
                .auto_shrink([false; 2])
                .min_scrolled_height(0.0);

            for _ in UserColumn::iter() {
                let mut column = Column::initial(100.0);
                if clip_added < 2 {
                    column = column.clip(true);
                    clip_added += 1;
                }
                table = table.column(column);
            }
            table
        });
    }
}

impl UserTable {
    fn handle_message(&mut self, message: &MessageWithUser, event_bus: &mut EventBus) {
        self.reload_count += 1;
        let username = &message.sender.username;
        let global_name = if let Some(name) = &message.sender.global_name {
            name
        } else {
            username
        };
        let user_id = message.sender.user_id;
        let guild_id = message.message.guild_id;
        let channel_id = message.message.channel_id;

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

        let user_row = UserRowData::new(global_name, username, user_id, local_time);

        let entry = self.user_data.entry(local_date).or_default();
        entry.entry(user_id).or_insert(user_row);

        let target_data = self.user_data.get_mut(&local_date).unwrap();
        let user_row_data = target_data.get_mut(&user_id).unwrap();

        // Update last and first seen in this date for this user
        if user_row_data.first_seen > local_time {
            user_row_data.set_first_seen(local_time);
        }

        if user_row_data.last_seen < local_time {
            user_row_data.set_last_seen(local_time);
        }

        // User table has a copy of the handler only. Modifying here doesn't impact the UI. Check
        // here if update is necessary in the main UI, if yes, send an event for processing
        let needs_update = self.date_handler.update_dates(local_date);
        if needs_update {
            event_bus.publish(AppEvent::UpdateDate(local_date, guild_id));
        }

        if !deleted_message {
            let message_text = message.message.message_content.clone().unwrap_or_default();

            let total_char = message_text.len() as u32;
            let total_word = message_text.split_whitespace().count() as u32;

            user_row_data.increment_total_message();
            user_row_data.increment_total_word(total_word);
            user_row_data.increment_total_char(total_char);
        } else {
            user_row_data.increment_deleted_message();
        }

        user_row_data.add_channel(channel_id);

        if self.reload_count == PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::UserTableNeedsReload(guild_id));
        }
    }

    fn get_total_user(&self) -> usize {
        self.table.total_rows()
    }

    /// Create the rows that will be shown in the UI.
    fn create_rows(&mut self) {
        self.reload_count = 0;
        self.table.clear_all_rows();
        let mut total_message = 0;
        let mut deleted_message = 0;
        let mut id_map = HashMap::new();

        // Go by all the data that are within the range and join them together
        for (date, data) in &self.user_data {
            if !self.date_handler.within_range(*date) {
                continue;
            }

            for (id, row) in data {
                total_message += row.total_message;
                deleted_message += row.deleted_message;

                if let Some(row_id) = id_map.get(id) {
                    self.table.add_modify_row(|rows| {
                        let target_row = rows.get_mut(row_id).unwrap();
                        let user_row_data = &mut target_row.row_data;

                        if user_row_data.first_seen > row.first_seen {
                            user_row_data.set_first_seen(row.first_seen);
                        }

                        if user_row_data.last_seen < row.last_seen {
                            user_row_data.set_last_seen(row.last_seen);
                        }

                        let total_char = row.total_char;
                        let total_word = row.total_word;
                        let total_message = row.total_message;
                        let deleted_message = row.deleted_message;
                        let channel_list = &row.unique_channels;

                        user_row_data.increase_message_by(total_message);
                        user_row_data.increase_deleted_by(deleted_message);
                        user_row_data.increment_total_word(total_word);
                        user_row_data.increment_total_char(total_char);
                        user_row_data.extend_channels(channel_list);
                        None
                    });
                } else {
                    let new_id = self.table.add_modify_row(|_| Some(row.clone()));
                    id_map.insert(row.id, new_id.unwrap());
                }
            }
        }
        self.total_message = total_message;
        self.deleted_message = deleted_message;
        self.table.recreate_rows();
    }

    pub fn set_date_handler(&mut self, handler: DateHandler) {
        self.date_handler = handler;
    }
}

impl TabHandler {
    pub fn user_table_recreate_rows(&mut self, guild_id: i64) {
        self.user_table.get_mut(&guild_id).unwrap().create_rows();
    }

    pub fn handle_message_user_table(
        &mut self,
        message: &MessageWithUser,
        event_bus: &mut EventBus,
    ) {
        let guild_id = message.message.guild_id;
        self.user_table
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus)
    }
}
