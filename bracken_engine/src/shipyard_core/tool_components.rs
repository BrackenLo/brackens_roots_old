//===============================================================

// use std::{any::TypeId, collections::HashMap};

use brackens_tools::{asset_manager, general};
use shipyard::{Component, Unique};

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

// #[derive(shipyard::Component)]
// pub struct Handle<T>(pub(crate) asset_manager::Handle<T>)
// where
//     T: asset_manager::Asset;

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

#[derive(Component)]
pub struct Timer(pub(crate) general::Timer);
impl Timer {
    //--------------------------------------------------

    pub fn new(duration: f32, repeating: bool) -> Self {
        Self(general::Timer::new(duration, repeating))
    }
    pub fn restart(&mut self) {
        self.0.restart()
    }
    pub fn progress(&self) -> f32 {
        self.0.progress()
    }

    //--------------------------------------------------

    pub fn duration(&self) -> f32 {
        self.0.duration
    }

    pub fn repeating(&self) -> bool {
        self.0.repeating
    }

    pub fn paused(&self) -> bool {
        self.0.paused
    }

    pub fn finished(&self) -> bool {
        self.0.is_finished()
    }

    //--------------------------------------------------

    pub fn set_duration(&mut self, val: f32) {
        self.0.duration = val;
    }

    pub fn set_repeating(&mut self, val: bool) {
        self.0.repeating = val;
    }

    pub fn set_paused(&mut self, val: bool) {
        self.0.paused = val;
    }

    //--------------------------------------------------
}

//===============================================================
