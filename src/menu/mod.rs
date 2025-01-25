use bevy::tasks::AsyncComputeTaskPool;

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

pub fn control_butt_size() -> [f32; 2] {
    [164.0, 36.0]
}

pub fn render_tier_grid(
    ui: &mut egui::Ui,
    total_width: f32,
    tiers: &LevelTierCutoff,
    hiscore: Option<f32>,
) {
    egui::Grid::new("one")
        .num_columns(3)
        .min_col_width(total_width / 3.0)
        .show(ui, |ui| {
            ui.label("Star One");
            ui.label("Star Two");
            ui.label("Star Three");
            ui.end_row();

            ui.label(tiers.one.to_string());
            ui.label(tiers.two.to_string());
            ui.label(tiers.three.to_string());
            ui.end_row();
        });
}

pub fn menu_ui(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MenuState>,
    mut commands: Commands,
    loading_anim: Query<&AnimMan<LoadingAnim>>,
    mut save_data: ResMut<Pers<SaveData>>,
    mut store: ResMut<PkvStore>,
) {
    if loading_anim
        .iter()
        .any(|anim| anim.get_state() != LoadingAnim::None)
    {
        return;
    }

    let ctx = contexts.ctx_mut();

    let current_level_ix = save_data.get().menu_ix;
    let can_dec_level_ix = current_level_ix > 0;
    let can_inc_level_ix = current_level_ix + 1 < LEVEL_DEFNS.len() as u32;
    let current_level = &LEVEL_DEFNS[save_data.get().menu_ix as usize];
    let current_hiscore_str = save_data
        .get()
        .map
        .get(&current_level.lid)
        .cloned()
        .unwrap_or_default()
        .hiscore
        .map(|num| format!("{num}"))
        .unwrap_or("NONE".to_string());

    let mut update_level_ix = |diff: i32| {
        let old = save_data.get().clone();
        let menu_ix = (save_data.get().menu_ix as i32 + diff) as u32;
        save_data.set(SaveData { menu_ix, ..old });
        save_data.save(&mut store);
    };

    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            let screen_size = ctx.available_rect().size();
            match menu_state.kind {
                MenuKind::Title => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(screen_size.y / 2.0);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("PLAY"))
                            .clicked()
                        {
                            menu_state.kind = MenuKind::Levels;
                        }
                        ui.add_space(control_butt_size()[1] / 2.0);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("CONTROLS"))
                            .clicked()
                        {
                            menu_state.kind = MenuKind::Levels;
                        }
                        ui.add_space(control_butt_size()[1] / 2.0);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("SETTINGS"))
                            .clicked()
                        {
                            menu_state.kind = MenuKind::Levels;
                        }
                    });
                }
                MenuKind::Levels => {
                    let force_width = 600.0;
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
                                ui.add_space(vspacing);
                                ui.label(format!("HISCORE: {current_hiscore_str}"));
                            });
                            ui.add_space(vspacing);
                            render_tier_grid(ui, force_width, &current_level.tiers, None);
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
                                    update_level_ix(-1);
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
                                    update_level_ix(1);
                                }
                            });
                        });
                        ui.add_space(vspacing);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("PLAY"))
                            .clicked()
                        {
                            commands.trigger(LoadLevel::lid(&current_level.lid));
                        }
                        ui.add_space(vspacing);
                        if ui
                            .add_sized(control_butt_size(), egui::Button::new("BACK"))
                            .clicked()
                        {
                            menu_state.kind = MenuKind::Title;
                        }
                        ui.add_space(vspacing);
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
        app.add_systems(Startup, setup_egui_visuals);

        app.add_systems(Update, egui_always_helpers);

        app.add_systems(OnEnter(MetaState::Menu), on_enter);
        app.add_systems(
            Update,
            menu_ui
                .run_if(in_state(MetaState::Menu))
                .after(egui_always_helpers),
        );
        app.add_systems(OnExit(MetaState::Menu), on_exit);
    }
}
