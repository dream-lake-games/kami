use crate::prelude::*;

#[derive(Default, Resource)]
struct UiState {
    is_open: bool,
}

fn setup_egui_visuals(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn menu_ui(mut contexts: EguiContexts, mut menu_state: ResMut<MenuState>) {
    let ctx = contexts.ctx_mut();

    match menu_state.kind {
        MenuKind::Title => {
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        if ui.button("Play").clicked() {
                            println!("play");
                            menu_state.kind = MenuKind::Levels;
                        }
                        if ui.button("Settings").clicked() {
                            println!("Settings");
                        }
                    });
                });
        }
        MenuKind::Levels => {
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        if ui.button("BACK").clicked() {
                            menu_state.kind = MenuKind::Title;
                        }
                    });
                });
        }
    }
}

fn on_enter(mut commands: Commands) {
    commands.insert_resource(MenuState::default());
}

fn on_exit(mut commands: Commands) {
    commands.remove_resource::<MenuState>();
}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default());
        app.add_systems(Startup, setup_egui_visuals);

        app.add_systems(OnEnter(MetaState::Menu), on_enter);
        app.add_systems(Update, menu_ui.run_if(in_state(MetaState::Menu)));
        app.add_systems(OnExit(MetaState::Menu), on_exit);
    }
}
