use crate::prelude::*;

mod state_level;
mod state_menu;

pub use state_level::*;
pub use state_menu::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub enum MetaState {
    Setup,
    Menu,
    Level,
}

fn hack_past_setup(mut next_meta_state: ResMut<NextState<MetaState>>) {
    next_meta_state.set(MetaState::Menu);
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MetaState::Setup);
        app.add_systems(Update, hack_past_setup);
    }
}
