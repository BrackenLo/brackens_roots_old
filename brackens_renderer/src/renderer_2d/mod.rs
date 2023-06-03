//===============================================================

use brackens_assets::HandleID;

pub use {
    assets::{RendererTexture, Texture},
    texture_renderer::{RawTextureInstance, RawTextureVertex, TextureDrawCall, TextureRenderer},
};

pub mod assets;
pub mod renderer;
pub mod renderer_components;
pub mod texture_renderer;

//===============================================================

pub type TextureID = HandleID<RendererTexture>;

//===============================================================
