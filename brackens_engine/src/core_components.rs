//===============================================================

use brackens_renderer::{wgpu, Size};

use brackens_tools::{
    input::{self, KeyCode},
    upkeep,
    winit::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::MouseButton,
    },
};
use shipyard::Unique;

//===============================================================

#[cfg(feature = "debug")]
#[derive(Unique)]
pub struct TimingsDebug(Vec<(String, f32)>, std::time::Instant);
#[cfg(feature = "debug")]
impl Default for TimingsDebug {
    fn default() -> Self {
        Self(vec![], std::time::Instant::now())
    }
}

#[cfg(feature = "debug")]
impl TimingsDebug {
    pub fn add_log(&mut self, label: String, time: f32) {
        self.0.push((label, time));
    }
    pub fn print_log(&self) {
        println!("========================================");
        self.0
            .iter()
            .for_each(|(label, time)| println!("{} - {}", label, time));
        println!("Total Frame Time = {}", self.1.elapsed().as_secs_f32());
    }
    pub fn clear(&mut self) {
        self.1 = std::time::Instant::now();
        self.0.clear();
    }
}

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
impl WindowSize {
    pub fn width(&self) -> u32 {
        self.0.width
    }
    pub fn height(&self) -> u32 {
        self.0.height
    }
    pub fn size(&self) -> Size<u32> {
        self.0
    }
}

#[derive(Unique)]
pub struct Window(pub(crate) brackens_tools::winit::window::Window);

//===============================================================

#[derive(Unique, Default)]
pub struct KeyManager(pub(crate) input::KeyManager);
impl KeyManager {
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.0.pressed(key)
    }
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.0.just_pressed(key)
    }
    pub fn just_released(&self, key: KeyCode) -> bool {
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
