use crate::prelude::*;

#[derive(Component, Default)]
#[require(RenderLayers(|| OverlayLayer::RENDER_LAYERS))]
struct Overlay;

// Name::new("text"),
// Text2d::new(text.content.clone()),
// TextFont::from_font(font_hand).with_font_size(42.0),
// TextLayout::new_with_justify(JustifyText::Center),
// TextBounds::new(
//     38.0 * TextLayer::growth_factor() as f32,
//     280.0 * TextLayer::growth_factor() as f32,
// ),
// Anchor::CenterLeft,
// Transform::from_translation(
//     text.induced_text_offset(&size).extend(ZIX_CONVO_TEXT),
// ),

#[derive(Component)]
#[require(Overlay)]
struct TextOverlay;
#[derive(Bundle)]
struct TextOverlayBundle {
    name: Name,
    text_overlay: TextOverlay,
    pos: Pos,
    text2d: Text2d,
    font: TextFont,
    layout: TextLayout,
    anchor: Anchor,
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
            text2d: Text2d::new(text.to_string()),
            font: TextFont::from_font(font_hand).with_font_size(12.0),
            layout: TextLayout::new_with_justify(justify),
            anchor,
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
            ass.load("fonts/Jersey15-Regular.ttf"),
            JustifyText::Left,
            Anchor::TopLeft,
        ))
        .set_parent(root.eid());
    commands
        .spawn(TextOverlayBundle::new(
            "TextScore",
            "SCORE: 0",
            Pos::new(SCREEN_VEC.x / 2.0 - off_corner, SCREEN_VEC.y / 2.0),
            ass.load("fonts/Jersey15-Regular.ttf"),
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

// fn on_exit_level(mut commands: Commands, overlays: Query<Entity, With<Overlay>>) {
//     for eid in &overlays {
//         commands.entity(eid).despawn_recursive();
//     }
// }

pub(super) fn register_overlay(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Level), on_enter_level);
    app.add_systems(Update, update.run_if(in_state(MetaState::Level)));
    // app.add_systems(OnExit(MetaState::Level), on_exit_level);
}
