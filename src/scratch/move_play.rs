use std::ops::Mul;

use crate::{debug::debug_resource, prelude::*};

use super::Player;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;

#[derive(Resource, Reflect)]
struct MovePlayConsts {
    max_tran_vel: f32,
    max_ang_vel: f32,

    tilt_mul: f32,
    tilt_always_ratio: f32,

    lift_mul: f32,

    air_mul: f32,

    gravity: f32,

    gas_vel: f32,
    gas_bonus_ang_vel: f32,
    gas_bonus_ang_always_ratio: f32,

    disdain_for_down: f32,
}
impl Default for MovePlayConsts {
    fn default() -> Self {
        Self {
            max_tran_vel: 120.0,
            max_ang_vel: 2.0 * PI,

            tilt_mul: 8.0,
            tilt_always_ratio: 0.6,

            lift_mul: 4.0,

            air_mul: 200.0,

            gravity: 300.0,

            gas_vel: 800.0,
            gas_bonus_ang_vel: 4.0,
            gas_bonus_ang_always_ratio: 0.33,

            disdain_for_down: 0.1,
        }
    }
}

#[derive(Component)]
pub(super) struct Fly {
    pub ang: f32,
    pub ang_vel: f32,
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

    // Gas
    let gas = mouse.pressed(MouseButton::Right);
    if gas {
        let mut diff = vec_from_ang(fly.ang, consts.gas_vel * bullet_time.delta_secs());
        if diff.y < 0.0 {
            diff.y *= consts.disdain_for_down;
        }
        dyno.vel += diff;
        fly.ang +=
            consts.gas_bonus_ang_vel * consts.gas_bonus_ang_always_ratio * bullet_time.delta_secs();
        if fly.ang > PI {
            fly.ang -= fly.ang.sin()
                * consts.gas_bonus_ang_vel
                * bullet_time.delta_secs()
                * (1.0 - consts.gas_bonus_ang_always_ratio);
        }
    }

    // Gravity
    dyno.vel.y -= consts.gravity * bullet_time.delta_secs();

    // Air resistence (always happens, weird logically but feels good)
    let air_dir = vec_from_ang(fly.ang + PI / 2.0, 1.0);
    dyno.vel += air_dir * consts.air_mul * bullet_time.delta_secs();

    // Tilt
    let tilt_cos = fly.ang.cos();
    fly.ang_vel -= (tilt_cos.signum() * consts.tilt_always_ratio
        + tilt_cos * (1.0 - consts.tilt_always_ratio))
        * consts.tilt_mul
        * bullet_time.delta_secs();

    // Lift
    let lift_contrib = dyno
        .vel
        .dot(vec_from_ang(fly.ang, 1.0))
        .mul(0.1)
        .clamp(0.0, consts.lift_mul);
    fly.ang_vel += lift_contrib * bullet_time.delta_secs();

    fly.ang += fly.ang_vel * bullet_time.delta_secs();

    dyno.vel *= 0.99;
    fly.ang_vel *= 0.98;

    dyno.vel = dyno.vel.clamp_length(0.0, consts.max_tran_vel);
    fly.ang_vel = fly.ang_vel.clamp(-consts.max_ang_vel, consts.max_ang_vel);

    fly.ang = fly.ang.rem_euclid(TAU);
    tran.rotation = Quat::from_rotation_z(fly.ang);
}

pub(super) fn register_move_play(app: &mut App) {
    app.insert_resource(MovePlayConsts::default());
    debug_resource!(app, MovePlayConsts);

    app.add_systems(Update, fly.after(PhysicsSet));
}
