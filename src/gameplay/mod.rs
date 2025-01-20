use crate::prelude::*;

mod camera;
mod chef;
mod platforms;

pub(super) struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        camera::register_camera(app);
        chef::register_chefs(app);
        platforms::register_platforms(app);
    }
}
