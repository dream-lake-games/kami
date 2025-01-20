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

fn draw_hitboxes(
    srx_q: Query<(&Pos, &StaticRx)>,
    stx_q: Query<(&Pos, &StaticTx)>,
    trx_q: Query<(&Pos, &TriggerRx)>,
    ttx_q: Query<(&Pos, &TriggerTx)>,
    mut gz: Gizmos,
) {
    macro_rules! to_thbox_iter {
        ($thing:expr) => {
            $thing
                .iter()
                .flat_map(|(pos, ctrl)| ctrl.get_thboxes(pos.clone()))
        };
    }
    let full_iter = to_thbox_iter!(srx_q)
        .chain(to_thbox_iter!(stx_q))
        .chain(to_thbox_iter!(trx_q))
        .chain(to_thbox_iter!(ttx_q));
    for thbox in full_iter {
        gz.rect_2d(
            Isometry2d::from_translation(thbox.get_offset()),
            thbox.get_size().as_vec2(),
            Color::WHITE,
        );
    }
}

fn restart_level(
    keyboard: Res<ButtonInput<KeyCode>>,
    level_state: Option<Res<LevelState>>,
    meta_state: Res<State<MetaState>>,
    mut commands: Commands,
    existing_root: Query<Entity, With<LdtkProjectHandle>>,
    mut time_since_pressed: Local<Option<f32>>,
    time: Res<Time>,
) {
    let Some(level_state) = level_state else {
        return;
    };
    if !matches!(meta_state.get(), MetaState::Level) {
        return;
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        *time_since_pressed = Some(0.0);
        for eid in &existing_root {
            commands.entity(eid).despawn_recursive();
        }
    } else {
        match time_since_pressed.as_mut() {
            Some(thing) => {
                if *thing > 0.1 {
                    commands.trigger(LoadLevel::lid(level_state.lid.clone()));
                    *time_since_pressed = None;
                } else {
                    *thing += time.delta_secs();
                }
            }
            None => {}
        }
    }
}

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

        app.add_systems(
            Update,
            draw_hitboxes
                .after(PhysicsSet)
                .run_if(input_toggle_active(false, KeyCode::KeyH)),
        );

        app.add_systems(First, restart_level);
    }
}
