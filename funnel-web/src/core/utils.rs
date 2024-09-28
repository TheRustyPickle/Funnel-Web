use eframe::egui::RichText;
use std::fmt::Display;

pub fn to_header(text: impl Display) -> RichText {
    RichText::new(text.to_string()).heading()
}
