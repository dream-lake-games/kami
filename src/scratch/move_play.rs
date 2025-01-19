use crate::{debug::debug_resource, prelude::*};

use super::Player;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;

#[derive(Resource, Reflect)]
struct MovePlayConsts {
    max_tran_speed: f32,
    gravity: f32,
    x_drag: f32,
    up_mul: f32,
}
impl Default for MovePlayConsts {
    fn default() -> Self {
        Self {
            max_tran_speed: 150.0,
            gravity: 150.0,
            x_drag: 20.0,
            up_mul: 6.0,
        }
    }
}

#[derive(Component)]
pub(super) struct Fly {
    pub ang: f32,
    pub flap: Option<f32>,
}

fn vec_from_ang(ang: f32, len: f32) -> Vec2 {
    let c = ang.cos();
    let s = ang.sin();
    Vec2::new(c * len, s * len)
}

fn fly(
    bullet_time: Res<BulletTime>,
    mut player_q: Query<(&mut Transform, &mut Dyno, &mut Fly), With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
    consts: Res<MovePlayConsts>,
) {
    let (mut tran, mut dyno, mut fly) = player_q.single_mut();

    // Gravity
    dyno.vel.y -= consts.gravity * bullet_time.delta_secs();

    // Some drag
    dyno.vel.x -= dyno.vel.x.signum() * consts.x_drag * bullet_time.delta_secs();

    // Update flap info
    if mouse.just_pressed(MouseButton::Right) {
        fly.flap = Some(if dyno.vel.x >= 0.0 {
            PI / 2.0
        } else {
            -PI / 2.0
        });
    }
    if !mouse.pressed(MouseButton::Right) {
        fly.flap = None;
    }

    fly.ang = dyno.vel.to_angle();
    if let Some(ang_diff) = fly.flap {
        let up = vec_from_ang(fly.ang + ang_diff, 1.0);
        let diff = up * dyno.vel.length() * consts.up_mul * bullet_time.delta_secs();
        dyno.vel += diff;
    }

    // // Air resistence (always happens, weird logically but feels good)
    // let air_dir = vec_from_ang(fly.ang + PI / 2.0, 1.0);
    // dyno.vel += air_dir * consts.air_mul * bullet_time.delta_secs();

    // // Tilt
    // let tilt_cos = fly.ang.cos();
    // fly.ang_vel -= (tilt_cos.signum() * consts.tilt_always_ratio
    //     + tilt_cos * (1.0 - consts.tilt_always_ratio))
    //     * consts.tilt_mul
    //     * bullet_time.delta_secs();

    // // Lift
    // let lift_contrib = dyno
    //     .vel
    //     .dot(vec_from_ang(fly.ang, 1.0))
    //     .mul(0.1)
    //     .clamp(0.0, consts.lift_mul);
    // fly.ang_vel += lift_contrib * bullet_time.delta_secs();

    // fly.ang += fly.ang_vel * bullet_time.delta_secs();

    dyno.vel = dyno.vel.clamp_length(0.0, consts.max_tran_speed);

    fly.ang = fly.ang.rem_euclid(TAU);
    tran.rotation = Quat::from_rotation_z(fly.ang);
}

pub(super) fn register_move_play(app: &mut App) {
    app.insert_resource(MovePlayConsts::default());
    debug_resource!(app, MovePlayConsts);

    app.add_systems(Update, fly.after(PhysicsSet));
}
