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
    render_layers: RenderLayers,
    parallax_x: ParallaxX,
}
impl BgParallaxBundle {
    pub fn new(name: &str, sprite: Sprite, parallax: f32) -> Self {
        Self {
            name: Name::new(name.to_string()),
            sprite,
            render_layers: BgLayer::RENDER_LAYERS,
            parallax_x: ParallaxX::new_wrapped(parallax, 2.0 * SCREEN_VEC.x),
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
    let kind = trigger.kind.clone();
    // Wait some ticks before saying were done
    commands.spawn(BlockMyLdtkLoad::ticks(30));
    match kind {
        BgKind::Sky => {
            let sprite = Sprite {
                image: ass.load("environment/bg/sky.png"),
                custom_size: Some(Vec2::new(3.0 * SCREEN_VEC.x, SCREEN_VEC.y)),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0,
                },
                ..default()
            };
            let bundle = BgParallaxBundle::new("Sky", sprite, 0.3);
            commands.spawn(bundle).set_parent(root.eid());
        }
    }
}

pub(super) struct BgPlugin;
impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_setup_bg);
    }
}
