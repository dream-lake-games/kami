//! This shit is awesome.
//! It doesn't work on WASM tho :(

#![expect(dead_code)]

use crate::prelude::*;

#[doc(hidden)]
pub(crate) struct _AnimWizardry {
    pub(crate) do_register: fn(app: &mut App) -> (),
}
inventory::collect!(_AnimWizardry);

/// Still some stutter, but much less
macro_rules! defn_anim {
    ($name:ident, $i:item) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, Default, Reflect, PartialEq, Eq, Hash, AnimStateMachine)]
            $i
            #[doc(hidden)]
            #[expect(nonstandard_style)]
            pub(crate) fn [<_wizardry_for_$name>](app: &mut App) {
                app.add_plugins(AnimDefnPlugin::<$name, AnimTimeRes>::default());
                panic!("adding some plugin....");
            }
            inventory::submit! {
                _AnimWizardry { do_register: [<_wizardry_for_ $name>] }
            }
        }
    };
}
pub(crate) use defn_anim;

pub(super) fn register_anim_wizardry(app: &mut App) {
    for spell in inventory::iter::<_AnimWizardry> {
        (spell.do_register)(app);
    }
}
