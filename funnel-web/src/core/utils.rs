use egui::{RichText, Ui};
use std::fmt::Display;

pub fn get_new_x(ui: &mut Ui, percentage: f32) -> (f32, f32) {
    let available_size = ui.available_size();
    let x_size = available_size.x;
    let new_x_size = x_size * percentage / 100.0;
    let full_new_x = x_size - new_x_size;

    (new_x_size, full_new_x)
}

pub fn to_header(text: impl Display) -> RichText {
    RichText::new(text.to_string()).heading()
}
