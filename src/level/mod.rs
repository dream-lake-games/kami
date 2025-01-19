use crate::prelude::*;

#[derive(Resource, Default)]
pub struct LevelState {
    pub lid: String,
    pub paused: bool,
}

fn on_enter() {}

fn on_exit() {}

pub(super) struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelState::default());
    }
}
