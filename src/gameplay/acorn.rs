use crate::prelude::*;

#[derive(Bundle)]
struct AcornBundle {
    transform: Transform,
    pos: Pos,
    anim: AnimMan<AcornAnim>,
    ttx: TriggerTx,
}
impl MyLdtkEntity for AcornBundle {
    type Root = LevelActiveRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            transform: pos.to_transform(ZIX_ITEMS),
            pos,
            anim: default(),
            ttx: TriggerTx::single(TriggerTxKind::Acorn, HBox::new(12, 12)),
        }
    }
}

fn update_acorns(
    mut acorns: Query<(&TriggerTx, &mut AnimMan<AcornAnim>, &Pos)>,
    mut level_state: ResMut<LevelState>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
    detail_root: Res<LevelDetailRoot>,
) {
    for (ttx, mut anim, pos) in &mut acorns {
        if anim.get_state() != AcornAnim::Pulse {
            continue;
        }
        if trigger_colls
            .get_refs(&ttx.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Chef)
        {
            anim.set_state(AcornAnim::Pop);
            level_state.score += 150;
            commands
                .spawn((EphemeralAnim::new(
                    ScoreAnim::P150,
                    false,
                    pos.clone().translated(Vec2::Y * 8.0),
                    ZIX_CHEF + 4.0,
                ),))
                .set_parent(detail_root.eid());
            commands
                .spawn((SoundEffect::ScoreP150,))
                .set_parent(detail_root.eid());
        }
    }
}

pub(super) fn register_acorn(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<AcornBundle>::new("Entities", "Acorn"));
    app.add_systems(
        Update,
        update_acorns
            .after(PhysicsSet)
            .run_if(in_state(MetaState::Level)),
    );
}
