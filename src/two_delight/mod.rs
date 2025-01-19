//! The purpose of this mod is to set up all of my internal libraries correctly.
//! Hopefully makes organization more logic, and helps decouple from the underlying
//! impls if I need to change them

use crate::prelude::*;

pub mod two_delight_anims;
pub mod two_delight_physics;

pub use two_delight_anims::*;
pub use two_delight_physics::*;

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
                // mode: bevy::window::WindowMode::Windowed,
                mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Primary),
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
    }
}
