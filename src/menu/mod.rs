use crate::prelude::*;

#[derive(Resource, Debug, Default, Reflect)]
pub struct MenuState {
    pub kind: MenuKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, Default)]
pub enum MenuKind {
    #[default]
    Title,
    Levels,
    Controls,
    Settings,
}

fn setup_egui_visuals(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let all_round = 2.0;
    let rounding = egui::Rounding {
        ne: all_round,
        nw: all_round,
        se: all_round,
        sw: all_round,
    };

    ctx.set_visuals(egui::Visuals {
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                weak_bg_fill: EGC8,
                bg_fill: EGC7,
                bg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC6,
                },
                expansion: 1.0,
                fg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC7,
                },
                rounding: rounding.clone(),
            },
            inactive: egui::style::WidgetVisuals {
                weak_bg_fill: EGC2,
                bg_fill: EGC2,
                bg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC2,
                },
                expansion: 1.0,
                fg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC8,
                },
                rounding: rounding.clone(),
            },
            hovered: egui::style::WidgetVisuals {
                weak_bg_fill: EGC1,
                bg_fill: EGC1,
                bg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC1,
                },
                expansion: 1.0,
                fg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC8,
                },
                rounding: rounding.clone(),
            },
            active: egui::style::WidgetVisuals {
                weak_bg_fill: EGC3,
                bg_fill: EGC3,
                bg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC3,
                },
                expansion: 1.0,
                fg_stroke: egui::Stroke {
                    width: egui::Stroke::default().width,
                    color: EGC8,
                },
                rounding: rounding.clone(),
            },
            ..default()
        },
        window_rounding: 0.0.into(),
        ..default()
    });

    ctx.style_mut(|style| {
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(48.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(72.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(48.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(36.0, egui::FontFamily::Monospace),
        );
    });

    // https://docs.rs/egui/latest/egui/struct.FontDefinitions.html
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/VT323-Regular.ttf")),
    );
    // Put twice to make top prio for prop and monospace
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn egui_always_helpers(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let screen_size = ctx.screen_rect().size();

    let side_width = if screen_size.x < screen_size.y {
        0.0
    } else {
        (screen_size.x - screen_size.y) / 2.0
    };
    let head_toes_width = if screen_size.x < screen_size.y {
        (screen_size.y - screen_size.x) / 2.0
    } else {
        0.0
    };
    egui::SidePanel::left("left")
        .exact_width(side_width)
        .frame(egui::Frame::none())
        .show(ctx, |_ui| {});
    egui::SidePanel::right("right")
        .exact_width(side_width)
        .frame(egui::Frame::none())
        .show(ctx, |_ui| {});
    egui::TopBottomPanel::top("top")
        .exact_height(head_toes_width)
        .frame(egui::Frame::none())
        .show(ctx, |_ui| {});
    egui::TopBottomPanel::bottom("bottom")
        .exact_height(head_toes_width)
        .frame(egui::Frame::none())
        .show(ctx, |_ui| {});
}

#[derive(Resource)]
pub struct EguiImageHandles {
    pub tier_1_full: Handle<Image>,
    pub tier_1_empty: Handle<Image>,
    pub tier_2_full: Handle<Image>,
    pub tier_2_empty: Handle<Image>,
    pub tier_3_full: Handle<Image>,
    pub tier_3_empty: Handle<Image>,
    pub title: Handle<Image>,
}
impl FromWorld for EguiImageHandles {
    fn from_world(world: &mut World) -> Self {
        let ass = world.get_resource::<AssetServer>().unwrap();
        Self {
            tier_1_full: ass.load("menu/tier_1.png"),
            tier_1_empty: ass.load("menu/tier_1_empty.png"),
            tier_2_full: ass.load("menu/tier_2.png"),
            tier_2_empty: ass.load("menu/tier_2_empty.png"),
            tier_3_full: ass.load("menu/tier_3.png"),
            tier_3_empty: ass.load("menu/tier_3_empty.png"),
            title: ass.load("menu/title.png"),
        }
    }
}
#[derive(Resource, Default)]
pub struct EguiTextures {
    pub tier_1_full: egui::TextureId,
    pub tier_1_empty: egui::TextureId,
    pub tier_2_full: egui::TextureId,
    pub tier_2_empty: egui::TextureId,
    pub tier_3_full: egui::TextureId,
    pub tier_3_empty: egui::TextureId,
    pub title: egui::TextureId,
}

pub fn control_butt_size() -> [f32; 2] {
    [164.0, 36.0]
}

pub fn render_tier_grid(
    ui: &mut egui::Ui,
    total_width: f32,
    tiers: &LevelTierCutoff,
    showscore: Option<i32>,
    egui_textures: &EguiTextures,
) {
    let hiscore = showscore.unwrap_or(-1000000);
    egui::Grid::new("one")
        .num_columns(3)
        .min_col_width(total_width / 3.0)
        .show(ui, |ui| {
            ui.label(tiers.one.to_string());
            ui.label(tiers.two.to_string());
            ui.label(tiers.three.to_string());
            ui.end_row();

            let image_size = [96.0, 96.0];
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                if hiscore < tiers.one {
                    egui_textures.tier_1_empty
                } else {
                    egui_textures.tier_1_full
                },
                image_size.clone(),
            )));
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                if hiscore < tiers.two {
                    egui_textures.tier_2_empty
                } else {
                    egui_textures.tier_2_full
                },
                image_size.clone(),
            )));
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                if hiscore < tiers.three {
                    egui_textures.tier_3_empty
                } else {
                    egui_textures.tier_3_full
                },
                image_size.clone(),
            )));
            ui.end_row();
        });
}

