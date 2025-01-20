use crate::{debug::debug_resource, prelude::*};

#[derive(Resource, Reflect)]
struct ChefConsts {
    min_launch_speed: f32,
    max_launch_speed: f32,
    gravity: f32,
    max_speed: f32,
    stop_speed_cutoff: f32,
    stop_sleep_time: f32,
    dirt_par_mul: f32,
    dirt_perp_mul: f32,
    dirt_slide_speed: f32,
    smooth_par_mul: f32,
    smooth_perp_mul: f32,
    smooth_slide_speed: f32,
    air_drag_speed: f32,
    lift_mul: f32,
}
impl Default for ChefConsts {
    fn default() -> Self {
        Self {
            min_launch_speed: 5.0,
            max_launch_speed: 100.0,
            gravity: 100.0,
            max_speed: 300.0,
            stop_speed_cutoff: 8.0,
            stop_sleep_time: 0.5,
            dirt_par_mul: 0.7,
            dirt_perp_mul: 0.4,
            dirt_slide_speed: 100.0,
            smooth_par_mul: 0.95,
            smooth_perp_mul: 0.2,
            smooth_slide_speed: 10.0,
            air_drag_speed: 2.0,
            lift_mul: 5.0,
        }
    }
}

struct CurrentLift {
    ang_diff: f32,
}

#[derive(Component)]
pub struct Chef {
    order: u32,
    charge_time: f32,
    stopped_time: f32,
    was_sliding_last_frame: bool,
    current_lift: Option<CurrentLift>,
}

/// I'm too bundle-pilled
/// But really tho not being able to have the added things depend is an L
/// Basically bundles just feel like better constructors I guess idk
/// I don't want to remember what I have to spawn
#[derive(Bundle)]
struct ChefBundle {
    name: Name,
    chef: Chef,
    pos: Pos,
    transform: Transform,
    dyno: Dyno,
    srx: StaticRx,
    trx: TriggerRx,
    ttx: TriggerTx,
    anim: AnimMan<ChefAnim>,
}
impl ChefBundle {
    const LINE_GAP: f32 = 10.0;

    fn new(start: Pos, order: u32) -> Self {
        let physical_size = 10;
        let my_pos = Pos::new(start.x - order as f32 * Self::LINE_GAP, start.y);
        let hbox = HBox::new(physical_size, physical_size);

        Self {
            name: Name::new(format!("Chef_{order}")),
            chef: Chef {
                order,
                charge_time: 0.0,
                stopped_time: 0.0,
                was_sliding_last_frame: false,
                current_lift: None,
            },
            pos: my_pos,
            transform: my_pos.to_transform(ZIX_CHEF + 1.0 / (order as f32 + 1.0)),
            dyno: default(),
            srx: StaticRx::single(StaticRxKind::Default, hbox.clone()),
            trx: TriggerRx::single(TriggerRxKind::Chef, hbox.clone()),
            ttx: TriggerTx::single(TriggerTxKind::Chef, hbox.clone()),
            anim: AnimMan::default(),
        }
    }
}

#[derive(Component)]
pub struct ChefStart {
    needs_spawn: u32,
}
#[derive(Bundle)]
pub struct ChefStartBundle {
    name: Name,
    chef_start: ChefStart,
    pos: Pos,
}
impl MyLdtkEntity for ChefStartBundle {
    type Root = LevelActiveRoot;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let FieldValue::Int(Some(quantity)) = fields.get("Quantity").unwrap() else {
            panic!("Bad chefstart quantity: {:?}", fields);
        };
        Self {
            name: Name::new("ChefStart"),
            chef_start: ChefStart {
                needs_spawn: quantity.clone() as u32,
            },
            pos,
        }
    }
}

fn is_active_anim(chef_anim: ChefAnim) -> bool {
    matches!(
        chef_anim,
        ChefAnim::Ready | ChefAnim::Charge | ChefAnim::Drop | ChefAnim::Lift
    )
}

