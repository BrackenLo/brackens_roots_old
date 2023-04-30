//===============================================================

use std::collections::{HashMap, HashSet};

use brackens_tools::{
    asset_manager::{Handle, HandleID},
    general::Transform,
    renderer::{
        render_tools,
        texture_renderer::{self, RawTextureInstance},
    },
    wgpu,
};
use shipyard::{Component, Unique};

pub use brackens_tools::renderer::{
    texture::LoadedTexture, texture_renderer::TextureDrawCall as FinalTextureDrawCall,
};

//===============================================================

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================

pub struct TextureDrawCall {
    pub(crate) handle: Handle<LoadedTexture>,
    pub(crate) to_draw: Vec<RawTextureInstance>,
}

#[derive(Unique)]
pub struct TextureRenderer {
    pub(crate) renderer: texture_renderer::TextureRenderer,
    pub(crate) should_render: Vec<HandleID>,

    pub(crate) final_draw_calls: HashMap<HandleID, FinalTextureDrawCall>,
}
impl TextureRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            renderer: texture_renderer::TextureRenderer::new(device, config.format),
            should_render: vec![],

            final_draw_calls: HashMap::new(),
        }
    }

    pub(crate) fn draw_texture(
        &mut self,
        texture_handle: &Handle<LoadedTexture>,
        transform: Transform,
    ) {
        // let id
    }

    pub(crate) fn remove_texture(&mut self, texture_handle: HandleID) {
        todo!()
    }

    pub(crate) fn process_texture(&mut self) {
        self.final_draw_calls.clear();

        // let mut prev_render_handles: HashSet<_> = self.prev_handles.drain().collect();

        todo!()
    }

    pub(crate) fn render(&mut self, render_tools: &mut render_tools::RenderPassTools) {
        // self.renderer.render(render_tools, &self.final_draw_calls);
    }
}

//===============================================================

#[derive(Component)]
pub struct Visible(pub bool);

#[derive(Component)]
pub struct Texture {
    pub(crate) handle: Handle<LoadedTexture>,
}

//===============================================================
