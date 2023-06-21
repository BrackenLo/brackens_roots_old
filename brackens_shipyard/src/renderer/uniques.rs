//===============================================================

use brackens_renderer::{wgpu, Size};
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
