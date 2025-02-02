use chrono::NaiveDate;
use eframe::egui::ahash::{HashMap, HashSet, HashSetExt};
use eframe::egui::{
    menu, Align, CentralPanel, Context, Image, ImageButton, Layout, Rounding, ScrollArea,
    SidePanel, Spinner, TopBottomPanel, Visuals,
};
use egui_theme_lerp::ThemeAnimator;
use funnel_shared::{Channel, GuildWithChannels, UserDetails};
use strum::IntoEnumIterator;

use crate::core::{FetchStatus, MainWindow, TabState};
use crate::ui::{AnimatedLabel, AnimatedMenuLabel, DateHandler, DateNavigator};
use crate::{AppEvent, AppStatus, EventBus};

pub struct PanelStatus {
    tab_state: TabState,
    show_guild: bool,
    show_channel: bool,
    dot_count: usize,
    date_nav: Vec<DateNavigator>,
    app_status: AppStatus,
    guild_channels: Vec<GuildWithChannels>,
    selected_guild: usize,
    selected_channel: Vec<HashSet<usize>>,
    hovered_guild: Option<usize>,
    guild_changed: bool,
    reset_guild_anim: bool,
    top_button_size: f32,
    theme_animator: ThemeAnimator,
    fetch_status: HashMap<i64, FetchStatus>,
    user_details: Option<UserDetails>,
}

impl Default for PanelStatus {
    fn default() -> Self {
        Self {
            tab_state: TabState::default(),
            show_guild: true,
            show_channel: true,
            dot_count: 0,
            date_nav: vec![DateNavigator::default()],
            app_status: AppStatus::default(),
            guild_channels: Vec::new(),
            selected_guild: 0,
            selected_channel: Vec::new(),
            hovered_guild: None,
            guild_changed: false,
            reset_guild_anim: false,
            top_button_size: 0.0,
            theme_animator: ThemeAnimator::new(Visuals::light(), Visuals::dark()),
            fetch_status: HashMap::default(),
            user_details: None,
        }
    }
}

