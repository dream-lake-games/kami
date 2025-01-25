use crate::prelude::*;

// mod my_ldtk_consolidate;
mod my_ldtk_entity;
mod my_ldtk_int_cell;
mod my_ldtk_level_maint;
mod my_ldtk_load;

// pub use my_ldtk_consolidate::*;
pub use my_ldtk_entity::*;
pub use my_ldtk_int_cell::*;
pub use my_ldtk_load::*;

/// The set that contains all weird ldtk maintenence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyLdtkMaintSet;

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        // my_ldtk_consolidate::register_my_ldtk_consolidate(app);
        my_ldtk_int_cell::register_my_ldtk_int_cell(app);
        my_ldtk_level_maint::register_my_ldtk_level_maint(app);
        my_ldtk_load::register_my_ldtk_load(app);

        app.add_plugins(LdtkPlugin).insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
            level_background: LevelBackground::Nonexistent,
            ..default()
        });
    }
}
