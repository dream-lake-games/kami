use crate::{debug::debug_resource, prelude::*};

use super::Player;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;

#[derive(Resource, Reflect)]
struct MovePlayConsts {
    gravity_y_always: f32,
    gravity_max_y: f32,
    gravity_max_x: f32,
    gas_tran_vel: f32,
    gas_ang_vel: f32,
    gas_x_favor: f32,
    gas_down_disfavor: f32,
    max_tran_vel: f32,

    goal_ang: f32,
}
impl Default for MovePlayConsts {
    fn default() -> Self {
        Self {
            gravity_y_always: 50.0,
            gravity_max_y: 150.0,
            gravity_max_x: 75.0,
            gas_tran_vel: 700.0,
            gas_ang_vel: PI * 0.75,
            gas_x_favor: 1.2,
            gas_down_disfavor: 0.2,
            max_tran_vel: 200.0,

            goal_ang: PI * 7.0 / 4.0,
        }
    }
}

#[derive(Component)]
pub(super) struct Fly {
    pub ang: f32,
    pub ang_vel: f32,
}

/// Mathy boi
fn quadrant(ang: f32) -> u32 {
    if 0.0 <= ang && ang < PI / 2.0 {
        1
    } else if PI / 2.0 <= ang && ang < PI {
        2
    } else if PI <= ang && ang < 3.0 * PI / 2.0 {
        3
    } else {
        4
    }
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

    let gas = mouse.pressed(MouseButton::Right);
    let current_quadrant = quadrant(fly.ang);

    // First handle gravity
    let diff = Vec2::new(0.0, -consts.gravity_y_always)
        + match current_quadrant {
            1 => {
                let two_ang = fly.ang * 2.0;
                Vec2::new(
                    -two_ang.sin() * consts.gravity_max_x,
                    -fly.ang.sin() * consts.gravity_max_y,
                )
            }
            2 => {
                let two_ang = (fly.ang - PI / 2.0) * 2.0;
                Vec2::new(
                    two_ang.sin() * consts.gravity_max_x,
                    -fly.ang.sin() * consts.gravity_max_y,
                )
            }
            3 => {
                let two_ang = (fly.ang - PI) * 2.0;
                Vec2::new(
                    -two_ang.sin() * consts.gravity_max_x,
                    fly.ang.sin() * consts.gravity_max_y,
                )
            }
            4 => {
                let two_ang = (fly.ang - PI - PI / 2.0) * 2.0;
                Vec2::new(
                    two_ang.sin() * consts.gravity_max_x,
                    fly.ang.sin() * consts.gravity_max_y,
                )
            }
            _ => unreachable!(),
        };
    let diff = diff * bullet_time.delta_secs();
    dyno.vel += diff;

    // Then input
    if gas {
        let mut raw_vec = vec_from_ang(fly.ang, 1.0);
        raw_vec.x *= consts.gas_x_favor;
        if raw_vec.y < 0.0 {
            raw_vec.y *= consts.gas_down_disfavor;
        }
        dyno.vel += raw_vec * bullet_time.delta_secs() * consts.gas_tran_vel;
        fly.ang += consts.gas_ang_vel * bullet_time.delta_secs();
    } else {
        match current_quadrant {
            1 => {
                fly.ang -= bullet_time.delta_secs() * PI * 0.4;
            }
            4 => {
                if fly.ang < PI * 7.0 / 4.0 {
                    fly.ang += bullet_time.delta_secs() * PI * 0.4;
                } else {
                    fly.ang -= bullet_time.delta_secs() * PI * 0.4;
                }
            }
            _ => {
                fly.ang += bullet_time.delta_secs() * PI * 0.4;
            }
        }
    }

    // maint
    fly.ang = fly.ang.rem_euclid(TAU);
    dyno.vel.x *= 0.999;
    dyno.vel = dyno.vel.clamp_length(0.0, consts.max_tran_vel);

    tran.rotation = Quat::from_rotation_z(fly.ang);
}

pub(super) fn register_move_play(app: &mut App) {
    app.insert_resource(MovePlayConsts::default());
    debug_resource!(app, MovePlayConsts);

    app.add_systems(Update, fly.after(PhysicsSet));
}
