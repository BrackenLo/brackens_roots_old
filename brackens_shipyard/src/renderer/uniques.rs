//===============================================================

use brackens_renderer::{render_tools::RenderPassTools as RenderPassToolsInner, wgpu, Size};
use shipyard::Unique;

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
