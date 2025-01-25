use egui::Color32;

use crate::prelude::*;

#[derive(Component, Default)]
#[require(RenderLayers(|| OverlayLayer::RENDER_LAYERS))]
struct Overlay;

#[derive(Component)]
#[require(Overlay)]
struct TextOverlay;
#[derive(Bundle)]
struct TextOverlayBundle {
    name: Name,
    text_overlay: TextOverlay,
    pos: Pos,
    transform: Transform,
    text2d: Text2d,
    font: TextFont,
    layout: TextLayout,
    anchor: Anchor,
    text_color: TextColor,
}
impl TextOverlayBundle {
    fn new(
        name: &str,
        text: &str,
        pos: Pos,
        font_hand: Handle<Font>,
        justify: JustifyText,
        anchor: Anchor,
    ) -> Self {
        Self {
            name: Name::new(name.to_string()),
            text_overlay: TextOverlay,
            pos,
            transform: Transform::from_scale(Vec3::ONE * 0.3),
            text2d: Text2d::new(text.to_string()),
            font: TextFont::from_font(font_hand)
                .with_font_size(36.0)
                .with_font_smoothing(bevy::text::FontSmoothing::None),
            layout: TextLayout::new_with_justify(justify),
            anchor,
            text_color: TextColor(Color::srgb_u8(205, 212, 165)),
        }
    }
}

fn on_enter_level(mut commands: Commands, root: Res<LevelMetaRoot>, ass: Res<AssetServer>) {
    let off_corner = 4.0;
    commands
        .spawn(TextOverlayBundle::new(
            "TextChefs",
            "CHEFS: 0",
            Pos::new(-SCREEN_VEC.x / 2.0 + off_corner, SCREEN_VEC.y / 2.0),
            ass.load("fonts/VT323-Regular.ttf"),
            JustifyText::Left,
            Anchor::TopLeft,
        ))
        .set_parent(root.eid());
    commands
        .spawn(TextOverlayBundle::new(
            "TextScore",
            "SCORE: 0",
            Pos::new(SCREEN_VEC.x / 2.0 - off_corner, SCREEN_VEC.y / 2.0),
            ass.load("fonts/VT323-Regular.ttf"),
            JustifyText::Right,
            Anchor::TopRight,
        ))
        .set_parent(root.eid());
}

fn update(
    chefs_q: Query<&AnimMan<ChefAnim>>,
    level_state: Res<LevelState>,
    mut texts_q: Query<&mut Text2d, With<TextOverlay>>,
) {
    let chefs_left = chefs_q
        .iter()
        .filter(|anim| {
            matches!(
                anim.get_state(),
                ChefAnim::Wait | ChefAnim::Ready | ChefAnim::Charge
            )
        })
        .count();
    let score = level_state.score;
    for mut text in &mut texts_q {
        if text.0.contains("CHEFS") {
            text.0 = format!("CHEFS: {chefs_left}");
        } else if text.0.contains("SCORE") {
            text.0 = format!("SCORE: {score}");
        }
    }
}

fn show_intro_messages(
    mut level_state: ResMut<LevelState>,
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    let ctx = contexts.ctx_mut();
    if level_state.intro_messages.is_empty() {
        return;
    }
    let msg = level_state.intro_messages[0].clone();
    let force_width = 500.0;
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(EGC8))
        .show(ctx, |ui| {
            ui.style_mut().visuals.override_text_color = Some(EGC1);
            let fh = ui.available_height() / 4.0;
            ui.add_space(fh);
            ui.vertical_centered(|ui| {
                ui.set_min_width(force_width);
                ui.set_max_width(force_width);
                ui.set_min_height(fh * 2.0);
                ui.set_max_height(fh * 2.0);
                ui.style_mut().visuals.override_text_color = Some(EGC1);
                ui.heading(&msg.title);
                ui.label(&msg.content);
            });
            ui.vertical_centered(|ui| {
                ui.style_mut().visuals.override_text_color = Some(EGC8);
                if ui.button("CONTINUE").clicked() {
                    commands.spawn(SoundEffect::MenuClick);
                    level_state.intro_messages.remove(0);
                }
            });
        });
}

fn pause_menu(
    mut level_state: ResMut<LevelState>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    egui_textures: Res<EguiTextures>,
    save_data: Res<Pers<SaveData>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        level_state.paused = !level_state.paused;
    }
    if !level_state.paused {
        return;
    }
    if !level_state.intro_messages.is_empty() {
        return;
    }

    let level_defn = get_level_defn(&level_state.lid);

    let hiscore = save_data
        .get()
        .map
        .get(&level_state.lid)
        .cloned()
        .unwrap_or_default()
        .hiscore;
    let hiscore_str = hiscore
        .map(|num| format!("{num}"))
        .unwrap_or("NONE".to_string());

    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(Color32::from_rgba_premultiplied(
            EGC8.r(),
            EGC8.g(),
            EGC8.b(),
            220,
        )))
        .show(ctx, |ui| {
            let force_width = 400.0;
            ui.vertical_centered(|ui| {
                ui.set_min_width(force_width);
                ui.set_max_width(force_width);
                let vspacing = ui.available_height() / 24.0;
                ui.style_mut().visuals.override_text_color = Some(EGC1);
                ui.add_space(vspacing);
                ui.heading("PAUSED");
                ui.small(format!("HI: {hiscore_str}"));
                ui.add_space(vspacing);
                ui.vertical(|ui| {
                    render_tier_grid(
                        ui,
                        force_width,
                        &level_defn.tiers,
                        Some(level_state.score),
                        &egui_textures,
                    );
                });
                ui.add_space(vspacing);
                ui.style_mut().visuals.override_text_color = Some(EGC8);
                ui.add_space(vspacing);
                if ui
                    .add_sized(control_butt_size(), egui::Button::new("RETRY"))
                    .clicked()
                {
                    commands.spawn(SoundEffect::MenuClick);
                    commands.trigger(
                        LoadLevel::lid(level_state.lid.clone()).with_skip_intro_messages(true),
                    );
                }
                ui.add_space(vspacing);
                if ui
                    .add_sized(control_butt_size(), egui::Button::new("MENU"))
                    .clicked()
                {
                    commands.spawn(SoundEffect::MenuClick);
                    commands.trigger(LoadMenu::kind(MenuKind::Levels));
                }
            });
        });
}

pub(super) fn register_overlay(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Level), on_enter_level);
    app.add_systems(
        Update,
        (update, show_intro_messages, pause_menu)
            .run_if(in_state(MetaState::Level))
            .after(egui_always_helpers),
    );
    // app.add_systems(OnExit(MetaState::Level), on_exit_level);
}
