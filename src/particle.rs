use crate::prelude::*;

#[derive(Component)]
struct Particle {
    lifespan: f32,
    max_lifespan: f32,
    max_size: f32,
}

#[derive(Bundle)]
pub struct ParticleBundle {
    name: Name,
    particle: Particle,
    pos: Pos,
    dyno: Dyno,
    transform: Transform,
    sprite: Sprite,
    srx: StaticRx,
}
impl ParticleBundle {
    pub fn new(pos: Pos) -> Self {
        let mut tran = pos.to_transform(5.0);
        tran.scale = Vec3::ONE * 6.0;
        Self {
            name: Name::new("Particle"),
            particle: Particle {
                lifespan: 1.0,
                max_lifespan: 1.0,
                max_size: 6.0,
            },
            pos,
            dyno: default(),
            transform: pos.to_transform(0.0),
            sprite: Sprite::default(),
            srx: StaticRx::single(StaticRxKind::Default, HBox::new(2, 2)),
        }
    }
    pub fn with_dyno(mut self, x: f32, y: f32) -> Self {
        self.dyno = Dyno::new(x, y);
        self
    }
    pub fn with_zix(mut self, zix: f32) -> Self {
        self.transform.translation.z = zix;
        self
    }
    pub fn with_size(mut self, size: f32) -> Self {
        self.particle.max_size = size;
        self
    }
    pub fn with_lifespan(mut self, lifespan: f32) -> Self {
        self.particle.lifespan = lifespan;
        self.particle.max_lifespan = lifespan;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.sprite.color = color;
        self
    }
}

fn update_particles(
    mut parts: Query<(Entity, &mut Transform, &mut Particle, &mut Dyno)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut tran, mut part, mut dyno) in &mut parts {
        if part.lifespan < 0.0 {
            commands.entity(eid).despawn();
            continue;
        }
        part.lifespan -= bullet_time.delta_secs();
        let life_frac = part.lifespan / part.max_lifespan;
        tran.translation.z -= bullet_time.delta_secs();
        tran.scale = Vec3::ONE * life_frac * part.max_size;
        dyno.vel.y -= 50.0 * bullet_time.delta_secs();
        dyno.vel *= 0.95;
    }
}

pub(super) struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_particles.after(PhysicsSet));
    }
}
