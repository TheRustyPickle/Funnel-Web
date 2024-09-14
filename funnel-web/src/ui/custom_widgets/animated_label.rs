use eframe::egui::{Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType};
use egui::Id;

pub struct AnimatedLabel {
    text: WidgetText,
    selected: bool,
    vertical_id_top: Id,
    vertical_id_bottom: Id,
    horizontal_size: f32,
}

impl AnimatedLabel {
    pub fn new(
        selected: bool,
        text: impl Into<WidgetText>,
        vertical_id_top: Id,
        vertical_id_bottom: Id,
        horizontal_size: f32,
    ) -> Self {
        Self {
            selected,
            text: text.into(),
            vertical_id_top,
            vertical_id_bottom,
            horizontal_size,
        }
    }
}

impl Widget for AnimatedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            selected,
            text,
            vertical_id_top,
            vertical_id_bottom,
            horizontal_size,
        } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let text = ui.painter().layout_no_wrap(
            text.text().to_string(),
            TextStyle::Button.resolve(ui.style()),
            ui.visuals().text_color(),
        );

        let mut desired_size = total_extra + text.size();

        desired_size.x = horizontal_size;

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        let target_y = rect.top() + (rect.height() - text.size().y) / 2.0;

        let y_top = ui
            .ctx()
            .animate_value_with_time(vertical_id_top, target_y, 0.5);

        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                text.text(),
            )
        });

        if ui.is_rect_visible(response.rect) {
            let mut text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;
            text_pos.y = y_top;

            let visuals = ui.style().interact_selectable(&response, selected);

            let mut background_rect = rect.expand(visuals.expansion);

            if selected {
                let y_bottom = ui
                    .ctx()
                    .animate_value_with_time(vertical_id_bottom, target_y, 0.5);
                background_rect.min.y = y_bottom - button_padding.y;
                background_rect.max.y = y_bottom + text.size().y + button_padding.y;
                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            } else if response.highlighted() || response.has_focus() || response.hovered() {
                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }

            ui.painter().galley(text_pos, text, visuals.text_color());
        }

        response
    }
}