fn is_functionally_stopped(vel: Vec2, consts: &ChefConsts) -> bool {
    vel.length() < consts.stop_speed_cutoff
}

fn invariants(chef_q: Query<&AnimMan<ChefAnim>>) {
    // Collecting to vec in case I want to do more nuanced assertions later
    let active = chef_q
        .iter()
        .filter(|anim| is_active_anim(anim.get_state()))
        .collect::<Vec<_>>();
    debug_assert!(active.len() <= 1);
}

fn maybe_spawn_chefs(
    mut chef_start: Query<(&mut ChefStart, &Pos)>,
    mut commands: Commands,
    root: Res<LevelActiveRoot>,
) {
    let (mut chef_start, start_pos) = chef_start.single_mut();
    for order in 0..chef_start.needs_spawn {
        commands
            .spawn(ChefBundle::new(
                // Need to translate bc the ldtk thing is alighned to 16, not 24.
                start_pos.translated(Vec2::new(0.0, -3.0)),
                order,
            ))
            .set_parent(root.eid());
    }
    chef_start.needs_spawn = 0;
}

fn maybe_promote_chef(mut all_chefs: Query<(&mut AnimMan<ChefAnim>, &Chef, &mut Pos)>) {
    if all_chefs
        .iter()
        .any(|thing| is_active_anim(thing.0.get_state()))
    {
        // Somebody already active
        return;
    }
    let Some(mut next_up) = all_chefs
        .iter_mut()
        .filter(|thing| thing.0.get_state() == ChefAnim::Wait)
        .min_by(|a, b| a.1.order.cmp(&b.1.order))
    else {
        // No one is ready to promote
        return;
    };
    next_up.0.set_state(ChefAnim::Ready);

    let needs_shift = next_up.1.order > 0;
    if needs_shift {
        for mut stuff in all_chefs
            .iter_mut()
            .filter(|thing| matches!(thing.0.get_state(), ChefAnim::Wait | ChefAnim::Ready))
        {
            stuff.2.x += ChefBundle::LINE_GAP;
        }
    }
}

fn maybe_start_charge(mut chef_q: Query<&mut AnimMan<ChefAnim>>, butt_input: Res<ButtInput>) {
    if !butt_input.just_pressed(ButtKind::A) {
        return;
    }
    let Some(mut ready_chef) = chef_q
        .iter_mut()
        .filter(|chef| chef.get_state() == ChefAnim::Ready)
        .next()
    else {
        return;
    };
    ready_chef.set_state(ChefAnim::Charge);
}

fn maybe_update_charge(
    mut chef_q: Query<(&mut AnimMan<ChefAnim>, &mut Chef, &mut Dyno)>,
    butt_input: Res<ButtInput>,
    time: Res<Time>,
    consts: Res<ChefConsts>,
) {
    let Some(mut charging_chef) = chef_q
        .iter_mut()
        .filter(|thing| thing.0.get_state() == ChefAnim::Charge)
        .next()
    else {
        return;
    };
    if butt_input.pressed(ButtKind::A) {
        charging_chef.1.charge_time += time.delta_secs();
    } else {
        let max_time = 1.0;
        let ratio = charging_chef.1.charge_time.min(max_time).max(0.0);
        let speed =
            consts.min_launch_speed + (consts.max_launch_speed - consts.min_launch_speed) * ratio;
        charging_chef.2.vel.x = speed;
        charging_chef.0.set_state(ChefAnim::Drop);
    }
}

// Harder to fuck up the logic if I do this
enum HorDir {
    Left,
    None,
    Right,
}
impl HorDir {
    fn from_x(x: f32) -> Self {
        if x.abs() < 1.0 {
            Self::None
        } else if x < 0.0 {
            Self::Left
        } else {
            Self::Right
        }
    }
}

