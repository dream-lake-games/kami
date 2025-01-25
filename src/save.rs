use crate::prelude::*;

/// Persistent resource-able
/// Yeah it's a terrible name
pub trait Persable: Resource + Reflect + Serialize + for<'de> Deserialize<'de> + Default {
    const KEY: &'static str;
}

#[derive(Resource, Reflect)]
pub struct Pers<R: Persable> {
    inner: R,
}
impl<R: Persable> Pers<R> {
    pub fn load(store: &mut PkvStore) -> Self {
        let inner = match store.get::<R>(R::KEY) {
            Ok(data) => data,
            Err(_) => {
                let inner_inner = R::default();
                if let Err(e) = store.set(R::KEY, &inner_inner) {
                    warn!("hmm couldn't set initial state for {}: {e:?}", R::KEY);
                }
                inner_inner
            }
        };
        Self { inner }
    }

    // Gets the current resource value from memory. Does NOT load from disk.
    pub fn get(&self) -> &R {
        &self.inner
    }

    /// Updates the resource but DOES NOT write it to disk
    pub fn set(&mut self, res: R) {
        self.inner = res;
    }

    /// TODO: I know bevy has special support for doing things like file IO, but I haven't
    /// learned it yet. This (imo) is a mild problem across the game
    /// I should learn how to do that
    /// For now call this sparingly
    pub fn save(&self, store: &mut PkvStore) {
        if let Err(e) = store.set(R::KEY, &self.inner) {
            warn!("couldn't save {}: {e:?}", R::KEY);
        }
    }
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Clone)]
pub struct SaveData {
    /// Maps lid to LevelSave
    pub map: HashMap<String, LevelSave>,
    pub menu_ix: u32,
}
impl Persable for SaveData {
    const KEY: &'static str = "SaveData";
}

#[derive(Resource, Reflect, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub music_volume: f32,
    pub effect_volume: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            music_volume: 0.5,
            effect_volume: 0.5,
        }
    }
}
impl Persable for Settings {
    const KEY: &'static str = "Settings";
}

pub fn new_store() -> PkvStore {
    return PkvStore::new("dreamlakegames", "kami");
}

pub(super) struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        let mut store = new_store();
        app.insert_resource(Pers::<SaveData>::load(&mut store));
        app.insert_resource(Pers::<Settings>::load(&mut store));
        app.insert_resource(store);
    }
}
