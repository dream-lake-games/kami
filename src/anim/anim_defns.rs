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
        #[fps(5.0)]
        #[render_layers(TransitionLayer)]
        Dots,
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
        #[length(16)]
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
        #[render_layers(MainStaticLayer)]
        #[offset(0.0, 7.0)]
        Sleep,
    }
);

macro_rules! wasm_hates_wizards {
    ($($anim:ident $(,)?)+) => {
        pub(super) fn register_anim_defns(app: &mut App) {
            app.add_plugins((
                $(
                    AnimDefnPlugin::<$anim, AnimTimeRes>::default(),
                )+
            ));
        }
    };
}

wasm_hates_wizards!(LoadingAnim, ChefAnim);
