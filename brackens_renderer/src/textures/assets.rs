//===============================================================

use brackens_assets::Asset;

use super::{RendererTexture, Texture};

//===============================================================

impl Asset for Texture {
    fn asset_name() -> &'static str {
        "Wgpu Texture"
    }
}

impl Asset for RendererTexture {
    fn asset_name() -> &'static str {
        "Renderer Texture"
    }
}

//===============================================================
