use crate::prelude::*;

mod cake;
mod camera;
mod chef;
mod overlay;
mod platforms;

pub(super) struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        cake::register_cake(app);
        camera::register_camera(app);
        chef::register_chefs(app);
        overlay::register_overlay(app);
        platforms::register_platforms(app);
    }
}
