use egui::{
    global_dark_light_mode_switch, menu, Align, CentralPanel, Context, Image, Layout, ScrollArea,
    SidePanel, Spinner, TopBottomPanel,
};

use crate::initializer::MainWindow;

pub struct PanelStatus {
    show_guild: bool,
    show_channel: bool,
}

impl PanelStatus {
    fn show_upper_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("upper_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                global_dark_light_mode_switch(ui);
                ui.separator();
                if ui
                    .selectable_label(self.show_guild, "Show Guild List")
                    .on_hover_text("Show/Hide Guild List")
                    .clicked()
                {
                    self.show_guild = !self.show_guild;
                };
                ui.separator();
                if ui
                    .selectable_label(self.show_channel, "Show Channel List")
                    .on_hover_text("Show/Hide Channel List")
                    .clicked()
                {
                    self.show_channel = !self.show_channel;
                };
            });
            ui.add_space(0.5);
        });
    }
}

impl Default for PanelStatus {
    fn default() -> Self {
        Self {
            show_guild: true,
            show_channel: true,
        }
    }
}

impl MainWindow {
    pub fn show_panels(&mut self, ctx: &Context) {
        self.panels.show_upper_bar(ctx);

        SidePanel::left("left_panel").max_width(70.0).resizable(false).show_animated(ctx, self.panels.show_guild, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label("Guild List");
                    ui.separator();
                    ui.add_space(10.0);
                    ui.add(Image::from_uri("https://cdn.discordapp.com/icons/1146449209206247528/07f3d615871d1be76db5a4b99c32e257.webp")
                        .fit_to_exact_size(egui::Vec2 { x: 40.0, y: 40.0 }).rounding(50.0));
                    ui.add_space(10.0);
                    ui.add(Image::from_uri("https://cdn.discordapp.com/icons/273534239310479360/0d87eff44967cfd5aafaabdb04f0159e.webp")
                        .fit_to_exact_size(egui::Vec2 { x: 40.0, y: 40.0 }).rounding(50.0));
                    ui.add_space(10.0);
                    ui.add(Image::from_uri("https://cdn.discordapp.com/icons/896132176091955261/3f328f7f7f6a8ac3fe87466eaf62a5c0.webp")
                        .fit_to_exact_size(egui::Vec2 { x: 40.0, y: 40.0 }).rounding(50.0));
                    ui.add_space(10.0);
                    ui.add(Image::from_uri("https://cdn.discordapp.com/icons/1070692720437383208/8c5fd92777be939f3722565c6287777d.webp")
                        .fit_to_exact_size(egui::Vec2 { x: 40.0, y: 40.0 }).rounding(50.0));
                })

            });
        });

        SidePanel::right("right_panel").show_animated(ctx, self.panels.show_channel, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label("Channel List");
                    ui.separator();
                });
                ui.with_layout(
                    Layout::top_down(Align::Min).with_cross_justify(true),
                    |ui| {
                        for _ in 0..100 {
                            ui.label("Hello World!");
                        }
                    },
                );
            });
        });

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(0.5);
            menu::bar(ui, |ui| {
                ui.selectable_label(true, "Overview");
                ui.separator();
                ui.selectable_label(false, "User Table");
                ui.separator();
                ui.selectable_label(false, "Chart");
                ui.separator();
                ui.selectable_label(false, "Commonly Used Words");
                ui.separator();
            });
            ui.add_space(0.5);
        });
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status: Doing Nothing");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(Spinner::new());
                });
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            self.show_pass_ui(ui);
        });
    }
}
