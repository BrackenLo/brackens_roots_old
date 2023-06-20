//===============================================================

use downcast_rs::DowncastSync;

pub mod asset_storage;
pub mod asset_storage_single;
pub mod default_implementations;
pub mod file_loading;
pub mod handle;

//===============================================================

pub use asset_storage::AssetStorageX;
pub use asset_storage_single::AssetStorageSingle;
pub use handle::{Handle, HandleID};

//===============================================================

pub(crate) type SenderType<T> = crossbeam::channel::Sender<T>;
pub(crate) type ReceiverType<T> = crossbeam::channel::Receiver<T>;

//===============================================================

pub trait Asset: Send + Sync + DowncastSync {
    fn asset_name(&self) -> &str;
}

pub trait AssetFileLoadable: Asset {
    fn load_from_file(path: &str) -> Self;
    fn load_default() -> Self;
}

//===============================================================
