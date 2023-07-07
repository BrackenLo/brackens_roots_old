//===============================================================

use std::collections::HashMap;

use brackens_renderer::{
    render_tools::{self, RenderPassTools as RenderPassToolsInner},
    renderer_2d::{tools::TextureProcessor, RawTextureInstance, TextureID, TextureRenderer},
    wgpu, Size,
};
use brackens_tools::glam::Mat4;
use shipyard::Unique;

use crate::assets::AssetStorage;

//===============================================================

#[derive(Unique)]
pub struct Device(wgpu::Device);
impl Device {
    pub fn new(device: wgpu::Device) -> Self {
        Self(device)
    }
    pub fn inner(&self) -> &wgpu::Device {
        &self.0
    }
}

#[derive(Unique)]
pub struct Queue(wgpu::Queue);
impl Queue {
    pub fn new(queue: wgpu::Queue) -> Self {
        Self(queue)
    }
    pub fn inner(&self) -> &wgpu::Queue {
        &self.0
    }
}

#[derive(Unique)]
pub struct Surface(wgpu::Surface);
impl Surface {
    pub fn new(surface: wgpu::Surface) -> Self {
        Self(surface)
    }
    pub fn inner(&self) -> &wgpu::Surface {
        &self.0
    }
}

#[derive(Unique)]
pub struct SurfaceConfig(wgpu::SurfaceConfiguration);
impl SurfaceConfig {
    pub fn new(surface_config: wgpu::SurfaceConfiguration) -> Self {
        Self(surface_config)
    }
    pub fn inner(&self) -> &wgpu::SurfaceConfiguration {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut wgpu::SurfaceConfiguration {
        &mut self.0
    }
    pub fn set_size(&mut self, size: Size<u32>) {
        self.0.width = size.width;
        self.0.height = size.height;
    }
}

//===============================================================

#[derive(Unique)]
pub struct RenderPassTools(RenderPassToolsInner);
impl RenderPassTools {
    pub fn new(inner: RenderPassToolsInner) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &RenderPassToolsInner {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut RenderPassToolsInner {
        &mut self.0
    }

    pub fn destroy(self) -> RenderPassToolsInner {
        self.0
    }
}

//===============================================================

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);
impl ClearColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self([r, g, b])
    }
    pub fn inner(&self) -> [f64; 3] {
        self.0
    }
    pub fn set_r(&mut self, r: f64) {
        self.0[0] = r;
    }
    pub fn set_g(&mut self, g: f64) {
        self.0[1] = g;
    }
    pub fn set_b(&mut self, b: f64) {
        self.0[2] = b;
    }
}

//===============================================================

#[derive(Unique)]
pub struct Renderer2D {
    renderer: TextureRenderer,
    processor: TextureProcessor<TextureID>,
}

impl Renderer2D {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        window_size: Size<u32>,
    ) -> Self {
        Self {
            renderer: TextureRenderer::new(device, config.format, window_size),
            processor: TextureProcessor::default(),
        }
    }

    #[inline]
    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.renderer.get_texture_layout()
    }

    #[inline]
    pub(crate) fn resize_depth(&mut self, device: &wgpu::Device, new_size: Size<u32>) {
        self.renderer.resize_depth(device, new_size);
    }

    #[inline]
    pub(crate) fn resize_projection(&mut self, queue: &wgpu::Queue, matrix: &Mat4) {
        self.renderer.set_projection(queue, matrix);
    }

    #[inline]
    pub(crate) fn resize_depth_and_projection(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: Size<u32>,
    ) {
        self.renderer.resize_depth_projection(device, queue, size);
    }

    #[inline]
    pub(crate) fn get_unprocessed_mut(
        &mut self,
    ) -> &mut HashMap<TextureID, Vec<RawTextureInstance>> {
        self.processor.get_unprocessed_mut()
    }

    #[inline]
    pub(crate) fn process_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.processor.process_texture(device, queue);
    }

    pub(crate) fn render(
        &mut self,
        texture_storage: &AssetStorage,
        render_tools: &mut render_tools::RenderPassTools,
    ) {
        let draw = self
            .processor
            .get_draw_data()
            .iter()
            .map(|(id, buffer)| {
                let bind_group = &texture_storage.get_data(*id).unwrap().bind_group;
                (bind_group, buffer)
            })
            .collect::<Vec<_>>();

        self.renderer.render(render_tools, &draw);
    }
}

//===============================================================
