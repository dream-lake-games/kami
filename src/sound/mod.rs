use crate::prelude::*;

pub mod effect;
pub mod effect_defns;
pub mod song;
pub mod song_defns;

pub use effect::*;
pub use effect_defns::*;
pub use song::*;
pub use song_defns::*;

pub(super) struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SongPlugin);
        app.add_plugins(SoundEffectPlugin);
        effect_defns::register_effect_defns(app);
        song_defns::register_song_defns(app);
    }
}
