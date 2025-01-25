use crate::prelude::*;

mod level_defns;
mod level_loading;

pub use level_defns::*;
pub use level_loading::DespawnLevel;

#[derive(Clone, Debug)]
pub struct LevelIntroMessage {
    pub title: String,
    pub content: String,
}
impl LevelIntroMessage {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct LevelState {
    pub lid: String,
    pub paused: bool,
    pub rect: Rect,
    pub score: i32,
    pub intro_messages: Vec<LevelIntroMessage>,
}

#[derive(Clone, Debug)]
pub struct LevelTierCutoff {
    pub one: i32,
    pub two: i32,
    pub three: i32,
}
impl LevelTierCutoff {
    pub fn new(one: i32, two: i32, three: i32) -> Self {
        Self { one, two, three }
    }
}

#[derive(Clone, Debug)]
pub struct LevelDefn {
    pub name: String,
    pub lid: String,
    pub intro_messages: Vec<LevelIntroMessage>,
    pub tiers: LevelTierCutoff,
}
impl LevelDefn {
    pub fn new(
        name: &str,
        lid: &str,
        intro_messages: Vec<LevelIntroMessage>,
        stars: LevelTierCutoff,
    ) -> Self {
        Self {
            name: name.to_string(),
            lid: lid.to_string(),
            intro_messages,
            tiers: stars,
        }
    }
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Clone)]
pub struct LevelSave {
    pub hiscore: Option<i32>,
}

pub fn physics_active(level_state: Option<Res<LevelState>>) -> bool {
    level_state
        .map(|ls| !ls.paused && ls.intro_messages.is_empty())
        .unwrap_or(false)
}

#[expect(dead_code)]
pub fn physics_inactive(level_state: Option<Res<LevelState>>) -> bool {
    !physics_active(level_state)
}

pub(super) struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelState::default());

        level_loading::register_level_loading(app);
    }
}
