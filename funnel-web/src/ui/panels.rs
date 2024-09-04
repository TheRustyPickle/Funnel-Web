use egui::{
    global_dark_light_mode_switch, menu, Align, CentralPanel, Context, Layout, ScrollArea,
    SidePanel, Spinner, TopBottomPanel,
};
use strum::IntoEnumIterator;

use crate::core::{MainWindow, TabState};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PanelStatus {
    tab_state: TabState,
    show_guild: bool,
    show_channel: bool,
}

impl Default for PanelStatus {
    fn default() -> Self {
        Self {
            tab_state: TabState::default(),
            show_guild: true,
            show_channel: true,
        }
    }
}

impl PanelStatus {
    fn show_upper_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("upper_bar")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
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

    fn show_top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                menu::bar(ui, |ui| {
                    for val in TabState::iter() {
                        let val_string = val.to_string();
                        ui.selectable_value(&mut self.tab_state, val, val_string);
                        ui.separator();
                    }
                });
                ui.add_space(1.0);
            });
    }
}

impl MainWindow {
    pub fn show_panels(&mut self, ctx: &Context) {
        self.panels.show_upper_bar(ctx);

        SidePanel::left("left_panel")
            .max_width(70.0)
            .resizable(false)
            .show_animated(ctx, self.panels.show_guild, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.0);
                        ui.label("Guild List");
                        ui.separator();
                        ui.add_space(10.0);
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

        if self.password.pass_authenticated() {
            self.panels.show_top_bar(ctx);
        }

        TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Status: Doing Nothing");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add(Spinner::new());
                    });
                })
            });

        CentralPanel::default().show(ctx, |ui| {
            if !self.password.pass_authenticated() {
                self.show_pass_ui(ui);
            } else {
                match self.panels.tab_state {
                    _ => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Under Construction");
                        });
                    }
                };
            }
        });
    }
}
