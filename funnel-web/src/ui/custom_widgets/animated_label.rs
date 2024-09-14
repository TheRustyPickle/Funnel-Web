use eframe::egui::{Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType};
use egui::Id;

pub struct AnimatedLabel {
    text: WidgetText,
    selected: bool,
    vertical_id_top: Id,
    vertical_id_bottom: Id,
    hover_position: Id,
}

impl AnimatedLabel {
    pub fn new(
        selected: bool,
        text: impl Into<WidgetText>,
        vertical_id_top: Id,
        vertical_id_bottom: Id,
        hover_position: Id,
    ) -> Self {
        Self {
            selected,
            text: text.into(),
            vertical_id_top,
            vertical_id_bottom,
            hover_position,
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
            hover_position,
        } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let text_galley = ui.painter().layout_no_wrap(
            text.text().to_string(),
            TextStyle::Button.resolve(ui.style()),
            ui.visuals().text_color(),
        );

        let mut desired_size = total_extra + text_galley.size();

        desired_size.x = ui.available_width();

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        let target_y = rect.top() + (rect.height() - text_galley.size().y) / 2.0;

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
                .align_size_within_rect(text_galley.size(), rect.shrink2(button_padding))
                .min;

            let text_x = ui.make_persistent_id("text_pos");

            // y value will remain as it is but if the right bar is resized, make the animation
            // smoother by modifying x value
            let val = ui.ctx().animate_value_with_time(text_x, text_pos.x, 0.5);
            text_pos.y = y_top;
            text_pos.x = val;

            let visuals = ui.style().interact_selectable(&response, selected);

            let mut background_rect = rect.expand(visuals.expansion);

            let mut blend_factor = 0.0;

            if selected {
                let y_bottom = ui
                    .ctx()
                    .animate_value_with_time(vertical_id_bottom, target_y, 0.5);
                background_rect.min.y = y_bottom - button_padding.y;
                background_rect.max.y = y_bottom + text_galley.size().y + button_padding.y;

                // Render background for the selected state
                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }

            if response.highlighted() || response.has_focus() || response.hovered() {
                let y = ui
                    .ctx()
                    .animate_value_with_time(hover_position, target_y, 0.5);

                // Only apply blend factor if the hovered item is also selected
                if selected {
                    blend_factor = ui.ctx().animate_value_with_time(
                        ui.make_persistent_id("blend_factor"),
                        1.0,
                        0.5,
                    );
                }

                let blended_y = if selected {
                    blend_factor * y + (1.0 - blend_factor) * target_y
                } else {
                    y
                };

                background_rect.min.y = blended_y - button_padding.y;
                background_rect.max.y = blended_y + text_galley.size().y + button_padding.y;

                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }
            ui.painter()
                .galley(text_pos, text_galley, visuals.text_color());
        }

        response
    }
}
