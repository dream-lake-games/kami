use crate::prelude::*;

#[derive(Resource, Clone, Debug, Default, Reflect, PartialEq, Eq)]
pub enum MyLdtkLoadState {
    #[default]
    Unloaded,
    Loading,
    Loaded,
}

#[derive(Event)]
pub struct LoadMyLdtkLevel {
    world_path: String,
    level_lid: String,
}
impl LoadMyLdtkLevel {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(world_path: S1, level_lid: S2) -> Self {
        Self {
            world_path: world_path.as_ref().to_string(),
            level_lid: level_lid.as_ref().to_string(),
        }
    }
}
#[derive(Event)]
pub struct UnloadMyLdtkLevel;

fn handle_load_my_ldtk_level(
    trigger: Trigger<LoadMyLdtkLevel>,
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
    root: Res<LevelMetaRoot>,
) {
    commands.spawn((
        Name::new("MyLdtkRoot"),
        LdtkWorldBundle {
            ldtk_handle: LdtkProjectHandle {
                handle: ass.load(&trigger.event().world_path),
            },
            ..default()
        },
    ));
    commands
        .spawn((Name::new("RogueBlockLoad"), BlockMyLdtkLoad::ticks(30)))
        .set_parent(root.eid());
    commands.insert_resource(LevelSelection::iid(&trigger.event().level_lid));
    *my_ldtk_load_state = MyLdtkLoadState::Loading;
}

fn handle_unload_my_ldtk_level(
    _trigger: Trigger<UnloadMyLdtkLevel>,
    mut commands: Commands,
    existing_root: Query<(Entity, &LdtkProjectHandle)>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
) {
    for (eid, _hand) in &existing_root {
        commands.entity(eid).despawn_recursive();
    }
    *my_ldtk_load_state = MyLdtkLoadState::Unloaded;
    commands.remove_resource::<LevelSelection>();
}

fn is_loading(res: Res<MyLdtkLoadState>) -> bool {
    res.into_inner() == &MyLdtkLoadState::Loading
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct BlockMyLdtkLoad {
    pub ticks: u32,
}
impl BlockMyLdtkLoad {
    pub fn ticks(ticks: u32) -> Self {
        Self { ticks }
    }
}

fn handle_loading(
    mut commands: Commands,
    mut blockers: Query<(Entity, &mut BlockMyLdtkLoad)>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
) {
    // Check for explicit blockers
    if !blockers.is_empty() {
        for (eid, mut blocker) in &mut blockers {
            if blocker.ticks == 0 {
                commands.entity(eid).remove::<BlockMyLdtkLoad>();
            } else {
                blocker.ticks -= 1;
            }
        }
        return;
    }
    *my_ldtk_load_state = MyLdtkLoadState::Loaded;
}

pub(super) fn register_my_ldtk_load(app: &mut App) {
    app.insert_resource(MyLdtkLoadState::default());

    app.add_observer(handle_load_my_ldtk_level);
    app.add_observer(handle_unload_my_ldtk_level);

    app.add_systems(Update, handle_loading.run_if(is_loading));
}
