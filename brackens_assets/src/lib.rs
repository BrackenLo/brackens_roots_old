// A lot of stuff found here is heavily inspired by the Bevy Engine. I wish I was that smart to figure this stuff out. I have learnt a lot though.
//===============================================================

use downcast_rs::DowncastSync;

pub mod asset_handle;
pub mod asset_storage;
pub mod asset_storage_generic;
pub mod default_implementations;
pub mod file_loading;
pub mod loadable_asset_storage;

//===============================================================

pub use asset_handle::{Handle, HandleID};
pub use asset_storage::AssetStorage;

//===============================================================

pub trait Asset: 'static + Send + Sync + DowncastSync {
    // fn asset_name() -> &'static str;
    fn asset_name(&self) -> &str;
}

pub trait AssetLoadableData: Asset {
    type Data;

    fn load_from_file(path: String, data: Self::Data) -> Self;
}

pub trait AssetLoadable<T>: Asset {
    fn load_from_file(path: String, data: T) -> Self;
    fn load_default(data: T) -> Self;
}

//===============================================================
