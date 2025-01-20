//! The purpose of this mod is to set up all of my internal libraries correctly.
//! Hopefully makes organization more logic, and helps decouple from the underlying
//! impls if I need to change them

use crate::{debug::debug_resource, prelude::*};

pub mod two_delight_anims;
pub mod two_delight_physics;

pub use two_delight_anims::*;
pub use two_delight_physics::*;

#[derive(Resource, Reflect)]
struct Framepace {
    fps_limit: f32,
    active: bool,
}
impl Default for Framepace {
    fn default() -> Self {
        Framepace {
            fps_limit: 60.0,
            active: true,
        }
    }
}

fn update_framepace(my: Res<Framepace>, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    if my.active {
        settings.limiter =
            bevy_framepace::Limiter::Manual(Duration::from_secs_f32(1.0 / my.fps_limit));
    } else {
        settings.limiter = bevy_framepace::Limiter::Auto;
    }
}

/// Plugin that wraps all my other plugins lol.
pub(super) struct TwoDelightPlugin;
impl Plugin for TwoDelightPlugin {
    fn build(&self, app: &mut App) {
        // Layers
        // NOTE: Has to happen first since this adds the default plugins, which include stuff the other ones need
        app.add_plugins(LayersPlugin {
            screen_size: SCREEN_UVEC,
            overlay_growth: OVERLAY_GROWTH,
            window: Window {
                resizable: true,
                title: "Pirate Jam 16".to_string(),
                resolution: bevy::window::WindowResolution::new(OVERLAY_VEC.x, OVERLAY_VEC.y),
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            },
            asset_plugin: AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            },
        });

        // Anims
        app.insert_resource(AnimTimeRes::default());
        app.add_plugins(
            AnimPlugin::<AnimTimeRes>::default()
                .with_default_fps(DEFAULT_ANIM_FPS)
                .with_default_render_layers(DEFAULT_ANIM_RENDER_LAYERS)
                .with_default_time_class(DEFAULT_ANIM_TIME_CLASS),
        );
        app.add_systems(PostUpdate, drive_anim_time_res.before(AnimSet));

        // Physics
        app.add_plugins(PhysicsPlugin::default());

        // Fuck it shoving framepacing in here too
        app.insert_resource(Framepace::default());
        debug_resource!(app, Framepace);
        app.add_plugins(bevy_framepace::FramepacePlugin);
        app.add_systems(Update, update_framepace);
    }
}
