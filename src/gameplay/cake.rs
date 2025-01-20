use crate::prelude::*;

#[derive(Bundle)]
struct CakeBundle<const HBOX_MARKER: HBoxMarker> {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl<const HBOX_MARKER: HBoxMarker> MyLdtkIntCellValue for CakeBundle<HBOX_MARKER> {
    type Root = LevelPlatformRoot;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        let hbox = HBox::new(8, 8).with_marker(HBOX_MARKER);
        Self {
            name: Name::new(format!("Cake_{HBOX_MARKER}")),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, hbox),
        }
    }
}

pub(super) fn register_cake(app: &mut App) {
    MyLdtkIntCellLayer::new("CakeStatic", MainStaticLayer).register(app);
    // MyLdtkIntCellLayer::new("DirtDetail", MainDetailLayer).register(app);

    macro_rules! add_cake {
        ($(($value:literal, $marker:ident)$(,)?)+) => {
            $(
            app.add_plugins(MyLdtkIntCellValuePlugin::<CakeBundle<$marker>>::single(
                "CakeStatic",
                $value,
            ));
            )+
        };
    }
    add_cake!(
        (1, HBOX_CAKE_GREEN),
        (2, HBOX_CAKE_BLUE),
        (3, HBOX_CAKE_PINK),
        (4, HBOX_CAKE_RED),
    );
}
