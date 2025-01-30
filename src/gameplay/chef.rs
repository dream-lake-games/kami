use egui::Color32;

use crate::{debug::debug_resource, prelude::*};

/// "Slide speed" is a bit of misnomer
/// To help with framerate independence, when sliding, we don't multiply by some frac,
/// but subtract some amount. This "slide speed" is how much we subtract. So higher
/// actually means a less slippery surface
#[derive(Resource, Reflect)]
struct ChefConsts {
    min_launch_speed: f32,
    max_launch_speed: f32,
    gravity: f32,
    max_speed: f32,
    stop_speed_cutoff: f32,
    stop_sleep_time: f32,
    dirt_rough_par_mul: f32,
    dirt_rough_perp_mul: f32,
    dirt_rough_slide_speed: f32,
    dirt_smooth_par_mul: f32,
    dirt_smooth_perp_mul: f32,
    dirt_smooth_slide_speed: f32,
    cake_par_mul: f32,
    cake_perp_mul: f32,
    cake_slide_speed: f32,
    air_drag_speed: f32,
    lift_mul: f32,
    sound_min_speed: f32,
    sound_max_speed: f32,
}
impl Default for ChefConsts {
    fn default() -> Self {
        Self {
            min_launch_speed: 5.0,
            max_launch_speed: 100.0,
            gravity: 60.0,
            max_speed: 125.0,
            stop_speed_cutoff: 8.0,
            stop_sleep_time: 0.5,
            dirt_rough_par_mul: 0.7,
            dirt_rough_perp_mul: 0.5,
            dirt_rough_slide_speed: 80.0,
            dirt_smooth_par_mul: 0.95,
            dirt_smooth_perp_mul: 0.3,
            dirt_smooth_slide_speed: 10.0,
            cake_par_mul: 0.5,
            cake_perp_mul: 0.2,
            cake_slide_speed: 120.0,
            air_drag_speed: 2.0,
            lift_mul: 2.5,
            sound_min_speed: 12.0,
            sound_max_speed: 75.0,
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
    time_since_z: f32,
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
    anim: AnimMan<ChefAnim>,
    light: LightMan<Light128Anim>,
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
                time_since_z: 0.7,
            },
            pos: my_pos,
            transform: my_pos.to_transform(ZIX_CHEF + 1.0 / (order as f32 + 1.0)),
            dyno: default(),
            srx: StaticRx::single(StaticRxKind::Default, hbox.clone()),
            trx: TriggerRx::single(TriggerRxKind::Chef, hbox.clone()),
            anim: AnimMan::default(),
            light: LightMan::new(Light128Anim::None),
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
    let Ok((mut chef_start, start_pos)) = chef_start.get_single_mut() else {
        return;
    };
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

fn maybe_promote_chef(
    mut all_chefs: Query<(
        &mut AnimMan<ChefAnim>,
        &Chef,
        &mut Pos,
        &mut Transform,
        &mut LightMan<Light128Anim>,
    )>,
    mut bullet_time: ResMut<BulletTime>,
) {
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
    next_up.4.set_state(Light128Anim::Grow);
    next_up.3.translation.z = ZIX_CHEF + 1.0 + next_up.1.order as f32 / 10.0;

    bullet_time.clear_effects();
    bullet_time.set_base(BulletTimeSpeed::Normal);

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

#[derive(Component)]
struct ChargeSoundEffect;

fn maybe_start_charge(
    mut chef_q: Query<&mut AnimMan<ChefAnim>>,
    butt_input: Res<ButtInput>,
    mut commands: Commands,
    detail_root: Res<LevelDetailRoot>,
) {
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
    commands
        .spawn((
            Name::new("ChargeSound1"),
            ChargeSoundEffect,
            SoundEffect::ChefCharge1,
        ))
        .set_parent(detail_root.eid());
}

fn maybe_update_charge(
    mut chef_q: Query<(&mut AnimMan<ChefAnim>, &mut Chef, &mut Dyno)>,
    butt_input: Res<ButtInput>,
    time: Res<Time>,
    consts: Res<ChefConsts>,
    charge_sound: Query<Entity, With<ChargeSoundEffect>>,
    mut commands: Commands,
    detail_root: Res<LevelDetailRoot>,
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
        if charge_sound.is_empty() {
            commands
                .spawn((
                    Name::new("ChargeSound2"),
                    ChargeSoundEffect,
                    SoundEffect::ChefCharge2,
                ))
                .set_parent(detail_root.eid());
        }
    } else {
        let max_time = 1.0;
        let ratio = charging_chef.1.charge_time.min(max_time).max(0.0);
        let speed =
            consts.min_launch_speed + (consts.max_launch_speed - consts.min_launch_speed) * ratio;
        charging_chef.2.vel.x = speed;
        charging_chef.0.set_state(ChefAnim::Drop);
        commands
            .spawn(SoundEffect::ChefLaunch)
            .set_parent(detail_root.eid());
        for eid in &charge_sound {
            if let Some(mut comms) = commands.get_entity(eid) {
                comms.try_despawn();
            }
        }
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
    mut bullet_time: ResMut<BulletTime>,
    consts: Res<ChefConsts>,
    static_colls: Res<StaticColls>,
    mut commands: Commands,
    detail_root: Res<LevelDetailRoot>,
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
        let (par_mul, perp_mul, slide_speed, se) = match coll.tx_hbox {
            HBOX_DIRT_ROUGH => {
                // So uhhhhh didn't I make a library to do this...? Lol
                (
                    consts.dirt_rough_par_mul,
                    consts.dirt_rough_perp_mul,
                    consts.dirt_rough_slide_speed,
                    SoundEffect::ImpactDirtRough,
                )
            }
            HBOX_DIRT_SMOOTH => (
                consts.dirt_smooth_par_mul,
                consts.dirt_smooth_perp_mul,
                consts.dirt_smooth_slide_speed,
                SoundEffect::ImpactDirtSmooth,
            ),
            HBOX_CAKE_GREEN | HBOX_CAKE_BLUE | HBOX_CAKE_PINK | HBOX_CAKE_RED => (
                consts.cake_par_mul,
                consts.cake_perp_mul,
                consts.cake_slide_speed,
                SoundEffect::ImpactCake,
            ),
            _ => {
                #[cfg(debug_assertions)]
                warn!("what is it colliding with? {:?}", coll.tx_hbox);
                continue;
            }
        };
        let (part_color, part_num, part_size) = match coll.tx_hbox {
            HBOX_DIRT_ROUGH => (RGC5, 2, 3),
            HBOX_DIRT_SMOOTH => (RGC4, 2, 3),
            HBOX_CAKE_GREEN => (RGC4, 4, 6),
            HBOX_CAKE_BLUE => (RGC4, 4, 6),
            HBOX_CAKE_PINK => (RGC3, 4, 6),
            HBOX_CAKE_RED => (RGC2, 4, 6),
            _ => (RGC5, 2, 3),
        };

        if (coll.push.x != 0.0
            && flying_chef.1.vel.x.abs() > consts.stop_speed_cutoff
            && flying_chef.3.was_sliding_last_frame
            && coll.push.x.signum() == flying_chef.1.vel.x.signum())
            || (coll.push.y != 0.0
                && flying_chef.1.vel.y.abs() > consts.stop_speed_cutoff
                && coll.push.y.signum() == flying_chef.1.vel.y.signum())
        {
            // Probably a bugged collision
            flying_chef.1.vel = coll.rx_perp + coll.rx_par;
            continue;
        }

        let perp_sound_mult = (coll
            .rx_perp
            .length()
            .clamp(consts.sound_min_speed, consts.sound_max_speed)
            - consts.sound_min_speed)
            / (consts.sound_max_speed - consts.sound_min_speed);
        if perp_sound_mult > 0.1 {
            commands
                .spawn((se, SoundMult(perp_sound_mult)))
                .set_parent(detail_root.eid());
            let base_part_dyno = (coll.rx_par - coll.rx_perp) * 0.5;
            for _ in 0..part_num {
                let part_dyno = Vec2::new(
                    base_part_dyno.x + thread_rng().gen_range(-20.0..20.0),
                    base_part_dyno.y + thread_rng().gen_range(-20.0..20.0),
                );
                commands
                    .spawn(
                        ParticleBundle::new(coll.rx_pos)
                            .with_color(part_color)
                            .with_zix(ZIX_CHEF + 5.0)
                            .with_dyno(part_dyno.x, part_dyno.y)
                            .with_size(part_size as f32),
                    )
                    .set_parent(detail_root.eid());
            }
        } else if flying_chef.3.was_sliding_last_frame {
            let par_sound_mult = (coll
                .rx_par
                .length()
                .clamp(consts.sound_min_speed, consts.sound_max_speed)
                - consts.sound_min_speed)
                / (consts.sound_max_speed - consts.sound_min_speed);
            commands.spawn((
                SoundEffect::ImpactSmoothSlide,
                SoundMult(par_sound_mult),
                OneSound::Ignore,
            ));
        }

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
        if flying_chef.1.vel.x.abs() < 0.05
            && coll.push.x.abs() > 0.0
            && !flying_chef.3.was_sliding_last_frame
        {
            flying_chef.1.vel.x = coll.push.x.signum() * 6.0;
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
    if flying_chef.3.current_lift.is_some() {
        if bullet_time.get_base() == BulletTimeSpeed::Normal {
            bullet_time.clear_effects();
            bullet_time.add_effect(BulletTimeSpeed::LiftTransition, 0.1);
        }
        bullet_time.set_base(BulletTimeSpeed::LiftSlow);
    } else {
        bullet_time.clear_effects();
        if bullet_time.get_base() == BulletTimeSpeed::LiftSlow {
            bullet_time.add_effect(BulletTimeSpeed::LiftTransition, 0.1);
        }
        bullet_time.set_base(BulletTimeSpeed::Normal);
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
    mut chef_q: Query<(
        &mut AnimMan<ChefAnim>,
        &mut Dyno,
        &mut Chef,
        &StaticRx,
        &mut Pos,
        &mut Transform,
        &mut LightMan<Light128Anim>,
        &TriggerRx,
    )>,
    consts: Res<ChefConsts>,
    mut bullet_time: ResMut<BulletTime>,
    static_colls: Res<StaticColls>,
    trigger_colls: Res<TriggerColls>,
    mut level_state: ResMut<LevelState>,
    pos_q: Query<&Pos, Without<AnimMan<ChefAnim>>>,
    mut commands: Commands,
    root: Res<LevelDetailRoot>,
) {
    let Some(mut flying_chef) = chef_q
        .iter_mut()
        .filter(|thing| matches!(thing.0.get_state(), ChefAnim::Drop | ChefAnim::Lift))
        .next()
    else {
        return;
    };

    let way_oob_dist = 20.0;
    let chef_pos = flying_chef.4.clone();
    let x_way_oob = chef_pos.x + way_oob_dist < level_state.rect.min.x
        || chef_pos.x - way_oob_dist > level_state.rect.max.x;
    // NOTE: "Too high" is intentionally not oob
    let y_way_oob = chef_pos.y + way_oob_dist < level_state.rect.min.y;
    if x_way_oob || y_way_oob {
        // TODO: This should count as a death
        flying_chef.1.vel = Vec2::ZERO;
        flying_chef.0.set_state(ChefAnim::Sleep);
        flying_chef.6.set_state(Light128Anim::Shrink);
        bullet_time.clear_effects();
        bullet_time.set_base(BulletTimeSpeed::Normal);
        return;
    }

    let above_colls = static_colls
        .get_refs(&flying_chef.3.coll_keys)
        .into_iter()
        .filter(|coll| coll.push.y > 0.0)
        .collect::<Vec<_>>();
    if !above_colls.is_empty() && is_functionally_stopped(flying_chef.1.vel, &consts) {
        flying_chef.2.stopped_time += bullet_time.delta_secs();
    } else {
        flying_chef.2.stopped_time = 0.0;
    }
    if flying_chef.2.stopped_time > consts.stop_sleep_time {
        bullet_time.clear_effects();
        bullet_time.set_base(BulletTimeSpeed::Normal);

        flying_chef.1.vel = Vec2::ZERO;
        flying_chef.0.set_state(ChefAnim::Sleep);
        flying_chef.6.set_state(Light128Anim::Shrink);
        let new_flip_x = flying_chef.0.get_flip_y();
        flying_chef.0.set_flip_x(new_flip_x);
        flying_chef.0.set_flip_y(false);
        flying_chef.0.set_body_rot(0.0);

        let Some(above_coll) = above_colls
            .into_iter()
            .max_by(|a, b| a.push.y.total_cmp(&b.push.y))
        else {
            panic!("there should be an above_coll here");
        };
        let block_pos = pos_q.get(above_coll.tx_ctrl).unwrap();
        flying_chef.4.x = flying_chef.4.x.clamp(block_pos.x - 4.0, block_pos.x + 4.0);
        let mut spawn_score = |anim: ScoreAnim, se: SoundEffect| {
            commands
                .spawn((EphemeralAnim::new(
                    anim,
                    false,
                    flying_chef.4.clone().translated(Vec2::Y * 18.0),
                    ZIX_CHEF + 5.0,
                ),))
                .set_parent(root.eid());
            commands.spawn(se).set_parent(root.eid());
        };
        match above_coll.tx_hbox {
            HBOX_CAKE_GREEN => {
                level_state.score -= 100;
                spawn_score(ScoreAnim::N100, SoundEffect::ScoreN100);
            }
            HBOX_CAKE_BLUE => {
                level_state.score += 100;
                spawn_score(ScoreAnim::P100, SoundEffect::ScoreP100);
            }
            HBOX_CAKE_PINK => {
                level_state.score += 200;
                spawn_score(ScoreAnim::P200, SoundEffect::ScoreP200);
            }
            HBOX_CAKE_RED => {
                level_state.score += 300;
                spawn_score(ScoreAnim::P300, SoundEffect::ScoreP300);
            }
            _ => (),
        }
        flying_chef.5.translation.z = ZIX_CHEF + flying_chef.2.order as f32 / 10.0;
    }

    if trigger_colls
        .get_refs(&flying_chef.7.coll_keys)
        .iter()
        .any(|coll| coll.tx_kind == TriggerTxKind::Spikes)
    {
        // Should explode
        flying_chef.0.set_state(ChefAnim::Explode);
        flying_chef.1.vel *= 0.05;
        commands.spawn(SoundEffect::Death);
    }
}

fn maybe_show_end(
    chef_q: Query<(&AnimMan<ChefAnim>,)>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    level_state: Res<LevelState>,
    mut save_data: ResMut<Pers<SaveData>>,
    mut store: ResMut<PkvStore>,
    egui_textures: Res<EguiTextures>,
) {
    let at_end = chef_q
        .iter()
        .all(|chef| matches!(chef.0.get_state(), ChefAnim::Sleep));
    if !at_end {
        return;
    }
    if save_data
        .get()
        .map
        .get(&level_state.lid)
        .cloned()
        .unwrap_or_default()
        .hiscore
        .unwrap_or(-1000000)
        < level_state.score
    {
        let mut new_map = save_data.get().map.clone();
        new_map.insert(
            level_state.lid.clone(),
            LevelSave {
                hiscore: Some(level_state.score),
            },
        );
        let old = save_data.get().clone();
        save_data.set(SaveData {
            map: new_map,
            ..old
        });
        save_data.save(&mut store);
    }
    let this_level_ix = LEVEL_DEFNS
        .iter()
        .position(|level_defn| level_defn.lid == level_state.lid)
        .unwrap();
    let this_level_defn = &LEVEL_DEFNS[this_level_ix];
    if save_data.get().menu_ix as usize != this_level_ix {
        let old = save_data.get().clone();
        save_data.set(SaveData {
            menu_ix: this_level_ix as u32,
            ..old
        });
        save_data.save(&mut store);
    }

    let hiscore = save_data
        .get()
        .map
        .get(&level_state.lid)
        .cloned()
        .unwrap_or_default()
        .hiscore
        .unwrap_or(level_state.score);

    let level_defn = get_level_defn(&level_state.lid);

    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(Color32::from_rgba_premultiplied(
            EGC8.r(),
            EGC8.g(),
            EGC8.b(),
            220,
        )))
        .show(ctx, |ui| {
            let force_width = 400.0;
            ui.vertical_centered(|ui| {
                ui.set_min_width(force_width);
                ui.set_max_width(force_width);
                let vspacing = ui.available_height() / 24.0;
                ui.style_mut().visuals.override_text_color = Some(EGC1);
                ui.add_space(vspacing);
                ui.heading("SCORE");
                ui.label(level_state.score.to_string());
                ui.small(format!("HI: {hiscore}"));
                ui.add_space(vspacing);
                ui.vertical(|ui| {
                    render_tier_grid(
                        ui,
                        force_width,
                        &level_defn.tiers,
                        Some(level_state.score),
                        &egui_textures,
                    );
                });
                ui.add_space(vspacing);
                ui.style_mut().visuals.override_text_color = Some(EGC8);
                if ui
                    .add_enabled(
                        hiscore >= this_level_defn.tiers.one,
                        egui::Button::new("NEXT").min_size(control_butt_size().into()),
                    )
                    .clicked()
                {
                    commands.spawn(SoundEffect::MenuClick);
                    commands.trigger(NextLevel);
                }
                ui.add_space(vspacing);
                if ui
                    .add_sized(control_butt_size(), egui::Button::new("RETRY"))
                    .clicked()
                {
                    commands.spawn(SoundEffect::MenuClick);
                    commands.trigger(
                        LoadLevel::lid(level_state.lid.clone()).with_skip_intro_messages(true),
                    );
                }
                ui.add_space(vspacing);
                if ui
                    .add_sized(control_butt_size(), egui::Button::new("MENU"))
                    .clicked()
                {
                    commands.spawn(SoundEffect::MenuClick);
                    commands.trigger(LoadMenu::kind(MenuKind::Levels));
                }
            });
        });
}

fn juice_trails(
    chef_q: Query<(&Pos, &AnimMan<ChefAnim>)>,
    mut commands: Commands,
    root: Res<LevelDetailRoot>,
) {
    for chef in &chef_q {
        let (part_size, part_color, part_lifespan) = match chef.1.get_state() {
            ChefAnim::Drop => (4, RGC2, 0.6),
            ChefAnim::Lift => (6, RGC1, 0.8),
            _ => continue,
        };
        commands
            .spawn(
                ParticleBundle::new(chef.0.clone())
                    .with_color(part_color)
                    .with_size(part_size as f32)
                    .with_lifespan(part_lifespan),
            )
            .set_parent(root.eid());
    }
}

fn handle_restart(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    level_state: Res<LevelState>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        commands.trigger(LoadLevel::lid(level_state.lid.clone()));
    }
}

fn juice_sleeping(
    mut chefs: Query<(&AnimMan<ChefAnim>, &Pos, &mut Chef)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
    detail_root: Res<LevelDetailRoot>,
) {
    for (anim, pos, mut chef) in chefs
        .iter_mut()
        .filter(|thing| thing.0.get_state() == ChefAnim::Sleep)
    {
        chef.time_since_z += bullet_time.delta_secs() * thread_rng().gen_range(0.8..1.2);
        if chef.time_since_z > 1.0 {
            chef.time_since_z = 0.0;
            let real_pos = pos.translated(Vec2::new(
                10.0 * if anim.get_flip_x() { -1.0 } else { 1.0 },
                6.0,
            ));
            commands
                .spawn((
                    real_pos.clone(),
                    Dyno::new(thread_rng().gen_range(-10.0..10.0), 10.0),
                    EphemeralAnim::new(ZAnim::Rise, false, real_pos, ZIX_CHEF + 1.0),
                ))
                .set_parent(detail_root.eid());
        }
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
        (handle_restart, maybe_spawn_chefs).run_if(in_state(MetaState::Level)),
    );

    app.add_systems(
        Update,
        (
            invariants,
            maybe_promote_chef,
            maybe_start_charge,
            maybe_update_charge,
            maybe_update_flight,
            maybe_end_flight,
            maybe_show_end,
        )
            .chain()
            .run_if(in_state(MetaState::Level))
            .run_if(physics_active)
            .after(InputSet)
            .after(PhysicsSet)
            .after(egui_always_helpers)
            .after(maybe_spawn_chefs),
    );

    app.add_systems(
        Update,
        (juice_trails, juice_sleeping)
            .chain()
            .run_if(in_state(MetaState::Level))
            .run_if(physics_active)
            .after(InputSet)
            .after(PhysicsSet)
            .after(egui_always_helpers),
    );
}
