use crate::prelude::*;

derive_anim!(
    pub enum LoadingAnim {
        #[default]
        #[file("none.png")]
        #[size(1, 1)]
        None,
        #[file("transition/loading.png")]
        #[size(64, 64)]
        #[length(5)]
        #[fps(10.0)]
        #[render_layers(TransitionLayer)]
        Dots,
        #[file("transition/loading.png")]
        #[size(64, 64)]
        #[length(5)]
        #[fps(10.0)]
        #[render_layers(TransitionLayer)]
        #[next(None)]
        FakeDots,
    }
);

derive_anim!(
    pub enum ChefAnim {
        #[default]
        #[file("chefs/wait.png")]
        #[size(24, 24)]
        #[length(2)]
        #[fps(4.0)]
        #[render_layers(MainStaticLayer)]
        #[offset(0.0, 7.0)]
        Wait,
        #[file("chefs/ready.png")]
        #[size(24, 24)]
        #[length(6)]
        #[fps(4.0)]
        #[render_layers(MainStaticLayer)]
        #[offset(0.0, 7.0)]
        Ready,
        #[file("chefs/charge.png")]
        #[size(24, 24)]
        #[length(19)]
        #[render_layers(MainStaticLayer)]
        #[next(Drop)]
        #[offset(0.0, 7.0)]
        Charge,
        #[file("chefs/drop.png")]
        #[size(24, 24)]
        #[render_layers(MainStaticLayer)]
        Drop,
        #[file("chefs/lift.png")]
        #[size(24, 24)]
        #[render_layers(MainStaticLayer)]
        Lift,
        #[file("chefs/sleep.png")]
        #[size(24, 24)]
        #[length(6)]
        #[fps(5.0)]
        #[render_layers(MainStaticLayer)]
        #[offset(0.0, 7.0)]
        Sleep,
        #[file("chefs/explode.png")]
        #[size(24, 24)]
        #[length(6)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        Explode,
    }
);

derive_anim!(
    pub enum ChefTrailAlwaysAnim {
        #[default]
        #[file("chefs/trail_always.png")]
        #[size(12, 12)]
        #[length(4)]
        #[fps(8.0)]
        #[render_layers(MainStaticLayer)]
        #[zix(-1.0)]
        #[next(Despawn)]
        Disappear,
    }
);
derive_anim!(
    pub enum ChefTrailLiftAnim {
        #[default]
        #[file("chefs/trail_smoke1.png")]
        #[size(12, 12)]
        #[length(3)]
        #[fps(12.0)]
        #[render_layers(MainStaticLayer)]
        #[zix(-1.0)]
        #[next(Despawn)]
        Smoke1,
        #[file("chefs/trail_smoke2.png")]
        #[size(12, 12)]
        #[length(3)]
        #[fps(12.0)]
        #[render_layers(MainStaticLayer)]
        #[zix(-1.0)]
        #[next(Despawn)]
        Smoke2,
        #[file("chefs/trail_smoke3.png")]
        #[size(12, 12)]
        #[length(3)]
        #[fps(12.0)]
        #[render_layers(MainStaticLayer)]
        #[zix(-1.0)]
        #[next(Despawn)]
        Smoke3,
    }
);
impl ChefTrailLiftAnim {
    pub fn rand() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Smoke1,
            1 => Self::Smoke2,
            2 => Self::Smoke3,
            _ => unreachable!(),
        }
    }
}

derive_anim!(
    pub enum ScoreAnim {
        #[default]
        #[file("score/n100.png")]
        #[size(24, 24)]
        #[length(7)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        N100,
        #[file("score/100.png")]
        #[size(24, 24)]
        #[length(8)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        P100,
        #[file("score/150.png")]
        #[size(24, 24)]
        #[length(8)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        P150,
        #[file("score/200.png")]
        #[size(24, 24)]
        #[length(8)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        P200,
        #[file("score/300.png")]
        #[size(24, 24)]
        #[length(8)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        P300,
    }
);

derive_anim!(
    pub enum AcornAnim {
        #[default]
        #[file("items/acorn_pulse.png")]
        #[size(16, 16)]
        #[length(12)]
        #[render_layers(MainStaticLayer)]
        Pulse,
        #[file("items/acorn_pop.png")]
        #[size(16, 16)]
        #[length(3)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        Pop,
    }
);

derive_anim!(
    pub enum ZAnim {
        #[default]
        #[file("chefs/z.png")]
        #[size(12, 12)]
        #[length(7)]
        #[render_layers(MainStaticLayer)]
        #[next(Despawn)]
        Rise,
    }
);

derive_anim!(
    pub enum Light128Anim {
        #[default]
        #[file("none.png")]
        #[size(1, 1)]
        #[render_layers(LightLayer)]
        None,
        #[file("chefs/light_grow.png")]
        #[size(128, 128)]
        #[length(3)]
        #[render_layers(LightLayer)]
        #[next(Full)]
        Grow,
        #[file("chefs/light_full.png")]
        #[size(128, 128)]
        #[render_layers(LightLayer)]
        Full,
        #[file("chefs/light_shrink.png")]
        #[size(128, 128)]
        #[length(3)]
        #[render_layers(LightLayer)]
        #[next(None)]
        Shrink,
    }
);
impl LightAnim for Light128Anim {
    fn light_radius(&self) -> Option<f32> {
        match self {
            // Self::None => None,
            // _ => Some(64.0),
            // Sad, but I'd rather have the game run fast then have cool lighting
            _ => None,
        }
    }
}

macro_rules! wasm_hates_wizards {
    ($($anim:ident $(,)?)+) => {
        pub(super) fn register_anim_defns(app: &mut App) {
            // this is so bad
            app.add_plugins(LightDefnPlugin::<Light128Anim>::default());

            app.add_plugins((
                $(
                    AnimDefnPlugin::<$anim, AnimTimeRes>::default(),
                )+
            ));
        }
    };
}

wasm_hates_wizards!(
    LoadingAnim,
    ChefAnim,
    ChefTrailAlwaysAnim,
    ChefTrailLiftAnim,
    ScoreAnim,
    AcornAnim,
    ZAnim,
);
