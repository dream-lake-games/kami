use bevy::render::sync_world::RenderEntity;

use crate::prelude::*;

#[derive(Resource, Default)]
pub(super) struct MyLdtkIntCellLayerInfo {
    map: HashMap<String, RenderLayers>,
}

pub trait MyLdtkIntCellValue: Bundle {
    type Root: RootKind;
    fn from_ldtk(pos: Pos, value: i32) -> Self;
}

#[derive(Component, Default)]
struct MyLdtkIntCellWrapper<B: MyLdtkIntCellValue> {
    _pd: std::marker::PhantomData<B>,
    value: i32,
    _blocker: BlockMyLdtkLoad,
}
impl<B: MyLdtkIntCellValue> LdtkIntCell for MyLdtkIntCellWrapper<B> {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _layer_instance: &LayerInstance) -> Self {
        Self {
            _pd: default(),
            value: int_grid_cell.value,
            _blocker: BlockMyLdtkLoad::ticks(10),
        }
    }
}

#[derive(Component)]
struct LayerHandled;

fn post_ldtk_int_cell_layer_blessing(
    layer_info: Res<MyLdtkIntCellLayerInfo>,
    layer_q: Query<(Entity, &Name), (With<TilemapType>, Without<LayerHandled>)>,
    // These are the ldtk backgrounds. Useless.
    hooligans: Query<Entity, (With<RenderEntity>, Without<Name>, With<Sprite>)>,
    mut commands: Commands,
) {
    for (eid, name) in &layer_q {
        let Some(render_layers) = layer_info.map.get(name.as_str()) else {
            continue;
        };
        commands
            .entity(eid)
            .insert((LayerHandled, render_layers.clone()));
    }
    for hooligan in &hooligans {
        commands.entity(hooligan).despawn_recursive();
    }
}

fn post_ldtk_int_cell_value_blessing<B: MyLdtkIntCellValue>(
    mut commands: Commands,
    mut wrappers: Query<(Entity, &GlobalTransform, &MyLdtkIntCellWrapper<B>)>,
    root: Res<B::Root>,
) {
    for (ldtk_eid, gt, wrapper) in &mut wrappers {
        let pos = Pos::new(gt.translation().x, gt.translation().y);
        let bund = B::from_ldtk(pos, wrapper.value);
        commands.spawn(bund).set_parent(root.eid());
        commands
            .entity(ldtk_eid)
            .remove::<MyLdtkIntCellWrapper<B>>();
    }
}

#[derive(Clone)]
pub struct MyLdtkIntCellLayer {
    layer_id: String,
    render_layers: RenderLayers,
}
impl MyLdtkIntCellLayer {
    pub fn new<S: AsRef<str>, L: Layer>(layer_id: S, _l: L) -> Self {
        Self {
            layer_id: layer_id.as_ref().to_string(),
            render_layers: L::RENDER_LAYERS,
        }
    }
    pub fn register(&self, app: &mut App) {
        let data = self.clone();
        app.add_systems(
            OnEnter(MetaState::Setup),
            move |mut layer_info: ResMut<MyLdtkIntCellLayerInfo>| {
                if layer_info.map.contains_key(&data.layer_id) {
                    panic!(
                        "Registered the same ldtk int cell layer twice: {:?}",
                        data.layer_id
                    );
                }
                layer_info
                    .map
                    .insert(data.layer_id.clone(), data.render_layers.clone());
            },
        );
    }
}

pub struct MyLdtkIntCellValuePlugin<B: MyLdtkIntCellValue> {
    layer_id: &'static str,
    values: Vec<i32>,
    _pd: std::marker::PhantomData<B>,
}
impl<B: MyLdtkIntCellValue> MyLdtkIntCellValuePlugin<B> {
    pub fn single(layer_id: &'static str, value: i32) -> Self {
        Self {
            layer_id,
            values: vec![value],
            _pd: default(),
        }
    }
    pub fn multiple(layer_id: &'static str, values: Vec<i32>) -> Self {
        Self {
            layer_id,
            values,
            _pd: default(),
        }
    }
}
impl<B: MyLdtkIntCellValue> Plugin for MyLdtkIntCellValuePlugin<B> {
    fn build(&self, app: &mut App) {
        for value in &self.values {
            app.register_ldtk_int_cell_for_layer::<MyLdtkIntCellWrapper<B>>(&self.layer_id, *value);
        }
        app.add_systems(PreUpdate, post_ldtk_int_cell_value_blessing::<B>);
    }
}

pub(super) fn register_my_ldtk_int_cell(app: &mut App) {
    app.insert_resource(MyLdtkIntCellLayerInfo::default());
    app.add_systems(Update, post_ldtk_int_cell_layer_blessing);
}
