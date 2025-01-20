use crate::prelude::*;

mod anim_defns;
// mod anim_wizardry;

pub(crate) use anim_defns::*;
// pub(crate) use anim_wizardry::*;

pub(super) struct BonusAnimPlugin;
impl Plugin for BonusAnimPlugin {
    fn build(&self, app: &mut App) {
        // anim_wizardry::register_anim_wizardry(app);
        anim_defns::register_anim_defns(app);
    }
}
