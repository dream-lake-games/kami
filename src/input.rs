use crate::prelude::*;

#[derive(SystemSet, std::hash::Hash, Debug, PartialEq, Eq, Clone)]
pub struct InputSet;

#[derive(Clone, Copy, Debug, std::hash::Hash, PartialEq, Eq)]
pub enum ButtKind {
    A,
}
impl ButtKind {
    pub fn all() -> Vec<Self> {
        vec![Self::A]
    }
}

#[derive(Resource, Default)]
pub struct ButtInput {
    just_pressed: HashSet<ButtKind>,
    pressed: HashSet<ButtKind>,
}
impl ButtInput {
    fn update(&mut self, keyboad: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) {
        self.just_pressed.clear();
        self.pressed.clear();
        for kind in ButtKind::all() {
            let pressed = match kind {
                ButtKind::A => keyboad.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left),
            };
            let just_pressed = match kind {
                ButtKind::A => {
                    keyboad.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)
                }
            };
            if just_pressed {
                self.just_pressed.insert(kind);
            }
            if pressed {
                self.pressed.insert(kind);
            }
        }
    }

    pub fn just_pressed(&self, kind: ButtKind) -> bool {
        self.just_pressed.contains(&kind)
    }
    pub fn pressed(&self, kind: ButtKind) -> bool {
        self.pressed.contains(&kind)
    }
}

fn update_butt_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut butt_input: ResMut<ButtInput>,
) {
    butt_input.update(&keyboard, &mouse);
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ButtInput::default());

        app.add_systems(Update, update_butt_input.in_set(InputSet));
    }
}
