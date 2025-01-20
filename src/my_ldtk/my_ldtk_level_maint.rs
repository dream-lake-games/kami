use crate::prelude::*;

/// The set that contains all ldtk level maintainence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct MyLdtkLevelMaint;

/// Updates level rect
fn update_my_level_rect(
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_state: ResMut<LevelState>,
) {
    let Ok(project) = ldtk_projects.get_single() else {
        return;
    };
    let Some(ldtk_project) = ldtk_project_assets.get(project) else {
        return;
    };
    for (level_lid, level_transform) in levels.iter() {
        if level_lid.as_str() != &level_state.lid {
            continue;
        }
        let level = ldtk_project
            .get_raw_level_by_iid(level_lid.get())
            .expect("level should exist in only project");
        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        level_state.rect = level_bounds;
    }
}

pub(super) fn register_my_ldtk_level_maint(app: &mut App) {
    app.add_systems(Update, update_my_level_rect.in_set(MyLdtkLevelMaint));
}
