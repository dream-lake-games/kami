use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, Default)]
pub enum MenuKind {
    #[default]
    Title,
    Levels,
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub kind: MenuKind,
}
