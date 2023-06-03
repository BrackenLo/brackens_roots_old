//===============================================================

use std::{collections::HashMap, marker::PhantomData};

use log::info;

use crate::{AssetLoadable, AssetStorage, Handle, HandleID};

//===============================================================

pub struct LoadableAssetStorage<D, T: AssetLoadable<D>> {
    inner: AssetStorage<T>,
    phantom: PhantomData<D>,

    // Hashmap containing path to asset as a key. Used to check if data
    // is already loaded and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleID<T>>,
    // Hashmap that is the opposite of loaded_paths used to access the file
    // path when unloading the data.
    // If a collection did key <-> key instead of key -> value exists, using
    // that would be preferable.
    asset_paths: HashMap<HandleID<T>, String>,

    load_path: String,
}

impl<D, T> LoadableAssetStorage<D, T>
where
    T: AssetLoadable<D>,
{
    pub fn new() -> Self {
        let inner = AssetStorage::new();

        Self {
            inner,
            phantom: PhantomData,
            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),

            load_path: "".into(),
        }
    }

    //----------------------------------------------

    pub fn get_file_loaded(&self, path: &str) -> Option<Handle<T>> {
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

    pub fn load_from_file(&mut self, path: String, data: D) -> Handle<T> {
        info!("Loading new {} asset from path {}", T::asset_name(), path);

        // Check if file is already loaded. If so, we can create a new handle to the existing data.
        if let Some(handle) = self.get_file_loaded(&path) {
            return handle;
        }

        // Otherwise, load and store the data accordingly
        let data = T::load_from_file(format!("{}{}", self.load_path, &path), data);
        let handle = self.inner.add_asset(data);

        self.loaded_paths.insert(path.clone(), handle.id());
        self.asset_paths.insert(handle.id(), path);

        handle
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

    pub fn load_asset(&mut self, asset: T) -> Handle<T> {
        self.inner.add_asset(asset)
    }

    //----------------------------------------------
}

//===============================================================
