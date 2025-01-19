use crate::prelude::*;

#[derive(Resource, Default)]
pub struct LevelState {
    pub lid: String,
    pub paused: bool,
    pub rect: Rect,
}

fn on_enter_loading(mut commands: Commands) {
    commands.trigger(LoadMyLdtkLevel::new(
        "worlds/base.ldtk",
        "6dab9440-c210-11ef-ab00-79b1690c4bfe",
    ));
}

fn update_loading(my_ldtk_load_state: Res<MyLdtkLoadState>, mut commands: Commands) {
    if !matches!(my_ldtk_load_state.into_inner(), MyLdtkLoadState::Loaded) {
        return;
    }
    commands.trigger(StartLevel);
}

#[derive(Bundle)]
struct ABundle {}
impl MyLdtkIntCellValue for ABundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {}
    }
}

pub(super) struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        MyLdtkIntCellLayer::new("A", MainStaticLayer).register(app);
        MyLdtkIntCellLayer::new("ADetail", MainDetailLayer).register(app);
        app.add_plugins(MyLdtkIntCellValuePlugin::<ABundle>::single("A", 1));

        app.insert_resource(LevelState::default());

        app.add_systems(OnEnter(MetaState::LevelLoading), on_enter_loading);
        app.add_systems(
            Update,
            update_loading.run_if(in_state(MetaState::LevelLoading)),
        );
    }
}
