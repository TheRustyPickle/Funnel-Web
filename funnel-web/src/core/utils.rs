use egui::Ui;

pub fn get_new_x(ui: &mut Ui) -> (f32, f32) {
    let available_size = ui.available_size();
    let x_size = available_size.x;
    let x_size_10 = x_size * 10.0 / 100.0;
    let new_x = x_size - x_size_10;

    (x_size_10, new_x)
}
