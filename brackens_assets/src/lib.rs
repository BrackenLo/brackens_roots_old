//===============================================================

use downcast_rs::DowncastSync;

pub mod asset_handle;
pub mod asset_storage;
pub mod asset_storage_generic;
pub mod default_implementations;
pub mod file_loading;
pub mod handle_x;

//===============================================================

pub use asset_handle::{Handle, HandleID};
pub use asset_storage::AssetStorage;

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

pub trait AssetFileLoadableData: Asset {}

//===============================================================
