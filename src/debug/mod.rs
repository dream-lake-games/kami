use crate::prelude::*;

macro_rules! debug_resource {
    ($app:expr, $resource:ty) => {{
        #[cfg(debug_assertions)]
        {
            $app.add_plugins(
                bevy_inspector_egui::quick::ResourceInspectorPlugin::<$resource>::new()
                    .run_if(input_toggle_active(false, KeyCode::Tab)),
            );
        }
    }};
}
pub(crate) use debug_resource;

/// Everything we use to debug the app
/// NOTE: Assumed that this is configured out at a higher level. That is, this plugin is only
///       added if we know that we want debug stuff.
pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            #[cfg(not(target_arch = "wasm32"))]
            {
                bevy_inspector_egui::quick::WorldInspectorPlugin::default()
                    .run_if(input_toggle_active(false, KeyCode::Tab))
            },
        ));
    }
}