fn maybe_update_flight(
    mut chef_q: Query<(&mut AnimMan<ChefAnim>, &mut Dyno, &StaticRx, &mut Chef)>,
    butt_input: Res<ButtInput>,
    bullet_time: Res<BulletTime>,
    consts: Res<ChefConsts>,
    static_colls: Res<StaticColls>,
) {
    let Some(mut flying_chef) = chef_q
        .iter_mut()
        .filter(|thing| matches!(thing.0.get_state(), ChefAnim::Drop | ChefAnim::Lift))
        .next()
    else {
        return;
    };
    let my_colls = static_colls.get_refs(&flying_chef.2.coll_keys);

    // Ground collisions
    let mut is_sliding_this_frame = false;
    for coll in &my_colls {
        let (par_mul, perp_mul, slide_speed) = match coll.tx_hbox {
            HBOX_DIRT => {
                // So uhhhhh didn't I make a library to do this...? Lol
                (
                    consts.dirt_par_mul,
                    consts.dirt_perp_mul,
                    consts.dirt_slide_speed,
                )
            }
            HBOX_SMOOTH => (
                consts.smooth_par_mul,
                consts.smooth_perp_mul,
                consts.smooth_slide_speed,
            ),
            _ => {
                #[cfg(debug_assertions)]
                warn!("what is it colliding with? {:?}", coll.tx_hbox);
                continue;
            }
        };

        if flying_chef.3.was_sliding_last_frame && coll.push.y > 0.0 {
            let decrease_by = slide_speed * bullet_time.delta_secs();
            if flying_chef.1.vel.x.abs() < decrease_by {
                flying_chef.1.vel.x = 0.0;
            } else {
                flying_chef.1.vel.x -= flying_chef.1.vel.x.signum() * decrease_by;
            }
        } else {
            flying_chef.1.vel = coll.rx_par * par_mul - coll.rx_perp * perp_mul;
        }

        is_sliding_this_frame = is_sliding_this_frame || coll.push.y > 0.0;
    }
    flying_chef.3.was_sliding_last_frame = is_sliding_this_frame;

    let enough_speed_to_lift = flying_chef.1.vel.length() > consts.stop_speed_cutoff;

    // Lift
    match (
        enough_speed_to_lift,
        flying_chef.3.current_lift.as_mut(),
        butt_input.pressed(ButtKind::A),
    ) {
        (_, Some(current_lift), true) => {
            // Can always continue a lift (important for consistency of angle)
            let dyno_ang = flying_chef.1.vel.to_angle();
            let up_ang = dyno_ang + current_lift.ang_diff;
            let up = Vec2::new(up_ang.cos(), up_ang.sin()) * flying_chef.1.vel.length();
            flying_chef.1.vel += up * consts.lift_mul * bullet_time.delta_secs();
        }
        (false, _, _) | (true, _, false) => flying_chef.3.current_lift = None,
        (true, None, true) => {
            // BUT we can only start if we have enough speed
            let hor_dir = HorDir::from_x(flying_chef.1.vel.x);
            let is_right = matches!(hor_dir, HorDir::Right)
                || (matches!(hor_dir, HorDir::None) && !flying_chef.0.get_flip_y());
            if is_right {
                flying_chef.3.current_lift = Some(CurrentLift { ang_diff: PI / 2.0 });
            } else {
                flying_chef.3.current_lift = Some(CurrentLift {
                    ang_diff: -PI / 2.0,
                });
            }
        }
    }

    // Air drag
    if my_colls.is_empty() {
        let decrease_by = consts.air_drag_speed * bullet_time.delta_secs();
        if flying_chef.1.vel.length() < decrease_by {
            flying_chef.1.vel = Vec2::ZERO;
        } else {
            flying_chef.1.vel = Vec2::new(
                flying_chef.1.vel.x - flying_chef.1.vel.x.signum() * decrease_by,
                flying_chef.1.vel.y - flying_chef.1.vel.y.signum() * decrease_by,
            );
        }
    }

    // Anim
    // This is pretty cursed. It is definitely not the correct, elegant solution
    // I got here from a bunch of little tweaks. Honestly if you want to fix probably just throw out
    // this logic and restart
    let hor_dir = HorDir::from_x(flying_chef.1.vel.x);
    match flying_chef.3.current_lift.as_ref() {
        Some(current_lift) => {
            flying_chef.0.set_state(ChefAnim::Lift);
            flying_chef.0.set_flip_y(current_lift.ang_diff < 0.0);
            if !matches!(hor_dir, HorDir::None) {
                flying_chef.0.set_body_rot(flying_chef.1.vel.to_angle());
            } else {
                if is_sliding_this_frame {
                    let new_ang = if flying_chef.0.get_flip_y() { -PI } else { 0.0 };
                    flying_chef.0.set_body_rot(new_ang);
                } else if flying_chef.1.vel.y.abs() > consts.stop_speed_cutoff {
                    flying_chef.0.set_body_rot(flying_chef.1.vel.to_angle());
                }
            }
        }
        None => {
            flying_chef.0.set_state(ChefAnim::Drop);
            match hor_dir {
                HorDir::None => {
                    if is_sliding_this_frame {
                        let new_ang = if flying_chef.0.get_flip_y() { -PI } else { 0.0 };
                        flying_chef.0.set_body_rot(new_ang);
                    } else if flying_chef.1.vel.y.abs() > consts.stop_speed_cutoff {
                        flying_chef.0.set_body_rot(flying_chef.1.vel.to_angle());
                    }
                }
                HorDir::Left => {
                    flying_chef.0.set_flip_y(true);
                    flying_chef.0.set_body_rot(flying_chef.1.vel.to_angle());
                }
                HorDir::Right => {
                    flying_chef.0.set_flip_y(false);
                    flying_chef.0.set_body_rot(flying_chef.1.vel.to_angle());
                }
            }
        }
    }

    // Gravity
    flying_chef.1.vel.y -= consts.gravity * bullet_time.delta_secs();

    // Limit speed
    flying_chef.1.vel = flying_chef.1.vel.clamp_length_max(consts.max_speed);
}

