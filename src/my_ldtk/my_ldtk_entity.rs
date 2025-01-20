use crate::prelude::*;

pub trait MyLdtkEntity: Bundle {
    type Root: RootKind;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, iid: String) -> Self;
}

#[derive(Component, Default)]
struct MyLdtkEntityWrapper<B: MyLdtkEntity> {
    _pd: std::marker::PhantomData<B>,
    _blocker: BlockMyLdtkLoad,
    fields: HashMap<String, FieldValue>,
    iid: String,
}
impl<B: MyLdtkEntity> LdtkEntity for MyLdtkEntityWrapper<B> {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            _pd: default(),
            _blocker: BlockMyLdtkLoad::ticks(30),
            fields: entity_instance
                .field_instances
                .clone()
                .into_iter()
                .map(|fi| (fi.identifier, fi.value))
                .collect(),
            iid: entity_instance.iid.clone(),
        }
    }
}

fn post_ldtk_entity_blessing<B: MyLdtkEntity>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &MyLdtkEntityWrapper<B>)>,
    root: Res<B::Root>,
) {
    for (ldtk_eid, gt, wrapper) in &wrappers {
        let pos = Pos::new(gt.translation().x, gt.translation().y);
        let bund = B::from_ldtk(pos, &wrapper.fields, wrapper.iid.clone());
        commands.spawn(bund).set_parent(root.eid());
        commands.entity(ldtk_eid).remove::<MyLdtkEntityWrapper<B>>();
    }
}

pub struct MyLdtkEntityPlugin<B: MyLdtkEntity> {
    _pd: std::marker::PhantomData<B>,
    layer_id: &'static str,
    entity_id: &'static str,
}
impl<B: MyLdtkEntity> MyLdtkEntityPlugin<B> {
    pub fn new(layer_id: &'static str, entity_id: &'static str) -> Self {
        Self {
            layer_id,
            entity_id,
            _pd: default(),
        }
    }
}
impl<B: MyLdtkEntity> Plugin for MyLdtkEntityPlugin<B> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<MyLdtkEntityWrapper<B>>(
            &self.layer_id,
            &self.entity_id,
        );
        app.add_systems(PreUpdate, post_ldtk_entity_blessing::<B>);
    }
}
