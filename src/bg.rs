use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum BgKind {
    Sky,
}
impl BgKind {
    pub fn from_field_value(fv: &FieldValue) -> Self {
        let FieldValue::Enum(Some(inner)) = fv else {
            panic!("Bad bg kind1: {:?}", fv);
        };
        match inner.as_str() {
            "Sky" => Self::Sky,
            _ => panic!("Bad bg kind2: {inner}"),
        }
    }
}

#[derive(Bundle)]
pub struct BgParallaxBundle {
    name: Name,
    sprite: Sprite,
    transform: Transform,
    render_layers: RenderLayers,
    parallax_x: ParallaxX,
    pos: Pos,
    dyno: Dyno,
}
impl BgParallaxBundle {
    pub fn new(name: &str, sprite: Sprite, parallax: f32, zix: f32, vel: f32) -> Self {
        Self {
            name: Name::new(name.to_string()),
            sprite,
            transform: Transform::from_translation(Vec2::ZERO.extend(zix)),
            render_layers: BgLayer::RENDER_LAYERS,
            parallax_x: ParallaxX::new_wrapped(parallax, 2.0 * SCREEN_VEC.x),
            pos: Pos::new(thread_rng().gen_range(0.0..1000.0), 0.0),
            dyno: Dyno::new(vel, 0.0),
        }
    }
}

#[derive(Event)]
pub struct SetupBg {
    kind: BgKind,
}
impl SetupBg {
    pub fn kind(kind: BgKind) -> Self {
        Self { kind }
    }
}
fn handle_setup_bg(
    trigger: Trigger<SetupBg>,
    ass: Res<AssetServer>,
    root: Res<LevelBgRoot>,
    mut commands: Commands,
) {
    let make_px_sprite = |path: &'static str| Sprite {
        image: ass.load(path),
        custom_size: Some(Vec2::new(3.0 * SCREEN_VEC.x, SCREEN_VEC.y)),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.0,
        },
        ..default()
    };

    let kind = trigger.kind.clone();
    match kind {
        BgKind::Sky => {
            let cloud_speed = 6.0;

            let sky_bund = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/sky.png"),
                0.0,
                0.0,
                0.0,
            );

            let clouds_bot = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/clouds_bot.png"),
                0.08,
                0.1,
                cloud_speed,
            );

            let mountain_1 = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/mountain_1.png"),
                0.12,
                0.2,
                0.0,
            );

            let clouds_mid = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/clouds_mid.png"),
                0.16,
                0.3,
                cloud_speed,
            );

            let mountain_2 = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/mountain_2.png"),
                0.24,
                0.4,
                0.0,
            );

            let clouds_top = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/clouds_top.png"),
                0.4,
                0.5,
                cloud_speed,
            );

            let mountain_trees = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/mountain_trees.png"),
                0.6,
                0.6,
                0.0,
            );

            let mountain_hills = BgParallaxBundle::new(
                "Sky",
                make_px_sprite("environment/bg/mountain_hills.png"),
                0.9,
                0.7,
                0.0,
            );

            for bund in [
                sky_bund,
                clouds_bot,
                mountain_1,
                clouds_mid,
                mountain_2,
                clouds_top,
                mountain_trees,
                mountain_hills,
            ] {
                commands
                    // Wait some ticks before saying were done
                    .spawn((bund, BlockMyLdtkLoad::ticks(30)))
                    .set_parent(root.eid());
            }
        }
    }
}

pub(super) struct BgPlugin;
impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_setup_bg);
    }
}
