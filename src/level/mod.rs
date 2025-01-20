use crate::prelude::*;

mod level_loading;

#[derive(Resource, Default)]
pub struct LevelState {
    pub lid: String,
    pub paused: bool,
    pub rect: Rect,
    pub score: i32,
}

pub(super) struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelState::default());

        level_loading::register_level_loading(app);
    }
}
