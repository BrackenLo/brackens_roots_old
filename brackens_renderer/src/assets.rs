//===============================================================

use brackens_assets::Asset;

use crate::textures::{LoadedTexture, Texture};

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
