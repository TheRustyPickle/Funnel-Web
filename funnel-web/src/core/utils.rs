use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, Id, RichText, Ui};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
use dirs::data_local_dir;

use crate::core::{CHANGE, JET};
use crate::ui::Card;

pub struct ChangeLog {
    pub header: RichText,
    pub normal_text: String,
}

pub struct CardData {
    pub x_size: f32,
    pub y_size: f32,
    pub card_type: CardType,
    pub number: u32,
    pub compare_num: Option<u32>,
    pub id: Id,
    pub compare_id: Option<Id>,
}

impl CardData {
    pub fn add_to_ui(self, ui: &mut Ui, max_content: &mut usize) {
        let Self {
            x_size,
            y_size,
            card_type,
            number,
            compare_num,
            id,
            compare_id,
        } = self;

        let mut header_text = match card_type {
            CardType::TotalMessage => String::from("Total Messages"),
            CardType::UniqueUser => String::from("Unique Users"),
            CardType::DeletedMessage => String::from("Deleted Messages"),
            CardType::MemberCount => String::from("Member Count"),
            CardType::MemberJoin => String::from("Member Joins"),
            CardType::MemberLeave => String::from("Member Leaves"),
        };
        let mut hover_text = match card_type {
            CardType::TotalMessage => {
                format!("Total message gotten within the selected date: {number}")
            }
            CardType::UniqueUser => {
                format!("Total unique users gotten within the selected date: {number}")
            }
            CardType::DeletedMessage => {
                format!("Deleted message gotten within the selected date: {number}")
            }
            CardType::MemberCount => {
                format!("The final member count at the end of the selected date: {number}")
            }
            CardType::MemberJoin => {
                format!("The number of new members within the selected date: {number}")
            }
            CardType::MemberLeave => {
                format!("The number of member leaves within the selected date: {number}")
            }
        };

        let content_text = ui.ctx().animate_value_with_time(id, number as f32, 1.0) as u32;

        if let Some(compare_with) = compare_num {
            let difference = compare_number(ui, compare_with, content_text, compare_id.unwrap());
            header_text += &format!(" {difference}");

            let header_text_len = header_text.chars().count();
            if header_text_len > *max_content {
                *max_content = header_text_len;
            }

            let compare_hover_text = match card_type {
                CardType::TotalMessage => {
                    format!("\nTotal message gotten within the compare date: {compare_with}")
                }
                CardType::DeletedMessage => {
                    format!("\nDeleted message gotten within the compare date: {compare_with}")
                }
                CardType::UniqueUser => {
                    format!("\nUnique members within the compare date: {compare_with}")
                }
                CardType::MemberCount => format!(
                    "\nThe final member count at the end of the compare date: {compare_with}",
                ),
                CardType::MemberJoin => {
                    format!("\nThe number of new members within the compare date: {compare_with}")
                }
                CardType::MemberLeave => {
                    format!(
                        "\nThe number of members leaves within the compare date: {compare_with}"
                    )
                }
            };

            hover_text += &compare_hover_text;
        }

        ui.add(Card::new(
            to_header(header_text),
            to_header(content_text),
            x_size,
            y_size,
        ))
        .on_hover_text(hover_text);
    }
}
pub enum CardType {
    TotalMessage,
    UniqueUser,
    DeletedMessage,
    MemberCount,
    MemberJoin,
    MemberLeave,
}

impl ChangeLog {
    pub fn to_ui(self, ui: &mut Ui) {
        ui.label(self.header);
        ui.separator();
        ui.label(self.normal_text);
    }
}

pub fn to_header(text: impl Display) -> RichText {
    RichText::new(text.to_string()).heading()
}

pub fn to_semi_header(text: impl Display) -> RichText {
    RichText::new(text.to_string()).size(15.0).strong()
}

pub fn add_font(ctx: &Context) {
    let name = "jetbrains";
    let font = JET;
    let font_jet = Arc::new(FontData::from_owned(font.into()));
    let mut font_definitions = FontDefinitions::default();
    font_definitions.font_data.insert(name.to_owned(), font_jet);

    font_definitions
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, name.to_owned());
    font_definitions
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push(name.to_owned());
    ctx.set_fonts(font_definitions);
}

