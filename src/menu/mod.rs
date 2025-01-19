use crate::prelude::*;

#[derive(Resource, Debug, Default, Reflect)]
pub struct MenuState {
    pub kind: MenuKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, Default)]
pub enum MenuKind {
    #[default]
    Title,
    Levels,
}

fn setup_egui_visuals(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn menu_ui(mut contexts: EguiContexts, mut menu_state: ResMut<MenuState>, mut commands: Commands) {
    let ctx = contexts.ctx_mut();

    match menu_state.kind {
        MenuKind::Title => {
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("PLAY").clicked() {
                            menu_state.kind = MenuKind::Levels;
                        }
                    });
                });
        }
        MenuKind::Levels => {
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("SCRATCH").clicked() {
                            commands.trigger(LoadLevel::lid("bad_lid"));
                        }
                        if ui.button("BACK").clicked() {
                            menu_state.kind = MenuKind::Title;
                        }
                    });
                });
        }
    }
}

fn on_enter() {}

fn on_exit() {}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuState::default());
        app.add_systems(Startup, setup_egui_visuals);

        app.add_systems(OnEnter(MetaState::Menu), on_enter);
        app.add_systems(Update, menu_ui.run_if(in_state(MetaState::Menu)));
        app.add_systems(OnExit(MetaState::Menu), on_exit);
    }
}