fn maybe_end_flight(
    mut chef_q: Query<(&mut AnimMan<ChefAnim>, &mut Dyno, &mut Chef, &StaticRx)>,
    consts: Res<ChefConsts>,
    bullet_time: Res<BulletTime>,
    static_colls: Res<StaticColls>,
) {
    let Some(mut flying_chef) = chef_q
        .iter_mut()
        .filter(|thing| matches!(thing.0.get_state(), ChefAnim::Drop | ChefAnim::Lift))
        .next()
    else {
        return;
    };
    let above_ground = static_colls
        .get_refs(&flying_chef.3.coll_keys)
        .iter()
        .any(|coll| coll.push.y > 0.0);
    if above_ground && is_functionally_stopped(flying_chef.1.vel, &consts) {
        flying_chef.2.stopped_time += bullet_time.delta_secs();
    } else {
        flying_chef.2.stopped_time = 0.0;
    }
    if flying_chef.2.stopped_time > consts.stop_sleep_time {
        flying_chef.1.vel = Vec2::ZERO;
        flying_chef.0.set_state(ChefAnim::Sleep);
        let new_flip_x = flying_chef.0.get_flip_y();
        flying_chef.0.set_flip_x(new_flip_x);
        flying_chef.0.set_flip_y(false);
        flying_chef.0.set_body_rot(0.0);
    }
}

pub(super) fn register_chefs(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<ChefStartBundle>::new(
        "Entities",
        "ChefStart",
    ));

    app.insert_resource(ChefConsts::default());
    debug_resource!(app, ChefConsts);

    app.add_systems(
        Update,
        (
            invariants,
            maybe_spawn_chefs,
            maybe_promote_chef,
            maybe_start_charge,
            maybe_update_charge,
            maybe_update_flight,
            maybe_end_flight,
        )
            .chain()
            .run_if(in_state(MetaState::Level))
            .after(InputSet)
            .after(PhysicsSet),
    );
}
