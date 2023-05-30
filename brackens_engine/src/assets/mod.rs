//===============================================================

use std::collections::HashMap;

use brackens_assets::asset_storage;
use log::info;
use shipyard::{Unique, World};

pub use brackens_assets::{Asset, Handle, HandleID};

//===============================================================

pub trait AssetLoadable: Asset {
    fn load_asset(world: &World) -> Result<Self, AssetLoadError>
    where
        Self: Sized;
}

pub struct AssetLoadError;

#[derive(Unique)]
pub struct AssetStorage<T: Asset> {
    inner: asset_storage::AssetStorage<T>,

    // Hashmap containing path to asset as a key. Used to check if data
    // is already loaded and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleID<T>>,
    // Hashmap that is the opposite of loaded_paths used to access the file
    // path when unloading the data.
    // If a collection did key <-> key instead of key -> value exists, using
    // that would be preferable.
    asset_paths: HashMap<HandleID<T>, String>,
}

impl<T> AssetStorage<T>
where
    T: Asset,
{
    //----------------------------------------------

    pub fn new() -> Self {
        let inner = asset_storage::AssetStorage::new();

        Self {
            inner,
            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),
        }
    }

    //----------------------------------------------

    pub fn add_asset(&mut self, asset: T) -> Handle<T> {
        self.inner.add_asset(asset)
    }

    pub fn get_handle(&self, id: &HandleID<T>) -> Option<Handle<T>> {
        self.inner.get_handle(id)
    }

    pub fn get_loaded_file(&self, path: &str) -> Option<Handle<T>> {
        if let Some(id) = self.loaded_paths.get(path) {
            info!(
                "Retrieving previously loaded {} asset with id {}",
                T::asset_name(),
                id
            );
            let data_access = self.inner.get_loaded().get(id).unwrap().clone();
            return Some(Handle::strong(
                *id,
                self.inner.get_sender().clone(),
                data_access,
            ));
        }

        None
    }

    //----------------------------------------------

    pub fn get_just_added(&self) -> Vec<Handle<T>> {
        self.inner.get_just_added()
    }

    pub fn get_removed_assets(&self) -> &Vec<HandleID<T>> {
        self.inner.get_removed_assets()
    }

    //----------------------------------------------

    pub fn tick(&mut self) {
        self.inner.check_asset_changes();

        for to_remove in self.inner.get_removed_assets() {
            self.loaded_paths
                .remove(&self.asset_paths.remove(&to_remove).unwrap());
        }

        self.inner.remove_pending_assets();
        self.inner.clear_just_added();
    }

    //----------------------------------------------
}

impl<T> AssetStorage<T>
where
    T: AssetLoadable,
{
    pub fn load_from_file(
        &mut self,
        world: &mut World,
        path: String,
    ) -> Result<Handle<T>, AssetLoadError> {
        info!("Loading new {} asset from path {}", T::asset_name(), path);

        // Check if file is already loaded. If so, we can create a new handle to the existing data
        if let Some(handle) = self.get_loaded_file(&path) {
            return Ok(handle);
        }

        match T::load_asset(world) {
            Ok(data) => {
                let handle = self.inner.add_asset(data);

                self.loaded_paths.insert(path.clone(), handle.id());
                self.asset_paths.insert(handle.id(), path);

                Ok(handle)
            }
            Err(e) => Err(e),
        }
    }
}

//===============================================================
