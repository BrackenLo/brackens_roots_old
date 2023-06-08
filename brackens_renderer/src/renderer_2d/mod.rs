//===============================================================

use brackens_assets::HandleID;

pub use {
    assets::{RendererTexture, Texture},
    renderer::{Renderer2D, TextureRenderer},
    renderer_components::{RawTextureInstance, TextureDrawBuffer},
};

pub mod assets;
pub mod renderer;
pub mod renderer_components;
pub mod tools;

//===============================================================

pub type TextureID = HandleID<RendererTexture>;

//===============================================================

//===============================================================
