use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, Id, RichText, Ui};
use std::fmt::Display;
use std::sync::Arc;

use crate::core::{CHANGE, JET};

pub struct ChangeLog {
    pub header: RichText,
    pub normal_text: String,
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
    let font_caska = Arc::new(FontData::from_owned(font.into()));
    let mut font_definitions = FontDefinitions::default();
    font_definitions
        .font_data
        .insert(name.to_owned(), font_caska);

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
        format!("{:.2}% ↑", difference)
    } else if difference < 0.0 {
        let difference = ui.ctx().animate_value_with_time(id, difference.abs(), 1.0);
        let difference = difference.abs();
        format!("{:.2}% ↓", difference)
    } else {
        format!("{:.2}%", difference)
    }
}

pub fn get_change_log() -> Vec<ChangeLog> {
    let full_change_log = String::from_utf8(CHANGE.into()).unwrap();
    let mut split_change_log: Vec<&str> = full_change_log.split("\n").collect();
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
            let proper_split = split.replace("*", "•");
            last_change_log.normal_text.push_str(proper_split.as_str());
            last_change_log.normal_text.push('\n');
        }
    }
    change_logs.push(last_change_log);

    change_logs
}
