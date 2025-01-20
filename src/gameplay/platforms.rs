use crate::prelude::*;

#[derive(Bundle)]
struct DirtBundle {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl MyLdtkIntCellValue for DirtBundle {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(8, 8).with_marker(HBOX_DIRT);
        Self {
            name: Name::new("Dirt"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, hbox),
        }
    }
}

#[derive(Bundle)]
struct SmoothBundle {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl MyLdtkIntCellValue for SmoothBundle {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(8, 8).with_marker(HBOX_SMOOTH);
        Self {
            name: Name::new("Smooth"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, hbox),
        }
    }
}

pub(super) fn register_platforms(app: &mut App) {
    MyLdtkIntCellLayer::new("DirtStatic", MainStaticLayer).register(app);
    app.add_plugins(MyLdtkIntCellValuePlugin::<DirtBundle>::multiple(
        "DirtStatic",
        vec![1, 2],
    ));
    MyLdtkIntCellLayer::new("DirtDetail", MainDetailLayer).register(app);

    MyLdtkIntCellLayer::new("SmoothStatic", MainStaticLayer).register(app);
    app.add_plugins(MyLdtkIntCellValuePlugin::<SmoothBundle>::multiple(
        "SmoothStatic",
        vec![1],
    ));
}
