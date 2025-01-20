use crate::prelude::*;

mod anim_wizardry;

pub(crate) use anim_wizardry::*;

defn_anim!(
    LoadingAnim,
    pub enum LoadingAnim {
        // #[default]
        #[file("none.png")]
        #[size(1, 1)]
        None,
        #[default]
        #[file("transition/loading.png")]
        #[size(64, 64)]
        #[length(5)]
        #[fps(5.0)]
        #[render_layers(TransitionLayer)]
        Dots,
    }
);

pub(super) struct BonusAnimPlugin;
impl Plugin for BonusAnimPlugin {
    fn build(&self, app: &mut App) {
        anim_wizardry::register_anim_wizardry(app);
    }
}
