use eframe::egui::{
    Id, LayerId, Order, Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText, WidgetType,
};

pub struct AnimatedLabel {
    text: WidgetText,
    selected: bool,
    text_position: Id,
    selection_position: Id,
    hover_position: Id,
}

impl AnimatedLabel {
    pub fn new(
        selected: bool,
        text: impl Into<WidgetText>,
        text_position: Id,
        selection_position: Id,
        hover_position: Id,
    ) -> Self {
        Self {
            selected,
            text: text.into(),
            text_position,
            selection_position,
            hover_position,
        }
    }
}

impl Widget for AnimatedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            selected,
            text,
            text_position,
            selection_position,
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

        // When a new guild is selected, everything starts from y 0.0. Animate going from 0.0 to
        // the appropriate position
        let text_position = ui
            .ctx()
            .animate_value_with_time(text_position, target_y, 0.5);

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
            text_pos.y = text_position;
            text_pos.x = val;

            // Color of the widget. Blue if selected, otherwise transparent grayish color
            let visuals = ui.style().interact_selectable(&response, selected);

            // The rect that is the shown when either hovering/selected
            let mut background_rect = rect.expand(visuals.expansion);

            if selected {
                // Make the blue colored rect animated from previous position to the current
                // selected one.
                let y_bottom = ui
                    .ctx()
                    .animate_value_with_time(selection_position, target_y, 0.5);
                background_rect.min.y = y_bottom - button_padding.y;
                background_rect.max.y = y_bottom + text_galley.size().y + button_padding.y;

                ui.painter().rect(
                    background_rect,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                );
            }

            if response.highlighted() || response.has_focus() || response.hovered() {
                // Make the transparent colored rect animated from previous position to the current
                // hovering one.
                let y = ui
                    .ctx()
                    .animate_value_with_time(hover_position, target_y, 0.5);

                background_rect.min.y = y - button_padding.y;
                background_rect.max.y = y + text_galley.size().y + button_padding.y;

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
                .with_layer_id(LayerId::new(Order::Background, Id::new("text_layer")))
                .galley(text_pos, text_galley, visuals.text_color());
        }

        response
    }
}
