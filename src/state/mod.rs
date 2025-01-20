use crate::{debug::debug_resource, prelude::*};

mod state_triggers;

pub use state_triggers::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub enum MetaState {
    Setup,
    Menu,
    LevelLoading,
    Level,
}

fn hack_past_setup(mut commands: Commands) {
    commands.trigger(LoadMenu::kind(MenuKind::Title));
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hack_past_setup.run_if(in_state(MetaState::Setup)));

        app.insert_state(MetaState::Setup);

        state_triggers::register_state_commands(app);

        debug_resource!(app, State<MetaState>);
    }
}
