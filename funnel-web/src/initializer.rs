use eframe::{App, Frame};
use egui::{
    global_dark_light_mode_switch, menu, Align, CentralPanel, Context, Image, Layout, ScrollArea,
    SidePanel, Spinner, TopBottomPanel,
};
use egui_extras::{install_image_loaders, DatePickerButton};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct MainWindow {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Default for MainWindow {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                global_dark_light_mode_switch(ui);
            });
        });
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status: Doing Nothing");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(Spinner::new());
                });
            })
        });

        SidePanel::left("left_panel").max_width(70.0).resizable(false).show(ctx, |ui| {
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

        SidePanel::right("right_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label("Channel List");
                    ui.separator();
                });
                ui.with_layout(
                    Layout::top_down(Align::Min).with_cross_justify(true),
                    |ui| {
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                        ui.label("Hello World");
                    },
                );
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });

            ui.separator();
        });
    }
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.3);
        install_image_loaders(&cc.egui_ctx);
        Default::default()
    }
}
