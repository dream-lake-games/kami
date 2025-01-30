use crate::prelude::*;

#[derive(Bundle)]
struct DirtRoughBundle {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl MyLdtkIntCellValue for DirtRoughBundle {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(8, 8).with_marker(HBOX_DIRT_ROUGH);
        Self {
            name: Name::new("DirtRough"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, hbox),
        }
    }
}

#[derive(Bundle)]
struct DirtSmoothBundle {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl MyLdtkIntCellValue for DirtSmoothBundle {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(8, 8).with_marker(HBOX_DIRT_SMOOTH);
        Self {
            name: Name::new("DirtSmooth"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, hbox),
        }
    }
}

#[derive(Bundle)]
struct SpikesBundle {
    name: Name,
    pos: Pos,
    ttx: TriggerTx,
}
impl MyLdtkIntCellValue for SpikesBundle {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(5, 5);
        Self {
            name: Name::new("Spikes"),
            pos,
            ttx: TriggerTx::single(TriggerTxKind::Spikes, hbox),
        }
    }
}

pub(super) fn register_platforms(app: &mut App) {
    MyLdtkIntCellLayer::new("DirtStatic", MainStaticLayer).register(app);
    MyLdtkIntCellLayer::new("DirtDetail", MainStaticLayer).register(app);
    MyLdtkIntCellLayer::new("DirtAmbience", MainAmbienceLayer).register(app);
    MyLdtkIntCellLayer::new("DirtAmbienceAutoAmbience", MainAmbienceLayer).register(app);
    MyLdtkIntCellLayer::new("DirtAmbienceAutoDetail", MainDetailLayer).register(app);

    app.add_plugins(MyLdtkIntCellValuePlugin::<DirtRoughBundle>::multiple(
        "DirtStatic",
        vec![1, 2],
    ));
    app.add_plugins(MyLdtkIntCellValuePlugin::<DirtSmoothBundle>::multiple(
        "DirtStatic",
        vec![3],
    ));

    MyLdtkIntCellLayer::new("SpikesStatic", MainStaticLayer).register(app);

    app.add_plugins(MyLdtkIntCellValuePlugin::<SpikesBundle>::single(
        "SpikesStatic",
        1,
    ));
}
