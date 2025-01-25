use crate::prelude::*;

macro_rules! defn_effects {
    ([$($name:ident, $path:literal, $mult:literal,)*]) => {
        #[derive(Component, Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
        pub enum SoundEffect {
            $($name,)*
        }
        impl SoundEffect {
            pub fn path(&self) -> String {
                match self {
                    $(Self::$name => $path.to_string(),)*
                }
            }
        }

        #[derive(Resource, Reflect)]
        pub struct SoundMults {
            pub map: HashMap<SoundEffect, f32>,
        }
        impl Default for SoundMults {
            fn default() -> Self {
                let mut map = HashMap::new();
                $(
                    map.insert(SoundEffect::$name, $mult);
                )*
                Self { map }
            }
        }
    };
}

defn_effects!([
    ScoreN100,
    "sounds/score/n100.ogg",
    0.08,
    ScoreP100,
    "sounds/score/100.ogg",
    0.08,
    ScoreP150,
    "sounds/score/150.ogg",
    0.08,
    ScoreP200,
    "sounds/score/200.ogg",
    0.08,
    ScoreP300,
    "sounds/score/300.ogg",
    0.08,
    ImpactCake,
    "sounds/impact/cake.ogg",
    0.04,
    ImpactDirtRough,
    "sounds/impact/dirt_rough.ogg",
    0.04,
    ImpactDirtSmooth,
    "sounds/impact/dirt_smooth.ogg",
    0.04,
    ImpactSmoothSlide,
    "sounds/impact/smooth_slide.ogg",
    0.1,
    ChefCharge1,
    "sounds/chef/charge1.ogg",
    0.005,
    ChefCharge2,
    "sounds/chef/charge2.ogg",
    0.005,
    ChefLaunch,
    "sounds/chef/launch.ogg",
    0.02,
]);

pub(super) fn register_effect_defns(app: &mut App) {
    app.insert_resource(SoundMults::default());
    // debug_resource!(app, SoundMults);
}
