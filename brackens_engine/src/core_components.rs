//===============================================================

use brackens_renderer::{wgpu, Size};

use brackens_tools::{
    input, upkeep,
    winit::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{MouseButton, VirtualKeyCode},
    },
};
use shipyard::Unique;

//===============================================================

#[derive(Unique)]
pub struct Device(pub wgpu::Device);

#[derive(Unique)]
pub struct Queue(pub wgpu::Queue);

#[derive(Unique)]
pub struct Surface(pub(crate) wgpu::Surface);

#[derive(Unique)]
pub struct SurfaceConfig(pub(crate) wgpu::SurfaceConfiguration);

#[derive(Unique)]
pub struct WindowSize(pub(crate) Size<u32>);

impl From<PhysicalSize<u32>> for WindowSize {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self(Size {
            width: value.width,
            height: value.height,
        })
    }
}

#[derive(Unique)]
pub struct Window(pub(crate) brackens_tools::winit::window::Window);

//===============================================================

#[derive(Unique, Default)]
pub struct KeyManager(pub(crate) input::KeyManager);
impl KeyManager {
    pub fn pressed(&self, key: VirtualKeyCode) -> bool {
        self.0.pressed(key)
    }
    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.0.just_pressed(key)
    }
    pub fn just_released(&self, key: VirtualKeyCode) -> bool {
        self.0.just_released(key)
    }
}

#[derive(Unique, Default)]
pub struct MouseKeyManager(pub(crate) input::MouseKeyManager);
impl MouseKeyManager {
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.0.pressed(button)
    }
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.0.just_pressed(button)
    }
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.0.just_released(button)
    }
}

#[derive(Unique, Default)]
pub struct MousePositionManager(pub(crate) input::MousePositionManager);
impl MousePositionManager {
    pub fn position(&self) -> PhysicalPosition<f64> {
        self.0.position()
    }
    pub fn movement(&self) -> (f64, f64) {
        self.0.movement()
    }
    pub fn moved(&self) -> bool {
        self.0.moved()
    }
}

#[derive(Unique, Default)]
pub struct UpkeepTracker(pub(crate) upkeep::UpkeepTracker);
impl UpkeepTracker {
    pub fn fps(&self) -> u16 {
        self.0.fps()
    }
    pub fn avg_fps(&self) -> f32 {
        self.0.avg_fps()
    }
    pub fn delta(&self) -> f32 {
        self.0.delta()
    }
    pub fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }
}

//===============================================================
