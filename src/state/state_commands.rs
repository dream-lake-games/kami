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
) {
    let kind = trigger.kind.clone();
    *menu_state = MenuState { kind };
    next_meta_state.set(MetaState::Menu);
}

#[derive(Event)]
pub struct LoadLevel {
    lid: String,
}
impl LoadLevel {
    pub fn lid<S: AsRef<str>>(lid: S) -> Self {
        Self {
            lid: lid.as_ref().to_string(),
        }
    }
}
fn handle_load_level(
    trigger: Trigger<LoadLevel>,
    mut level_state: ResMut<LevelState>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let lid = trigger.lid.clone();
    *level_state = LevelState { lid, paused: false };
    next_meta_state.set(MetaState::LevelLoading);
}

pub(super) fn register_state_commands(app: &mut App) {
    app.add_observer(handle_load_menu);
    app.add_observer(handle_load_level);
}
