use chrono::{DateTime, Local, NaiveDate};
use eframe::egui::ahash::{HashMap, HashMapExt, HashSet};
use eframe::egui::{Align, CursorIcon, Layout, Response, RichText, SelectableLabel, Slider, Ui};
use egui_extras::Column;
use egui_selectable_table::{
    ColumnOperations, ColumnOrdering, SelectableRow, SelectableTable, SortOrder,
};
use funnel_shared::{Channel, MessageWithUser, PAGE_VALUE};
use std::cmp::Ordering;
use strum::IntoEnumIterator;

use crate::core::WordColumn;
use crate::ui::{DateHandler, ShowUI, TabHandler};
use crate::{get_stripped_windows, AppEvent, EventBus};

#[derive(Default)]
pub struct Config {
    copy_selected: bool,
}

impl ColumnOperations<WordRowData, WordColumn, Config> for WordColumn {
    fn column_text(&self, row: &WordRowData) -> String {
        match self {
            WordColumn::Phrase => row.phrase.to_string(),
            WordColumn::Hits => row.hits.to_string(),
        }
    }

    fn create_header(
        &self,
        ui: &mut Ui,
        sort_order: Option<SortOrder>,
        _table: &mut SelectableTable<WordRowData, WordColumn, Config>,
    ) -> Option<Response> {
        let mut label_text = self.to_string();
        let hover_text = match self {
            WordColumn::Phrase => "The phrase that is being analyzed".to_string(),
            WordColumn::Hits => "The number of times this phrase was found in the chat".to_string(),
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
        row: &SelectableRow<WordRowData, WordColumn>,
        column_selected: bool,
        table: &mut SelectableTable<WordRowData, WordColumn, Config>,
    ) -> Response {
        let row_data = &row.row_data;
        let mut show_tooltip = false;
        let row_text = match self {
            WordColumn::Phrase => {
                show_tooltip = true;
                row_data.phrase.to_string()
            }
            WordColumn::Hits => row_data.hits.to_string(),
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

impl ColumnOrdering<WordRowData> for WordColumn {
    fn order_by(&self, row_1: &WordRowData, row_2: &WordRowData) -> Ordering {
        match self {
            WordColumn::Phrase => row_1.phrase.cmp(&row_2.phrase),
            WordColumn::Hits => row_1.hits.cmp(&row_2.hits),
        }
    }
}

#[derive(Clone, Debug)]
struct WordRowData {
    phrase: String,
    hits: u32,
}

impl WordRowData {
    fn new(phrase: String) -> Self {
        Self { phrase, hits: 0 }
    }

    fn increase_hits(&mut self) {
        self.hits += 1;
    }
}

pub struct WordTable {
    table: SelectableTable<WordRowData, WordColumn, Config>,
    date_handler: DateHandler,
    reload_count: u64,
    window_size: usize,
    stripped_contents: HashMap<NaiveDate, HashMap<i64, Vec<String>>>,

    channels: Vec<Channel>,
    selected_channels: HashSet<usize>,
}

impl Default for WordTable {
    fn default() -> Self {
        let table = SelectableTable::new(WordColumn::iter().collect())
            .auto_scroll()
            .select_full_row()
            .horizontal_scroll()
            .serial_column();
        Self {
            table,
            date_handler: DateHandler::default(),
            reload_count: 0,
            window_size: 1,
            stripped_contents: HashMap::new(),
            channels: Vec::default(),
            selected_channels: HashSet::default(),
        }
    }
}

impl ShowUI for WordTable {
    fn show_ui(&mut self, ui: &mut Ui, guild_id: i64, event_bus: &mut EventBus) {
        let to_copy = self.table.config.copy_selected;
        if to_copy {
            self.table.config.copy_selected = false;
            self.table.copy_selected_cells(ui);
            event_bus.publish(AppEvent::CellsCopied);
        }

        ui.horizontal(|ui| {
            ui.label("Phrase Size:");
            if ui.add(Slider::new(&mut self.window_size, 1..=20)).changed() {
                event_bus.publish(AppEvent::WordTableNeedsReload(guild_id));

            }
            ui.separator();
            ui.label("Where is word xyz?")
                .on_hover_text("Message contents are filtered out of words such as 'I' 'This' 'My' and many more to keep the count relevant to help find useful phrases")
                .on_hover_cursor(CursorIcon::Help)
        });
        ui.add_space(5.0);

        self.table.show_ui(ui, |builder| {
            let table = builder
                .striped(true)
                .resizable(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .drag_to_scroll(false)
                .column(Column::initial(500.0).clip(true))
                .column(Column::initial(150.0))
                .auto_shrink([false; 2])
                .min_scrolled_height(0.0);

            table
        });
    }
}

impl WordTable {
    fn handle_message(&mut self, message: &MessageWithUser, event_bus: &mut EventBus) {
        if message.message.delete_timestamp.is_some() || message.message.stripped_content.is_none()
        {
            return;
        }
        self.reload_count += 1;

        let guild_id = message.message.guild_id;
        let channel_id = message.message.channel_id;

        let timestamp = message.message.message_timestamp;

        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
        let local_time = datetime.with_timezone(&Local).naive_local();
        let local_date = local_time.date();
        let stripped_content = message.message.stripped_content.clone().unwrap();

        self.stripped_contents
            .entry(local_date)
            .or_default()
            .entry(channel_id)
            .or_default()
            .push(stripped_content.clone());

        if self.reload_count >= PAGE_VALUE * 5 {
            event_bus.publish_if_needed(AppEvent::WordTableNeedsReload(guild_id));
        }
    }

    fn create_rows(&mut self) {
        self.reload_count = 0;
        self.table.clear_all_rows();

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

        let mut row_map: HashMap<String, WordRowData> = HashMap::new();
        for (date, stripped_content) in &self.stripped_contents {
            if !self.date_handler.within_range(*date) {
                continue;
            }

            for (channel_id, content_list) in stripped_content {
                if !selected_channels.contains(channel_id) {
                    continue;
                }

                for content in content_list {
                    let split_stripped_content: Vec<&str> =
                        content.split(' ').filter(|s| !s.is_empty()).collect();
                    if split_stripped_content.len() < self.window_size {
                        continue;
                    }

                    let stripped_windows =
                        get_stripped_windows(split_stripped_content, self.window_size);

                    for phrase in stripped_windows {
                        let word_row = WordRowData::new(phrase.clone());
                        let entry = row_map.entry(phrase.clone()).or_insert(word_row);
                        entry.increase_hits();
                    }
                }
            }
        }

        for row in row_map.values() {
            self.table.add_modify_row(|_| Some(row.clone()));
        }

        self.table.recreate_rows();
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
    pub fn handle_message_word_table(
        &mut self,
        message: &MessageWithUser,
        event_bus: &mut EventBus,
    ) {
        let guild_id = message.message.guild_id;
        self.word_table
            .get_mut(&guild_id)
            .unwrap()
            .handle_message(message, event_bus);
    }

    pub fn word_table_recreate_rows(&mut self, guild_id: i64) {
        self.word_table.get_mut(&guild_id).unwrap().create_rows();
    }
}
