use egui::{
    global_dark_light_mode_switch, menu, Align, CentralPanel, Context, Image, ImageButton, Layout,
    ScrollArea, SidePanel, Spinner, TopBottomPanel,
};
use funnel_shared::GuildWithChannels;
use std::collections::VecDeque;
use strum::IntoEnumIterator;

use crate::core::{MainWindow, TabState};
use crate::ui::DateNavigator;
use crate::{AppEvent, AppStatus};

pub struct PanelStatus {
    tab_state: TabState,
    show_guild: bool,
    show_channel: bool,
    dot_count: usize,
    date_nav: DateNavigator,
    app_status: AppStatus,
    guild_channels: Vec<GuildWithChannels>,
    selected_guild: usize,
    selected_channel: usize,
}

impl Default for PanelStatus {
    fn default() -> Self {
        Self {
            tab_state: TabState::default(),
            show_guild: true,
            show_channel: true,
            dot_count: 0,
            date_nav: DateNavigator::default(),
            app_status: AppStatus::default(),
            guild_channels: Vec::new(),
            selected_guild: 0,
            selected_channel: 0,
        }
    }
}

impl PanelStatus {
    fn show_upper_bar(&mut self, ctx: &Context, events: &mut VecDeque<AppEvent>) {
        TopBottomPanel::top("upper_bar")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
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
                    ui.separator();
                    self.date_nav.show_ui(ui, events);
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

    fn show_left_bar(&mut self, ctx: &Context) {
        SidePanel::left("left_panel")
            .max_width(70.0)
            .resizable(false)
            .show_animated(ctx, self.show_guild, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.0);
                        ui.label("Guild List");
                        ui.separator();
                        ui.add_space(10.0);

                        for (index, guild) in self.guild_channels.iter().enumerate() {
                            let guild_name = &guild.guild.guild_name;
                            let guild_image = if guild.guild.guild_icon.is_none() {
                                let modified_name = guild_name.replace(" ", "%20");
                                format!(
                                    "https://api.dicebear.com/9.x/initials/png?seed={}",
                                    modified_name
                                )
                            } else {
                                guild.guild.guild_icon.clone().unwrap()
                            };
                            let selected = self.selected_guild == index;

                            let anim_id = ui.make_persistent_id("guild_rounding_anim").with(index);

                            let target_rounding = if selected { 50.0 } else { 10.0 };

                            let rounding =
                                ctx.animate_value_with_time(anim_id, target_rounding, 0.3);

                            if ui
                                .add(
                                    ImageButton::new(
                                        Image::from_uri(guild_image).rounding(rounding),
                                    )
                                    .selected(selected)
                                    .rounding(rounding)
                                    .frame(true),
                                )
                                .clicked()
                            {
                                self.selected_guild = index;
                            };
                        }
                    })
                });
            });
    }

    fn show_right_bar(&mut self, ctx: &Context) {
        SidePanel::right("right_panel").show_animated(ctx, self.show_channel, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label("Channel List");
                    ui.separator();
                });
                if !self.guild_channels.is_empty() {
                    ui.with_layout(
                        Layout::top_down(Align::Min).with_cross_justify(true),
                        |ui| {
                            let selected_guild = self.selected_guild;
                            let channel_list = &self.guild_channels[selected_guild].channels;

                            let all_channel_selected = self.selected_channel == 0;

                            if ui
                                .selectable_label(all_channel_selected, "All Channels")
                                .clicked()
                            {
                                self.selected_channel = 0;
                            };
                            for (index, channel) in channel_list.iter().enumerate() {
                                let index = index + 1;

                                let channel_selected = self.selected_channel == index;

                                if ui
                                    .selectable_label(channel_selected, &channel.channel_name)
                                    .clicked()
                                {
                                    self.selected_channel = index;
                                };
                            }
                        },
                    );
                }
            });
        });
    }

    fn show_bottom_bar(&mut self, ctx: &Context) {
        let show_spinner = self.app_status.show_spinner();
        TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let mut status_text = self.app_status.to_string();
                    if show_spinner {
                        status_text.push_str(".".repeat(self.dot_count).as_ref());
                    }
                    ui.label(format!("Status: {}", status_text));

                    if show_spinner {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add(Spinner::new());
                        });
                    }
                })
            });
    }

    pub fn set_app_status(&mut self, status: AppStatus) {
        self.app_status = status;
    }

    pub fn next_dot(&mut self) {
        if self.dot_count == 3 {
            self.dot_count = 0
        } else {
            self.dot_count += 1;
        }
    }

    pub fn set_guild_channels(&mut self, list: Vec<GuildWithChannels>) {
        self.guild_channels = list;
    }
}

impl MainWindow {
    pub fn show_panels(&mut self, ctx: &Context) {
        self.panels.show_upper_bar(ctx, &mut self.events);
        self.panels.show_left_bar(ctx);
        self.panels.show_right_bar(ctx);
        self.panels.show_bottom_bar(ctx);

        if self.password.pass_authenticated() {
            self.panels.show_top_bar(ctx);
        }

        CentralPanel::default().show(ctx, |ui| {
            if !self.password.pass_authenticated() {
                self.password.show_pass_ui(ui, &mut self.events);
            } else {
                self.tabs
                    .show_tab_ui(ui, self.panels.tab_state, &mut self.events);
            }
        });
    }
}
