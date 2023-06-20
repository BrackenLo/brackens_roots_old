//===============================================================

use brackens_assets::{
    asset_storage_generic::{AssetStorageError, AssetStorageX},
    handle_x::HandleInner,
    Asset, AssetFileLoadable,
};
use shipyard::Unique;

pub use brackens_assets::handle_x::{Handle, HandleID};

//===============================================================

#[derive(Unique, Default)]
pub struct AssetStorage(AssetStorageX);
impl AssetStorage {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn load_from_file<T: AssetFileLoadable>(
        &mut self,
        path: &str,
    ) -> anyhow::Result<Handle<T>> {
        self.0.load_from_file(path)
    }

    #[inline]
    pub fn load_from_data<T: Asset, F: Fn(&[u8]) -> T>(
        &mut self,
        path: &str,
        load_data: F,
    ) -> anyhow::Result<Handle<T>> {
        self.0.load_from_data(path, load_data)
    }

    #[inline]
    pub fn is_file_loaded(&self, path: &str) -> bool {
        self.0.is_file_loaded(path)
    }

    #[inline]
    pub fn get_file_loaded<T: Asset>(&self, path: &str) -> Result<Handle<T>, AssetStorageError> {
        self.0.get_file_loaded(path)
    }

    #[inline]
    pub fn get_handle<T: Asset, HI: Into<HandleInner>>(
        &self,
        id: HI,
    ) -> Result<Handle<T>, AssetStorageError> {
        self.0.get_handle(id)
    }

    #[inline]
    pub fn get_data<T: Asset>(&self, id: HandleID<T>) -> Result<&T, AssetStorageError> {
        self.0.get_data(id)
    }

    #[inline]
    pub fn get_data_raw<T: Asset, HI: Into<HandleInner>>(
        &self,
        id: HI,
    ) -> Result<&T, AssetStorageError> {
        self.0.get_data_raw(id)
    }

    #[inline]
    pub fn tick(&mut self) {
        self.0.tick()
    }
}

//===============================================================
