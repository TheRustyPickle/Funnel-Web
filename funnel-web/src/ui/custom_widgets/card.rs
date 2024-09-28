use eframe::egui::{Align2, Response, Sense, TextStyle, Ui, Vec2, Widget, WidgetText};

pub struct Card {
    header: WidgetText,
    content: WidgetText,
    x_size: f32,
    y_size: f32,
}

impl Card {
    pub fn new(
        header: impl Into<WidgetText>,
        content: impl Into<WidgetText>,
        x_size: f32,
        y_size: f32,
    ) -> Self {
        Self {
            header: header.into(),
            content: content.into(),
            x_size,
            y_size,
        }
    }
}

impl Widget for Card {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            header,
            content,
            x_size,
            y_size,
        } = self;
        let card_padding = Vec2::new(10.0, 10.0);
        let header_padding = Vec2::new(5.0, 5.0);

        let total_size = Vec2::new(x_size, y_size);

        let (rect, response) = ui.allocate_exact_size(total_size, Sense::click());

        let visuals = ui.style().noninteractive();

        let header_galley = ui.painter().layout_no_wrap(
            header.text().to_string(),
            TextStyle::Heading.resolve(ui.style()),
            visuals.text_color(),
        );

        let content_galley = ui.painter().layout_no_wrap(
            content.text().to_string(),
            TextStyle::Heading.resolve(ui.style()),
            visuals.text_color(),
        );

        let rounding = 10.0;
        ui.painter()
            .rect(rect, rounding, visuals.weak_bg_fill, visuals.bg_stroke);

        let header_pos = Align2::CENTER_TOP
            .align_size_within_rect(header_galley.size(), rect.shrink2(header_padding))
            .min;

        let content_pos = Align2::CENTER_TOP
            .align_size_within_rect(content_galley.size(), rect.shrink2(card_padding))
            .min
            + Vec2::new(0.0, header_galley.size().y + header_padding.y);

        ui.painter()
            .galley(header_pos, header_galley.clone(), visuals.text_color());

        ui.painter()
            .galley(content_pos, content_galley, visuals.text_color());

        response
    }
}
