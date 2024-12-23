use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, RichText};
use std::fmt::Display;
use std::sync::Arc;

use crate::core::JET;

pub fn to_header(text: impl Display) -> RichText {
    RichText::new(text.to_string()).heading()
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

pub fn compare_number(old_num: u32, new_num: u32) -> String {
    let old_num = old_num as f32;
    let new_num = new_num as f32;
    let difference = ((new_num - old_num) / old_num) * 100.0;
    if difference > 0.0 {
        format!("{:.2}% ↑", difference)
    } else {
        let difference = difference.abs();
        format!("{:.2}% ↓", difference)
    }
}