impl PanelStatus {
    fn show_upper_bar(&mut self, ctx: &Context, connected: bool, event_bus: &mut EventBus) {
        TopBottomPanel::top("upper_bar")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                menu::bar(ui, |ui| {
                    let theme_emoji = if !self.theme_animator.animation_done {
                        if self.theme_animator.theme_1_to_2 {
                            "☀"
                        } else {
                            "🌙"
                        }
                    } else if self.theme_animator.theme_1_to_2 {
                        "🌙"
                    } else {
                        "☀"
                    };
                    if ui.button(theme_emoji).clicked() {
                        self.theme_animator.start();
                    }

                    ui.set_style(ctx.style());
                    ui.separator();
                    if ui
                        .selectable_label(self.show_guild, "Guild List")
                        .on_hover_text("Show/Hide Guild List")
                        .clicked()
                    {
                        self.show_guild = !self.show_guild;
                    };
                    ui.separator();
                    if ui
                        .selectable_label(self.show_channel, "Channel List")
                        .on_hover_text("Show/Hide Channel List")
                        .clicked()
                    {
                        self.show_channel = !self.show_channel;
                    };
                    ui.separator();
                    self.date_nav[self.selected_guild].show_ui(ui, connected, event_bus);

                    if let Some(details) = self.user_details.as_ref() {
                        ui.separator();
                        if ui
                            .button("Log Out")
                            .on_hover_text("Log Out from the current discord")
                            .clicked()
                        {
                            event_bus.publish(AppEvent::LogOut);
                        };
                        ui.separator();
                        ui.add(Image::new(details.avatar_link()).rounding(15.0));
                        ui.label(details.full_username());
                    }
                });

                ui.add_space(0.5);
            });
    }

    fn show_top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(3.0);
                menu::bar(ui, |ui| {
                    ui.set_style(ctx.style());

                    let space_anim = ui.make_persistent_id("top_spacing_anim");
                    if self.top_button_size != 0.0 {
                        let max_size = ui.available_width();
                        let remaining = max_size - self.top_button_size;
                        let remaining = ui.painter().round_to_pixel_center(remaining);
                        let space_amount =
                            ui.ctx()
                                .animate_value_with_time(space_anim, remaining / 2.0, 0.5);
                        ui.add_space(space_amount);
                    } else {
                        ui.ctx().animate_value_with_time(space_anim, 0.0, 0.0);
                    }
                    let hover_position = ui.make_persistent_id("menu_hover");
                    let selected_position = ui.make_persistent_id("menu_selected");
                    let max_width = ui.available_width();
                    for val in TabState::iter() {
                        let val_string = val.to_string();
                        let selected = self.tab_state == val;

                        let first_value = val == TabState::first_value();

                        let resp = ui.add(AnimatedMenuLabel::new(
                            selected,
                            val_string,
                            selected_position,
                            hover_position,
                            100.0,
                            18.0,
                            Some(Rounding::ZERO),
                            (first_value, true),
                        ));

                        if resp.clicked() {
                            self.tab_state = val;
                        }
                    }
                    let space_taken = max_width - ui.available_width();
                    self.top_button_size = space_taken;
                });
                ui.add_space(3.0);
            });
    }

    fn show_left_bar(&mut self, ctx: &Context, event_bus: &mut EventBus) {
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

                        let mut nothing_hovered = true;

                        for (index, guild) in self.guild_channels.iter().enumerate() {
                            let guild_name = &guild.guild.guild_name;
                            let guild_image = if guild.guild.guild_icon.is_none() {
                                let modified_name = guild_name.replace(' ', "%20");
                                format!(
                                    "https://api.dicebear.com/9.x/initials/png?seed={modified_name}"
                                )
                            } else {
                                guild.guild.guild_icon.clone().unwrap()
                            };
                            let selected = self.selected_guild == index;

                            let anim_id = ui.make_persistent_id("guild_rounding_anim").with(index);

                            if self.reset_guild_anim {
                                ctx.animate_value_with_time(anim_id, 10.0, 0.0);
                                self.reset_guild_anim = false;
                            }

                            let target_rounding = if selected {
                                25.0
                            } else if let Some(id) = self.hovered_guild {
                                if id == index {
                                    20.0
                                } else {
                                    10.0
                                }
                            } else {
                                10.0
                            };

                            let rounding =
                                ctx.animate_value_with_time(anim_id, target_rounding, 0.5);

                            let resp = ui
                                .add(
                                    ImageButton::new(Image::from_uri(guild_image))
                                        .selected(selected)
                                        .rounding(rounding),
                                )
                                .on_hover_text(guild_name);

                            if resp.hovered() {
                                nothing_hovered = false;
                                self.hovered_guild = Some(index);
                            }

                            if resp.clicked() {
                                self.selected_guild = index;
                                self.guild_changed = true;
                                event_bus.publish(AppEvent::GuildChanged);
                            }
                        }

                        if nothing_hovered {
                            self.hovered_guild = None;
                        }
                    })
                });
            });
    }

    fn show_right_bar(&mut self, ctx: &Context, event_bus: &mut EventBus) {
        SidePanel::right("right_panel")
            .default_width(120.0)
            .max_width(200.0)
            .show_animated(ctx, self.show_channel, |ui| {
                ScrollArea::vertical().drag_to_scroll(false).show(ui, |ui| {
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

                                let mut channel_name_list = vec!["All Channels"];

                                for ch in channel_list {
                                    channel_name_list.push(&ch.channel_name);
                                }

                                let space_id = ui.make_persistent_id("space_id");

                                if self.guild_changed {
                                    ui.ctx().animate_value_with_time(space_id, 10.0, 0.0);
                                }
                                let spacing = ui.ctx().animate_value_with_time(space_id, 0.0, 0.5);

                                let reset_label = if self.guild_changed {
                                    self.guild_changed = false;
                                    true
                                } else {
                                    false
                                };

                                // The id where value of the current hover position is saved
                                let hover_position = ui.make_persistent_id("channel_hover_anim");

                                for (index, channel_name) in channel_name_list.iter().enumerate() {
                                    // The selection UI position, animate toward the current
                                    // position from either the top or the bottom
                                    let selection_position =
                                        ui.make_persistent_id("channel_selection_anim").with(index);

                                    ui.add_space(spacing);

                                    let channel_selected =
                                        self.selected_channel[self.selected_guild].contains(&index);

                                    // The text position. Animate from the current position from
                                    // either the top or the bottom
                                    let text_position =
                                        ui.make_persistent_id("text_position_anim").with(index);

                                    let resp = ui.add(AnimatedLabel::new(
                                        channel_selected,
                                        *channel_name,
                                        text_position,
                                        selection_position,
                                        hover_position,
                                    ));

                                    if reset_label {
                                        ui.ctx().animate_value_with_time(
                                            selection_position,
                                            ui.max_rect().top(),
                                            0.0,
                                        );
                                        ui.ctx().animate_value_with_time(
                                            text_position,
                                            ui.max_rect().top(),
                                            0.0,
                                        );
                                    }

                                    if resp.clicked() {
                                        // All channels cannot be selected if other channels are
                                        // also selected.
                                        // Other channels cannot be selected if all channels is
                                        // selected.
                                        // index 0 = all channel
                                        if index != 0 {
                                            self.selected_channel[self.selected_guild].remove(&0);
                                        } else if index == 0 {
                                            let insert_zero_again = self.selected_channel
                                                [self.selected_guild]
                                                .contains(&0);
                                            self.selected_channel[self.selected_guild].clear();
                                            if insert_zero_again {
                                                self.selected_channel[self.selected_guild]
                                                    .insert(0);
                                            }
                                        }

                                        let already_selected = self.selected_channel
                                            [self.selected_guild]
                                            .contains(&index);

                                        if already_selected {
                                            self.selected_channel[self.selected_guild]
                                                .remove(&index);
                                        } else {
                                            self.selected_channel[self.selected_guild]
                                                .insert(index);
                                            let available_rect = ui.max_rect();
                                            let rect_center = available_rect.center().y;

                                            let current_point = ui
                                                .ctx()
                                                .input(|i| i.pointer.hover_pos())
                                                .unwrap()
                                                .y;

                                            if current_point > rect_center {
                                                ui.ctx().animate_value_with_time(
                                                    selection_position,
                                                    available_rect.bottom(),
                                                    0.0,
                                                );
                                            } else {
                                                ui.ctx().animate_value_with_time(
                                                    selection_position,
                                                    available_rect.top(),
                                                    0.0,
                                                );
                                            }
                                        }
                                        event_bus.publish(AppEvent::SelectedChannelsChanged);
                                    }
                                }
                            },
                        );
                    }
                });
            });
    }

    pub fn show_bottom_bar(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let show_spinner = self.app_status.show_spinner();

                    let mut status_text = self.app_status.to_string();
                    if show_spinner {
                        status_text.push_str(".".repeat(self.dot_count).as_ref());
                    }
                    ui.label(format!("Status: {status_text}"));

                    if show_spinner {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add(Spinner::new());
                        });
                    }
                });
                ui.add_space(0.5);
            });
    }

    pub fn set_app_status(&mut self, status: AppStatus) {
        self.app_status = status;
    }

    pub fn next_dot(&mut self) {
        if self.dot_count == 3 {
            self.dot_count = 0;
        } else {
            self.dot_count += 1;
        }
    }

    pub fn set_guild_channels(&mut self, list: Vec<GuildWithChannels>) {
        self.selected_channel.clear();

        let mut date_list = vec![];

        for guild in &list {
            let guild_id = guild.guild.guild_id;

            date_list.push(DateNavigator::default());

            self.fetch_status.entry(guild_id).or_default();
            self.selected_channel.push(HashSet::new());
        }

        self.guild_channels = list;
        self.guild_changed = true;
        self.reset_guild_anim = true;
        self.date_nav = date_list;
    }

    pub fn date_update(&mut self, date: NaiveDate, guild_id: i64) -> DateHandler {
        let target_index = self
            .guild_channels
            .iter()
            .position(|g| g.guild.guild_id == guild_id)
            .unwrap();
        self.date_nav[target_index].handler().update_dates(date);
        self.date_nav[target_index].handler_i()
    }

    pub fn selected_guild(&self) -> i64 {
        let index = self.selected_guild;
        self.guild_channels[index].guild.guild_id
    }

    pub fn current_date_handler(&self) -> DateHandler {
        self.date_nav[self.selected_guild].handler_i()
    }

    pub fn date_handler(&self, guild_id: i64) -> DateHandler {
        let target_index = self
            .guild_channels
            .iter()
            .position(|g| g.guild.guild_id == guild_id)
            .unwrap();
        self.date_nav[target_index].handler_i()
    }

    pub fn current_selected_channels(&self) -> HashSet<usize> {
        self.selected_channel[self.selected_guild].clone()
    }

    pub fn current_guild_channels(&self) -> Vec<Channel> {
        self.guild_channels[self.selected_guild].channels.clone()
    }

    pub fn current_guild_status(&self) -> &FetchStatus {
        &self.fetch_status[&self.selected_guild()]
    }

    pub fn current_guild_status_m(&mut self) -> &mut FetchStatus {
        self.fetch_status.get_mut(&self.selected_guild()).unwrap()
    }

    pub fn set_user_details(&mut self, user_details: UserDetails) {
        self.user_details = Some(user_details);
    }

    pub fn has_user_details(&self) -> bool {
        self.user_details.is_some()
    }
}

impl MainWindow {
    pub fn show_panels(&mut self, ctx: &Context) {
        self.panels
            .show_upper_bar(ctx, self.connection.connected(), &mut self.event_bus);
        self.panels.show_left_bar(ctx, &mut self.event_bus);
        self.panels.show_right_bar(ctx, &mut self.event_bus);
        self.panels.show_bottom_bar(ctx);

        if self.connection.connected() {
            self.panels.show_top_bar(ctx);
        }

        CentralPanel::default().show(ctx, |ui| {
            if self.panels.theme_animator.anim_id.is_none() {
                self.panels.theme_animator.create_id(ui);
            } else {
                self.panels.theme_animator.animate(ctx);
            };

            if !self.connection.connected() {
                self.connection.show_start_ui(ui, &mut self.event_bus);
            } else {
                self.tabs
                    .show_tab_ui(ui, self.panels.tab_state, &mut self.event_bus);
            }
        });
    }
}
