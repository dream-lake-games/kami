use crate::prelude::*;

#[derive(Component, Debug, Clone)]
#[component(on_add = on_add_level_meta)]
pub struct LevelMeta {
    bg_kind: BgKind,
}
impl MyLdtkEntity for LevelMeta {
    type Root = LevelMetaRoot;
    fn from_ldtk(_pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let bg_kind = BgKind::from_field_value(fields.get("BgKind").unwrap());
        Self { bg_kind }
    }
}
fn on_add_level_meta(mut world: DeferredWorld, eid: Entity, _: ComponentId) {
    let level_meta = world.get::<LevelMeta>(eid).cloned().unwrap();
    world.commands().trigger(SetupBg::kind(level_meta.bg_kind));
}

fn setup(mut commands: Commands, root: Res<TransitionRoot>) {
    commands
        .spawn((
            Name::new("LoadingAnim"),
            Transform::from_scale(Vec3::new(SCREEN_VEC.x / 64.0, SCREEN_VEC.y / 64.0, 1.0)),
            Visibility::default(),
            AnimMan::new(LoadingAnim::default()),
        ))
        .set_parent(root.eid());
}

fn on_enter_loading(mut commands: Commands, mut loading: Query<&mut AnimMan<LoadingAnim>>) {
    commands.trigger(LoadMyLdtkLevel::new(
        "worlds/base.ldtk",
        "6dab9440-c210-11ef-ab00-79b1690c4bfe",
    ));
    loading.single_mut().set_state(LoadingAnim::Dots);
}

fn update_loading(my_ldtk_load_state: Res<MyLdtkLoadState>, mut commands: Commands) {
    if !matches!(my_ldtk_load_state.into_inner(), MyLdtkLoadState::Loaded) {
        return;
    }
    commands.trigger(StartLevel);
}

fn on_exit_loading(mut loading: Query<&mut AnimMan<LoadingAnim>>) {
    loading.single_mut().set_state(LoadingAnim::None);
}

pub(super) fn register_level_loading(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<LevelMeta>::new("Meta", "LevelMeta"));

    app.add_systems(OnEnter(MetaState::Setup), setup.after(RootInit));

    app.add_systems(OnEnter(MetaState::LevelLoading), on_enter_loading);
    app.add_systems(
        Update,
        update_loading.run_if(in_state(MetaState::LevelLoading)),
    );
    app.add_systems(OnExit(MetaState::LevelLoading), on_exit_loading);
}
