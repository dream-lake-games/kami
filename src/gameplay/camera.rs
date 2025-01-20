use crate::prelude::*;

fn setup(root: Res<CameraRoot>, mut commands: Commands) {
    commands
        .spawn((Name::new("DynamicCamera"), DynamicCamera))
        .set_parent(root.eid());
}

fn keep_inside_level(
    mut cam_q: Query<&mut Pos, With<DynamicCamera>>,
    level_state: Res<LevelState>,
) {
    let mut cam_pos = cam_q.single_mut();
    let at_least_x = level_state.rect.min.x + SCREEN_VEC.x / 2.0;
    let at_most_x = level_state.rect.max.x - SCREEN_VEC.x / 2.0;
    let at_least_y = level_state.rect.min.y + SCREEN_VEC.y / 2.0;
    let at_most_y = level_state.rect.max.y - SCREEN_VEC.y / 2.0;
    // Clamp might not work bc panics when level is exact right size... ugh...
    cam_pos.x = cam_pos.x.max(at_least_x).min(at_most_x);
    cam_pos.y = cam_pos.y.max(at_least_y).min(at_most_y);
}

pub(super) fn register_camera(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Setup), setup.after(RootInit));

    app.add_systems(Update, keep_inside_level.run_if(in_state(MetaState::Level)));
}
