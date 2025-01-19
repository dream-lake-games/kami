use core::f32;

use move_play::Fly;

use crate::prelude::*;

mod move_play;

#[derive(Component)]
struct Player;

#[derive(Component)]
#[require(Name(|| Name::new("Ground")))]
struct Ground;
#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
    pos: Pos,
    sprite: Sprite,
    static_tx: StaticTx,
    rl: RenderLayers,
}
impl GroundBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            ground: Ground,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                ..default()
            },
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(size.x, size.y)),
            rl: MainStaticLayer::RENDER_LAYERS,
        }
    }
}

#[derive(Component)]
#[require(Name(|| Name::new("Spike")))]
struct Spike;
#[derive(Bundle)]
struct SpikeBundle {
    spike: Spike,
    pos: Pos,
    sprite: Sprite,
    trigger_tx: TriggerTx,
}
impl SpikeBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            spike: Spike,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            trigger_tx: TriggerTx::single(TriggerTxKind::Spikes, HBox::new(size.x, size.y)),
        }
    }
}

derive_anim!(
    pub enum Light64Anim {
        #[default]
        #[file("scratch/platformer/light_on.png")]
        #[size(64, 64)]
        On,
        #[file("scratch/platformer/light_off.png")]
        #[size(64, 64)]
        Off,
    }
);
impl LightAnim for Light64Anim {
    fn light_radius(&self) -> Option<f32> {
        match self {
            Self::On => Some(32.0),
            Self::Off => None,
        }
    }
}
pub(super) type Light64Plugin = LightDefnPlugin<Light64Anim>;

fn startup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(DynamicCamera);

    let player_hbox = HBox::new(16, 8);
    commands.spawn((
        Name::new("Player"),
        Player,
        Fly {
            ang: 0.0,
            ang_vel: 0.0,
        },
        Sprite {
            custom_size: Some(player_hbox.get_size().as_vec2()),
            color: Color::linear_rgb(0.1, 1.0, 0.1),
            ..default()
        },
        Pos::new(0.0, 0.0),
        Dyno::new(0.0, 0.0),
        StaticRx::single(StaticRxKind::Default, player_hbox.clone()),
        TriggerRx::single(TriggerRxKind::Player, player_hbox.clone()),
        MainStaticLayer::RENDER_LAYERS,
        LightMan::new(Light64Anim::On),
    ));

    // commands.spawn(GroundBundle::new(
    //     Pos::new(0.0, -SCREEN_VEC.y / 2.0),
    //     UVec2::new(SCREEN_UVEC.x, 24),
    // ));
    commands.spawn(GroundBundle::new(
        Pos::new(-SCREEN_VEC.x / 2.0, 0.0),
        UVec2::new(SCREEN_UVEC.x / 2, 12),
    ));
    commands.spawn(GroundBundle::new(
        Pos::new(SCREEN_VEC.x / 2.0, 0.0),
        UVec2::new(SCREEN_UVEC.x / 2, 12),
    ));

    commands.spawn((
        SpikeBundle::new(Pos::new(-SCREEN_VEC.x / 2.0, 18.0), UVec2::new(36, 24)),
        MainAmbienceLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        SpikeBundle::new(Pos::new(SCREEN_VEC.x / 2.0, 18.0), UVec2::new(36, 24)),
        MainDetailLayer::RENDER_LAYERS,
    ));

    commands.spawn((
        Name::new("Bg"),
        Sprite {
            image: ass.load("scratch/platformer/bg.png"),
            custom_size: Some(SCREEN_VEC),
            ..default()
        },
        BgLayer::RENDER_LAYERS,
        ParallaxX::new_wrapped(0.3, SCREEN_VEC.x),
        ParallaxY::new_wrapped(0.3, SCREEN_VEC.y),
    ));
    commands.spawn((
        Name::new("Fg"),
        Sprite {
            image: ass.load("scratch/platformer/fg.png"),
            custom_size: Some(Vec2::new(SCREEN_VEC.x * 3.0, SCREEN_VEC.y)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        FgLayer::RENDER_LAYERS,
        ParallaxX::new_wrapped(1.2, SCREEN_VEC.x * 2.0),
    ));

    commands.spawn((
        Name::new("MoreAmbience"),
        Transform::from_translation(Vec3::Z * -5.0),
        Sprite {
            color: Color::linear_rgb(0.4, 0.4, 0.5),
            custom_size: Some(Vec2::new(120.0, 360.0)),
            ..default()
        },
        MainAmbienceLayer::RENDER_LAYERS,
    ));

    commands.spawn((
        Name::new("OverlayText"),
        Text2d::new("Overlay Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 12.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        OverlayLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        Name::new("MenuText"),
        Text2d::new("Menu Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 24.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        MenuLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        Name::new("TransitionText"),
        Text2d::new("Transition Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 36.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TransitionLayer::RENDER_LAYERS,
    ));
}

fn camera_follow_player(
    mut camera_q: Query<&mut Pos, (With<DynamicCamera>, Without<Player>)>,
    player_q: Query<&Pos, (Without<DynamicCamera>, With<Player>)>,
) {
    let mut cam_pos = camera_q.single_mut();
    let player_pos = player_q.single();
    *cam_pos = player_pos.clone();
}

fn shake_big_collisions(
    static_colls: Res<StaticColls>,
    player_q: Query<&StaticRx, With<Player>>,
    mut shake: ResMut<CameraShake>,
) {
    let player_srx = player_q.single();
    if static_colls
        .get_refs(&player_srx.coll_keys)
        .iter()
        .any(|coll| coll.rx_perp.length_squared() > 500.0)
    {
        shake.add_shake(0.1, -1..=1, -2..=2);
    }
}

fn physics_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut bullet_time: ResMut<BulletTime>,
    mut player_q: Query<&mut Pos, With<Player>>,
) {
    // Maybe toggle bullet time
    if keyboard.just_pressed(KeyCode::Space) {
        let new_speed = bullet_time.get_base().rotated();
        bullet_time.set_base(new_speed);
    }
    let mut pos = player_q.single_mut();

    if keyboard.just_pressed(KeyCode::Backspace) {
        *pos = Pos::default();
    }
}

pub(super) struct ScratchPlugin;
impl Plugin for ScratchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light64Plugin::default());

        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            (physics_update, camera_follow_player, shake_big_collisions)
                .after(PhysicsSet)
                .before(LayersCameraSet),
        );

        move_play::register_move_play(app);
    }
}
