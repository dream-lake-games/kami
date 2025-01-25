use crate::prelude::*;

/// The default FPS of animations, when unspecified
pub const DEFAULT_ANIM_FPS: f32 = 16.0;
/// The default render layer, when unspecified
pub const DEFAULT_ANIM_RENDER_LAYERS: RenderLayers = MainStaticLayer::RENDER_LAYERS;
/// The default time class, when unspecified
pub const DEFAULT_ANIM_TIME_CLASS: AnimTimeClass = ANIM_TIME_BULLET;

/// Animations that play only when physics is active, beholden to bullet time
pub const ANIM_TIME_BULLET: AnimTimeClass = 0;
/// Animations that play only when physics is active, beholden to real time
pub const ANIM_TIME_REAL: AnimTimeClass = 1;
/// Animations that always play, beholden to bullet time
pub const ANIM_TIME_BULLET_ALWAYS: AnimTimeClass = 2;
/// Animations that always play, beholden to real time
pub const ANIM_TIME_REAL_ALWAYS: AnimTimeClass = 2;

#[derive(Resource, Clone, Debug, Default, Reflect)]
pub struct AnimTimeRes {
    class_map: HashMap<AnimTimeClass, f32>,
}
impl AnimTimeProvider for AnimTimeRes {
    fn get_delta(&self, class: AnimTimeClass) -> f32 {
        *self.class_map.get(&class).unwrap_or(&0.0)
    }
}

#[derive(Component)]
pub(super) struct AgeBack;
#[derive(Bundle)]
pub struct EphemeralAnim<StateMachine: AnimStateMachine> {
    name: Name,
    anim: AnimMan<StateMachine>,
    transform: Transform,
    visibility: Visibility,
    age_back: AgeBack,
}
impl<StateMachine: AnimStateMachine> EphemeralAnim<StateMachine> {
    pub fn new(anim: StateMachine, flip_x: bool, pos: Pos, zix: f32) -> Self {
        Self {
            name: Name::new("ephemeral anim"),
            anim: AnimMan::new(anim).with_flip_x(flip_x),
            transform: pos.to_transform(zix),
            visibility: Visibility::Visible,
            age_back: AgeBack,
        }
    }
}

pub(super) fn age_ephemeral_anims_back(
    mut anims: Query<&mut Transform, With<AgeBack>>,
    bullet_time: Res<BulletTime>,
) {
    for mut tran in &mut anims {
        tran.translation.z -= bullet_time.delta_secs() * 0.1;
    }
}

pub(super) fn drive_anim_time_res(
    mut anim_time: ResMut<AnimTimeRes>,
    bullet_time: Res<BulletTime>,
    time: Res<Time>,
    // TODO: paused: Res<State<PauseState>>,
) {
    let paused_delta = 1.0;

    anim_time
        .class_map
        .insert(ANIM_TIME_BULLET, paused_delta * bullet_time.delta_secs());
    anim_time
        .class_map
        .insert(ANIM_TIME_REAL, paused_delta * time.delta_secs());
    anim_time
        .class_map
        .insert(ANIM_TIME_BULLET_ALWAYS, bullet_time.delta_secs());
    anim_time
        .class_map
        .insert(ANIM_TIME_REAL_ALWAYS, time.delta_secs());
}
