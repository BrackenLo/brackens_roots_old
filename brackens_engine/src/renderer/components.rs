//===============================================================

use brackens_renderer::render_tools;

use shipyard::{Component, Unique};

pub use brackens_renderer::{
    renderer_2d::RendererTexture, renderer_2d::TextureDrawCall as FinalTextureDrawCall,
};

//===============================================================
// Core rendering Uniques

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================
// Shared Rendering Components

#[derive(Component)]
pub struct Visible {
    pub visible: bool,
}
impl Default for Visible {
    fn default() -> Self {
        Self { visible: true }
    }
}

//===============================================================
