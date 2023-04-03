//===============================================================

// use std::{any::TypeId, collections::HashMap};

use brackens_tools::asset_manager::{Asset, AssetStorage};
use shipyard::Unique;

//===============================================================

#[derive(Unique)]
pub struct AssetStorageUnique<T>(AssetStorage<T>)
where
    T: Asset + 'static;

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
