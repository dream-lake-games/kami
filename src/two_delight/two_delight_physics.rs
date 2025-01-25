use crate::prelude::*;

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq)]
pub enum TriggerRxKind {
    Chef,
}
impl TriggerKind for TriggerRxKind {}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq)]
pub enum TriggerTxKind {
    Acorn,
}
impl TriggerKind for TriggerTxKind {}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum BulletTimeSpeed {
    #[default]
    Normal,
    LiftTransition,
    LiftSlow,
}
impl BulletTimeClass for BulletTimeSpeed {
    fn to_factor(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::LiftTransition => 0.84,
            Self::LiftSlow => 0.68,
        }
    }
}

// I _highly_ recommend you create type aliases here to cut back on some verbosity
pub type TriggerRx = TriggerRxGeneric<TriggerRxKind>;
pub type TriggerTx = TriggerTxGeneric<TriggerTxKind>;
pub type TriggerColls = TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>;
#[expect(dead_code)]
pub type TriggerCollRec = TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>;
pub type BulletTime = BulletTimeGeneric<BulletTimeSpeed>;
pub type PhysicsPlugin = PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind, BulletTimeSpeed>;
