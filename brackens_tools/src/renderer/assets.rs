//===============================================================

use crate::asset_manager::Asset;

use super::texture::{LoadedTexture, Texture};

//===============================================================

impl Asset for Texture {
    fn asset_name() -> &'static str {
        "WgpuTexture"
    }
}

impl Asset for LoadedTexture {
    fn asset_name() -> &'static str {
        "WgpuLoadedTexture"
    }
}

//===============================================================
