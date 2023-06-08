//===============================================================

use brackens_assets::asset_storage;
use log::info;
use shipyard::{Unique, World};

pub use brackens_assets::{Asset, Handle, HandleID};

//===============================================================

pub trait AssetLoadable: Asset {
    fn load_asset(world: &World, path: &str) -> Result<Self, AssetLoadError>
    where
        Self: Sized;
}

pub struct AssetLoadError;

#[derive(Unique, Default)]
pub struct AssetStorage<T: Asset> {
    inner: asset_storage::AssetStorage<T>,
}

impl<T> AssetStorage<T>
where
    T: Asset,
{
    //----------------------------------------------

    pub fn new() -> Self {
        let inner = asset_storage::AssetStorage::new();

        Self { inner }
    }

    //----------------------------------------------

    #[inline]
    pub fn add_asset(&mut self, asset: T) -> Handle<T> {
        self.inner.add_asset(asset)
    }

    #[inline]
    pub fn get_handle(&self, id: &HandleID<T>) -> Option<Handle<T>> {
        self.inner.get_handle(id)
    }

    #[inline]
    pub fn get_data(&self, id: &HandleID<T>) -> Option<&T> {
        self.inner.get_data(id)
    }

    //----------------------------------------------

    #[inline]
    pub fn get_just_added(&self) -> Vec<Handle<T>> {
        self.inner.get_just_added()
    }

    #[inline]
    pub fn get_removed_assets(&self) -> &Vec<HandleID<T>> {
        self.inner.get_removed_assets()
    }

    //----------------------------------------------

    #[inline]
    pub fn tick(&mut self) {
        self.inner.tick();
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
        if let Some(handle) = self.inner.get_loaded_file(&path) {
            return Ok(handle);
        }

        match T::load_asset(world, &path) {
            Ok(data) => {
                let handle = self.inner.add_asset_file(data, path);
                Ok(handle)
            }
            Err(e) => Err(e),
        }
    }
}

//===============================================================
