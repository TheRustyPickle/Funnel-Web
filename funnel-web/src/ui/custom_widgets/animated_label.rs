use eframe::egui::{Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType};
use egui::Id;

pub struct AnimatedLabel {
    text: WidgetText,
    selected: bool,
    horizontal_id: Id,
    vertical_id: Id,
}

impl AnimatedLabel {
    pub fn new(
        selected: bool,
        text: impl Into<WidgetText>,
        horizontal_id: Id,
        vertical_id: Id,
    ) -> Self {
        Self {
            selected,
            text: text.into(),
            horizontal_id,
            vertical_id,
        }
    }
}

impl Widget for AnimatedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            selected,
            text,
            horizontal_id,
            vertical_id,
        } = self;

        // No idea what most of these do, taken from egui selectable label source
        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let max_width = ui.available_width();
        let max_height = ui.spacing().interact_size.y;

        let mut desired_size = total_extra + text.size();

        // Update x and y value of the button size by the animated value
        // For whatever reason, x < 10.0 and y < 2.0 in the starting value freezes everything
        desired_size.x = ui
            .ctx()
            .animate_value_with_time(horizontal_id, max_width, 0.3);
        desired_size.y = ui
            .ctx()
            .animate_value_with_time(vertical_id, max_height, 0.5);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
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
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, selected);

            if selected || response.highlighted() || response.has_focus() || response.hovered() {
                let rect = rect.expand(visuals.expansion);

                ui.painter().rect(
                    rect,
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