pub fn compare_number(ui: &mut Ui, old_num: u32, new_num: u32, id: Id) -> String {
    let old_num = old_num as f32;
    let new_num = new_num as f32;
    let difference = ((new_num - old_num) / old_num) * 100.0;

    if difference > 0.0 {
        let difference = ui.ctx().animate_value_with_time(id, difference, 1.0);
        format!("{difference:.2}% ↑")
    } else if difference < 0.0 {
        let difference = ui.ctx().animate_value_with_time(id, difference.abs(), 1.0);
        let difference = difference.abs();
        format!("{difference:.2}% ↓")
    } else {
        format!("{difference:.2}%")
    }
}

#[must_use]
pub fn get_change_log() -> Vec<ChangeLog> {
    let full_change_log = String::from_utf8(CHANGE.into()).unwrap();
    let mut split_change_log: Vec<&str> = full_change_log.split('\n').collect();
    split_change_log.remove(0);

    let mut change_logs = Vec::new();

    let mut last_change_log = ChangeLog {
        header: RichText::new(""),
        normal_text: String::new(),
    };

    let mut header_found = false;

    for split in split_change_log {
        if split.is_empty() {
            continue;
        }

        if split.starts_with("##") {
            if header_found {
                change_logs.push(last_change_log);
                last_change_log = ChangeLog {
                    header: RichText::new(""),
                    normal_text: String::new(),
                }
            } else {
                header_found = true;
            }
            let proper_header = split.replace("## ", "");

            last_change_log.header = to_semi_header(proper_header);
        } else {
            let proper_split = split.replace('*', "•");
            last_change_log.normal_text.push_str(proper_split.as_str());
            last_change_log.normal_text.push('\n');
        }
    }
    change_logs.push(last_change_log);

    change_logs
}

#[must_use]
pub fn get_stripped_windows(content: Vec<&str>, window_size: usize) -> Vec<String> {
    let mut valid_windows = Vec::new();

    for window in content.windows(window_size) {
        let mut not_enough_words = false;
        let mut new_words = Vec::new();
        for word in window {
            if word.is_empty() {
                not_enough_words = true;
                break;
            }

            let w = word.trim_end_matches(['.', ',', '?', '!']).to_string();

            if w.is_empty() {
                not_enough_words = true;
                break;
            }

            new_words.push(w);
        }
        if not_enough_words {
            continue;
        }
        let joined_string = new_words.join(" ");
        valid_windows.push(joined_string);
    }
    valid_windows
}

#[derive(Serialize, Deserialize)]
struct StringSession {
    id: String,
}

pub fn save_session(id: String) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("session_id", &id);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(mut path) = get_target_path() {
            path.push("session.json");

            let session = StringSession { id };
            match serde_json::to_string(&session) {
                Ok(json) => match fs::File::create(&path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(json.as_bytes()) {
                            error!("Failed to write to session file {:?}: {}", path, e);
                        }
                    }
                    Err(e) => error!("Failed to create session file {:?}: {}", path, e),
                },
                Err(e) => error!("Failed to serialize session data: {}", e),
            }
        }
    }
}

#[must_use]
pub fn get_session() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("session_id").ok().flatten())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_session_path()?;

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<StringSession>(&content) {
                Ok(session) => Some(session.id),
                Err(e) => {
                    error!("Failed to deserialize session file {:?}: {}", path, e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to read session file {:?}: {}", path, e);
                None
            }
        }
    }
}

pub fn delete_session() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item("session_id");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_session_path();
        if let Some(path) = path {
            if let Err(e) = fs::remove_file(&path) {
                error!("Failed to delete session file {:?}: {}", path, e);
            } else {
                info!("Session file {:?} deleted successfully", path);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_target_path() -> Option<PathBuf> {
    if let Some(mut path) = data_local_dir() {
        path.push("Funnel");
        if let Err(e) = fs::create_dir_all(&path) {
            error!("Failed to create Funnel directory {:?}: {}", path, e);
            return None;
        }
        Some(path)
    } else {
        error!("Failed to determine local data directory.");
        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_session_path() -> Option<PathBuf> {
    if let Some(mut path) = data_local_dir() {
        path.push("Funnel");
        path.push("session.json");
        Some(path)
    } else {
        error!("Failed to determine local data directory.");
        None
    }
}
