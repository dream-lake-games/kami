use bevy::prelude::*;

mod anim;
mod bg;
mod consts;
mod debug;
mod gameplay;
mod input;
mod level;
mod menu;
mod my_ldtk;
mod root;
mod state;
mod two_delight;

/// Idk if this is good practice for maintainability, but it definitely saves a lot of time to just
/// be able to `use crate::prelude::*` and get everything I need at the top of files.
#[expect(unused_imports)]
mod prelude {
    pub use super::{
        anim::*, bg::*, consts::*, gameplay::*, input::*, level::*, menu::*, my_ldtk::*, root::*,
        state::*, two_delight::*,
    };
    pub use bevy::{
        color::palettes::tailwind,
        ecs::{
            component::{ComponentId, StorageType},
            world::DeferredWorld,
        },
        input::common_conditions::input_toggle_active,
        log,
        math::VectorSpace,
        prelude::*,
        reflect::GetTypeRegistration,
        render::view::RenderLayers,
        sprite::Anchor,
        text::TextBounds,
        utils::{HashMap, HashSet},
    };
    pub use bevy_2delight_anims::prelude::*;
    pub use bevy_2delight_layers::prelude::*;
    pub use bevy_2delight_physics::prelude::*;
    pub use bevy_ecs_ldtk::ldtk::FieldInstance;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_egui::{egui, EguiContexts};
    pub use bevy_pkv::PkvStore;
    pub use rand::prelude::SliceRandom;
    pub use rand::{thread_rng, Rng};
    pub use serde::{Deserialize, Serialize};
    pub use std::{ops::Range, time::Duration};
}

fn main() {
    let mut app = App::new();

    // NOTE: Has to be first
    app.add_plugins(two_delight::TwoDelightPlugin);
    // app.add_plugins(bevy_egui::EguiPlugin); // added by framepace

    app.add_plugins((
        anim::BonusAnimPlugin,
        bg::BgPlugin,
        gameplay::GameplayPlugin,
        input::InputPlugin,
        level::LevelPlugin,
        menu::MenuPlugin,
        my_ldtk::MyLdtkPlugin,
        root::RootPlugin,
        state::StatePlugin,
    ));

    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::DebugPlugin);
    }

    app.run();
}
