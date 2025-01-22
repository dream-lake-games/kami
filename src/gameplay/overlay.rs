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

fn show_intro_messages(mut level_state: ResMut<LevelState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    if level_state.intro_messages.is_empty() {
        return;
    }
    let msg = level_state.intro_messages[0].clone();
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(EGC8))
        .show(ctx, |ui| {
            ui.style_mut().visuals.override_text_color = Some(EGC1);
            let fh = ui.available_height() / 4.0;
            ui.set_min_width(600.0);
            ui.set_max_width(600.0);
            ui.add_space(fh);
            ui.vertical_centered(|ui| {
                ui.set_min_height(fh * 2.0);
                ui.set_max_height(fh * 2.0);
                ui.heading(&msg.title);
                ui.label(&msg.content);
            });
            ui.vertical_centered(|ui| {
                ui.style_mut().visuals.override_text_color = Some(EGC8);
                if ui.button("CONTINUE").clicked() {
                    level_state.intro_messages.remove(0);
                }
            });
        });
}

pub(super) fn register_overlay(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Level), on_enter_level);
    app.add_systems(
        Update,
        (update, show_intro_messages)
            .run_if(in_state(MetaState::Level))
            .after(egui_always_helpers),
    );
    // app.add_systems(OnExit(MetaState::Level), on_exit_level);
}
