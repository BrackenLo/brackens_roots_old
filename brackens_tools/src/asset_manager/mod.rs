// A lot of stuff found here is heavily inspired by the Bevy Engine. I wish I was that smart to figure this stuff out. I have learnt a lot though.
//===============================================================

pub mod asset_handle;
pub mod asset_storage;
pub mod default_implementations;

//===============================================================

pub use asset_handle::{Handle, HandleID};
pub use asset_storage::AssetStorage;

//===============================================================

pub trait Asset: 'static + Send + Sync {
    fn asset_name() -> &'static str;
}

pub trait AssetLoadable: Asset {
    fn load_from_file(path: String) -> Self;
    fn load_default() -> Self;
}

//===============================================================
