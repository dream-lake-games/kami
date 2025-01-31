use crate::prelude::*;

#[derive(Event)]
pub struct LoadMenu {
    kind: MenuKind,
}
impl LoadMenu {
    pub fn kind(kind: MenuKind) -> Self {
        Self { kind }
    }
}
fn handle_load_menu(
    trigger: Trigger<LoadMenu>,
    mut menu_state: ResMut<MenuState>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    mut commands: Commands,
    mut transition_anim: Query<&mut AnimMan<LoadingAnim>>,
) {
    commands.trigger(DespawnLevel);
    commands.trigger(UnloadMyLdtkLevel);
    let kind = trigger.kind.clone();
    *menu_state = MenuState { kind };
    next_meta_state.set(MetaState::Menu);
    transition_anim
        .get_single_mut()
        .map(|mut thing| {
            thing.set_state(LoadingAnim::FakeDots);
        })
        .ok();
}

#[derive(Event)]
pub struct LoadLevel {
    lid: String,
    skip_intro_messages: bool,
}
impl LoadLevel {
    pub fn lid<S: AsRef<str>>(lid: S) -> Self {
        Self {
            lid: lid.as_ref().to_string(),
            skip_intro_messages: false,
        }
    }
    pub fn with_skip_intro_messages(mut self, val: bool) -> Self {
        self.skip_intro_messages = val;
        self
    }
}
fn handle_load_level(
    trigger: Trigger<LoadLevel>,
    mut level_state: ResMut<LevelState>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let lid = trigger.lid.clone();
    *level_state = LevelState {
        lid: lid.clone(),
        paused: false,
        rect: default(),
        score: 0,
        intro_messages: if trigger.skip_intro_messages {
            vec![]
        } else {
            get_level_defn(lid).intro_messages
        },
    };
    next_meta_state.set(MetaState::LevelLoading);
}

#[derive(Event)]
pub struct NextLevel;
fn handle_next_level(
    _trigger: Trigger<NextLevel>,
    mut commands: Commands,
    level_state: Res<LevelState>,
    mut post_game_messages: ResMut<PostGameMessages>,
) {
    let ix = LEVEL_DEFNS
        .iter()
        .position(|level_defn| level_defn.lid == level_state.lid)
        .unwrap();
    if ix + 1 < LEVEL_DEFNS.len() {
        commands.trigger(LoadLevel::lid(LEVEL_DEFNS[ix + 1].lid.clone()));
    } else {
        *post_game_messages = PostGameMessages(vec![
            (
                "MISSION SUCCESS".to_string(),
                "It worked! The forest is none the wiser. The sleeper agents have been planted."
                    .to_string(),
            ),
            (
                "MISSION SUCCESS".to_string(),
                "Once the presence of these bombs is revealed, no one will dare question the squirrel regime."
                    .to_string(),
            ),
            (
                "GLORY UPON YOU".to_string(),
                "You are the weapon."
                    .to_string(),
            ),
            (
                "CREDITS".to_string(),
                "Design, programming, visual art by DREAM LAKE GAMES."
                    .to_string(),
            ),
            (
                "CREDITS".to_string(),
                "Sound effects from CC0, by Juhani Junkala."
                    .to_string(),
            ),
            (
                "CREDITS".to_string(),
                "Music, 'A Bag of Chips' (also CC0) by Zane Little."
                    .to_string(),
            ),
            (
                "CREDITS".to_string(),
                "Thank YOU for playing!"
                    .to_string(),
            ),
        ]);
        commands.trigger(LoadMenu::kind(MenuKind::PostGame));
    }
}

#[derive(Event)]
pub struct StartLevel;
fn handle_start_level(
    _trigger: Trigger<StartLevel>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    next_meta_state.set(MetaState::Level);
}

pub(super) fn register_state_commands(app: &mut App) {
    app.add_observer(handle_load_menu);
    app.add_observer(handle_load_level);
    app.add_observer(handle_next_level);
    app.add_observer(handle_start_level);
}
