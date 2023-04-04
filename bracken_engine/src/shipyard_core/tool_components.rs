//===============================================================

// use std::{any::TypeId, collections::HashMap};

use brackens_tools::asset_manager;
use shipyard::Unique;

//===============================================================

#[derive(Unique)]
pub struct AssetStorage<T>(pub(crate) asset_manager::AssetStorage<T>)
where
    T: asset_manager::Asset;

impl<T> AssetStorage<T>
where
    T: asset_manager::Asset,
{
    pub fn new() -> Self {
        Self(asset_manager::AssetStorage::new())
    }
}

#[derive(shipyard::Component)]
pub struct Handle<T>(pub(crate) asset_manager::Handle<T>)
where
    T: asset_manager::Asset;

//===============================================================

// pub trait MyAssetStorage {}
// impl<T> MyAssetStorage for AssetStorage<T> where T: Asset {}

// #[derive(Unique)]
// pub struct AssetStorageManager {
//     storages: HashMap<TypeId, Box<dyn MyAssetStorage>>,
// }
// impl AssetStorageManager {
//     pub fn add_storage<T: 'static>(&mut self, storage: Box<dyn MyAssetStorage>) {
//         let id = TypeId::of::<T>();
//         self.storages.insert(id, storage);
//     }
// }

//===============================================================
