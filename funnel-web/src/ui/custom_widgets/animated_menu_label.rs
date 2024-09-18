use egui::{Id, LayerId, Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType};
use egui::Order::Foreground;

pub struct AnimatedMenuLabel {
    text: WidgetText,
    selected: bool,
    selected_position: Id,
    hover_position: Id,
    x_size: f32,
    y_size: f32,
    separator_position: (bool, bool),
}

impl AnimatedMenuLabel {
    pub fn new(
        selected: bool,
        text: impl Into<WidgetText>,
        selected_position: Id,
        hover_position: Id,
        x_size: f32,
        y_size: f32,
        separator_position: (bool, bool),
    ) -> Self {
        Self {
            selected,
            text: text.into(),
            selected_position,
            hover_position,
            x_size,
            y_size,
            separator_position,
        }
    }
}

impl Widget for AnimatedMenuLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            selected,
            text,
            selected_position,
            hover_position,
            x_size,
            y_size,
            separator_position,
        } = self;

        // Whether to add a separator to the left or to the right side.
        // For now left side is only used for the tab's first value
        let (separator_left, separator_right) = separator_position;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let text_galley = ui.painter().layout_no_wrap(
            text.text().to_string(),
            TextStyle::Button.resolve(ui.style()),
            ui.visuals().text_color(),
        );

        // Force the given size so the selected/hovering rect does not have to resize each time
        let mut desired_size = total_extra + text_galley.size();
        desired_size.y = y_size;
        desired_size.x = x_size;

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // The y values for the separator
        let separator_y_1 = rect.min.y - 10.0;
        let separator_y_2 = rect.max.y + 10.0;

        if separator_left {
            // We draw the separator manually as ui.separator() creates a new rect that cause
            // stuttering in the UI
            // For whatever reason 4.0 is the magic number that maintains correct x distance between
            // the widgets. Or at least close to that.
            let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            ui.painter()
                .vline(rect.min.x - 4.0, separator_y_1..=separator_y_2, stroke);
        }
        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                text.text(),
            )
        });

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(text_galley.size(), rect.shrink2(button_padding))
                .min;

            let target_x = rect.left() + (rect.width() - text_galley.size().x) / 2.0;

            // Color of the widget. Blue if selected, otherwise transparent grayish color
            let visuals = ui.style().interact_selectable(&response, selected);

            // The rect that is the shown when either hovering/selected
            let mut background_rect = rect.expand(visuals.expansion);

            // Enforce the y size by checking difference and adjust accordingly
            // For whatever reason when selected y value is different
            let y_difference = background_rect.max.y - background_rect.min.y;

            let remaining = y_size - y_difference;

            // If no separator, don't take the entire available space
            if !separator_left && !separator_right {
                background_rect.min.y -= remaining / 2.0;
                background_rect.max.y += remaining / 2.0;
            } else {
                // Hard coded value, not sure what caused them to be different in my case.
                // Determined by manually testing
                background_rect.min.y = separator_y_1 + 5.0;
                background_rect.max.y = separator_y_2 - 4.0;
            }

            if selected {
                let x_selected = ui
                    .ctx()
                    .animate_value_with_time(selected_position, target_x, 0.5);
                background_rect.min.x = x_selected - button_padding.x;
                background_rect.max.x = x_selected + text_galley.size().x + button_padding.x;

                // Enfoce x value in the widget so each of them is the same size
                let rect_difference = background_rect.max.x - background_rect.min.x;
                let remaining = x_size - rect_difference;

                background_rect.min.x -= remaining / 2.0;
                background_rect.max.x += remaining / 2.0;

                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }

            if response.highlighted() || response.has_focus() || response.hovered() {
                let x_hover = ui
                    .ctx()
                    .animate_value_with_time(hover_position, target_x, 0.5);

                // Enfoce x value in the widget so each of them is the same size
                background_rect.min.x = x_hover - button_padding.x;
                background_rect.max.x = x_hover + text_galley.size().x + button_padding.x;

                let rect_difference = background_rect.max.x - background_rect.min.x;
                let remaining = x_size - rect_difference;

                background_rect.min.x -= remaining / 2.0;
                background_rect.max.x += remaining / 2.0;

                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }

            // Add the text. Prevent the text from being drawn in the background.
            ui.painter()
                .clone()
                .with_layer_id(LayerId::new(Foreground, Id::new("text_layer")))
                .galley(text_pos, text_galley, visuals.text_color());

            if separator_right {
                let fixed_line_x = rect.max.x + 4.0;
                let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                ui.painter()
                    .vline(fixed_line_x, separator_y_1..=separator_y_2, stroke);
            }
        }

        response
    }
}