pub fn handle_egui_images(
    mut images_initialized: Local<bool>,
    mut egui_textures: ResMut<EguiTextures>,
    egui_image_handles: Res<EguiImageHandles>,
    mut contexts: EguiContexts,
) {
    if !*images_initialized {
        *images_initialized = true;
        egui_textures.tier_1_empty =
            contexts.add_image(egui_image_handles.tier_1_empty.clone_weak());
        egui_textures.tier_1_full = contexts.add_image(egui_image_handles.tier_1_full.clone_weak());
        egui_textures.tier_2_empty =
            contexts.add_image(egui_image_handles.tier_2_empty.clone_weak());
        egui_textures.tier_2_full = contexts.add_image(egui_image_handles.tier_2_full.clone_weak());
        egui_textures.tier_3_empty =
            contexts.add_image(egui_image_handles.tier_3_empty.clone_weak());
        egui_textures.tier_3_full = contexts.add_image(egui_image_handles.tier_3_full.clone_weak());
        egui_textures.title = contexts.add_image(egui_image_handles.title.clone_weak());
    }
}

pub fn menu_ui(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MenuState>,
    mut commands: Commands,
    loading_anim: Query<&AnimMan<LoadingAnim>>,
    mut save_data: ResMut<Pers<SaveData>>,
    mut store: ResMut<PkvStore>,
    egui_textures: Res<EguiTextures>,
    mut settings: ResMut<Pers<Settings>>,
) {
    if loading_anim
        .iter()
        .any(|anim| anim.get_state() != LoadingAnim::None)
    {
        return;
    }

    let ctx = contexts.ctx_mut();

    let current_level = &LEVEL_DEFNS[save_data.get().menu_ix as usize];
    let current_level_ix = save_data.get().menu_ix;
    let current_hiscore = save_data
        .get()
        .map
        .get(&current_level.lid)
        .cloned()
        .unwrap_or_default()
        .hiscore;
    let current_hiscore_str = current_hiscore
        .map(|num| format!("{num}"))
        .unwrap_or("NONE".to_string());

    let can_dec_level_ix = current_level_ix > 0;
    let can_inc_level_ix = {
        current_level_ix + 1 < LEVEL_DEFNS.len() as u32
            && current_hiscore.unwrap_or(-1000000) >= current_level.tiers.one
    };

    let update_level_ix = |diff: i32, sd: &mut ResMut<Pers<SaveData>>, st: &mut PkvStore| {
        let old = sd.get().clone();
        let menu_ix = (sd.get().menu_ix as i32 + diff) as u32;
        sd.set(SaveData { menu_ix, ..old });
        sd.save(st);
    };

    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            let screen_size = ctx.available_rect().size();
            match menu_state.kind {
                MenuKind::Title => {
                    ui.vertical_centered(|ui| {
                        let scale_up_title = 3.5;
                        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                            egui_textures.title,
                            [128.0 * scale_up_title, 96.0 * scale_up_title],
                        )));
                        ui.add_space((screen_size.y / 2.0 - 96.0 * scale_up_title).max(0.0));
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("PLAY"))
                            .clicked()
                        {
                            commands.spawn(SoundEffect::MenuClick);
                            menu_state.kind = MenuKind::Levels;
                        }
                        ui.add_space(control_butt_size()[1] / 2.0);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("CONTROLS"))
                            .clicked()
                        {
                            commands.spawn(SoundEffect::MenuClick);
                            menu_state.kind = MenuKind::Controls;
                        }
                        ui.add_space(control_butt_size()[1] / 2.0);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("SETTINGS"))
                            .clicked()
                        {
                            commands.spawn(SoundEffect::MenuClick);
                            menu_state.kind = MenuKind::Settings;
                        }
                    });
                }
                MenuKind::Levels => {
                    let force_width = 400.0;
                    ui.vertical_centered(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(EGC1);
                        ui.set_min_width(force_width);
                        ui.set_max_width(force_width);
                        let vspacing = screen_size.y / 24.0;
                        ui.add_space(vspacing);
                        ui.heading("LEVELS");
                        ui.add_space(vspacing);
                        ui.vertical(|ui| {
                            ui.set_min_height(screen_size.y / 3.0);
                            ui.set_max_height(screen_size.y / 3.0);
                            ui.vertical_centered(|ui| {
                                ui.label(&current_level.name);
                                ui.small(format!(
                                    "({} / {})",
                                    current_level_ix + 1,
                                    LEVEL_DEFNS.len()
                                ));
                                ui.label(format!("HISCORE: {current_hiscore_str}"));
                            });
                            ui.add_space(vspacing);
                            render_tier_grid(
                                ui,
                                force_width,
                                &current_level.tiers,
                                current_hiscore,
                                &egui_textures,
                            );
                            ui.add_space(vspacing);
                        });
                        ui.style_mut().visuals.override_text_color = Some(EGC8);
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(can_dec_level_ix, |ui| {
                                if ui
                                    .add_sized(
                                        [force_width / 2.0, control_butt_size()[1]],
                                        egui::Button::new(egui::RichText::new(" < ").size(36.0)),
                                    )
                                    .clicked()
                                {
                                    commands.spawn(SoundEffect::MenuClick);
                                    update_level_ix(-1, &mut save_data, &mut store);
                                }
                            });
                            ui.add_enabled_ui(can_inc_level_ix, |ui| {
                                if ui
                                    .add_sized(
                                        ui.available_size(),
                                        egui::Button::new(egui::RichText::new(" > ").size(36.0)),
                                    )
                                    .clicked()
                                {
                                    commands.spawn(SoundEffect::MenuClick);
                                    update_level_ix(1, &mut save_data, &mut store);
                                }
                            });
                        });
                        ui.add_space(vspacing);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [force_width / 2.0, control_butt_size()[1]],
                                    egui::Button::new("BACK"),
                                )
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                menu_state.kind = MenuKind::Title;
                            }
                            if ui
                                .add_sized(ui.available_size(), egui::Button::new("PLAY"))
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                commands.trigger(LoadLevel::lid(&current_level.lid));
                            }
                        });
                    });
                }
                MenuKind::Controls => {
                    let force_width = 560.0;
                    let vspacing = 20.0;
                    ui.vertical_centered(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(EGC1);
                        ui.set_min_width(force_width);
                        ui.set_max_width(force_width);
                        ui.add_space(vspacing);
                        ui.heading("CONTROLS");
                        ui.label("This is a one-button game.");
                        ui.label("Use spacebar OR left click.");
                        ui.add_space(vspacing);
                        ui.label("Hold to launch.");
                        ui.label("Hold/tap to adjust flight.");
                        ui.add_space(vspacing);
                        ui.label("R restarts the level.");
                        ui.label("ESC pauses the level.");
                        ui.add_space(vspacing);
                        ui.label("Holding launch too long sends you nowhere.");
                        ui.add_space(vspacing);
                        ui.style_mut().visuals.override_text_color = Some(EGC8);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("BACK"))
                            .clicked()
                        {
                            commands.spawn(SoundEffect::MenuClick);
                            menu_state.kind = MenuKind::Title;
                        }
                    });
                }
                MenuKind::Settings => {
                    let force_width = 400.0;
                    let vspacing = 20.0;

                    ui.vertical_centered(|ui| {
                        ui.set_min_width(force_width);
                        ui.set_max_width(force_width);
                        ui.add_space(vspacing);
                        ui.style_mut().visuals.override_text_color = Some(EGC1);
                        ui.heading("SETTINGS");
                        ui.style_mut().visuals.override_text_color = Some(EGC8);

                        ui.horizontal(|ui| {
                            let old = settings.get().clone();
                            if ui
                                .add_sized(
                                    [force_width / 4.0, control_butt_size()[1]],
                                    egui::Button::new("-"),
                                )
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                settings.set(Settings {
                                    music_volume: (old.music_volume - 0.1).max(0.0),
                                    ..old
                                });
                                settings.save(&mut store);
                            }
                            ui.style_mut().visuals.override_text_color = Some(EGC1);
                            ui.add_sized(
                                [force_width / 2.0, ui.available_height()],
                                egui::Label::new(format!(
                                    "MUSIC: {:.1}",
                                    settings.get().music_volume
                                )),
                            );
                            ui.style_mut().visuals.override_text_color = Some(EGC8);
                            if ui
                                .add_sized(ui.available_size(), egui::Button::new("+"))
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                settings.set(Settings {
                                    music_volume: (old.music_volume + 0.1).min(1.0),
                                    ..old
                                });
                                settings.save(&mut store);
                            }
                        });

                        ui.add_space(vspacing);
                        ui.horizontal(|ui| {
                            let old = settings.get().clone();
                            if ui
                                .add_sized(
                                    [force_width / 4.0, control_butt_size()[1]],
                                    egui::Button::new("-"),
                                )
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                settings.set(Settings {
                                    effect_volume: (old.effect_volume - 0.1).max(0.0),
                                    ..old
                                });
                                settings.save(&mut store);
                            }
                            ui.style_mut().visuals.override_text_color = Some(EGC1);
                            ui.add_sized(
                                [force_width / 2.0, ui.available_height()],
                                egui::Label::new(format!(
                                    "SOUND: {:.1}",
                                    settings.get().effect_volume
                                )),
                            );
                            ui.style_mut().visuals.override_text_color = Some(EGC8);
                            if ui
                                .add_sized(ui.available_size(), egui::Button::new("+"))
                                .clicked()
                            {
                                commands.spawn(SoundEffect::MenuClick);
                                settings.set(Settings {
                                    effect_volume: (old.effect_volume + 0.1).min(1.0),
                                    ..old
                                });
                                settings.save(&mut store);
                            }
                        });

                        ui.add_space(vspacing);
                        if ui.add(egui::Button::new("RESET SAVE DATA")).clicked() {
                            commands.spawn(SoundEffect::MenuClick);
                            save_data.set(default());
                            save_data.save(&mut store);
                            menu_state.kind = MenuKind::Levels;
                        }

                        ui.add_space(vspacing);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("BACK"))
                            .clicked()
                        {
                            commands.spawn(SoundEffect::MenuClick);
                            menu_state.kind = MenuKind::Title;
                        }
                    });
                }
            }
        });
}

fn on_enter(mut commands: Commands) {
    commands.trigger(SetupBg::kind(BgKind::Sky));
}

fn on_exit() {}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuState::default());
        app.init_resource::<EguiImageHandles>();
        app.insert_resource(EguiTextures::default());
        app.add_systems(Startup, setup_egui_visuals);

        app.add_systems(Update, egui_always_helpers);

        app.add_systems(OnEnter(MetaState::Menu), on_enter);
        app.add_systems(
            Update,
            (handle_egui_images, menu_ui)
                .run_if(in_state(MetaState::Menu))
                .after(egui_always_helpers),
        );
        app.add_systems(OnExit(MetaState::Menu), on_exit);
    }
}
