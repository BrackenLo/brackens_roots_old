//===============================================================

use brackens_tools::{input, upkeep::UpkeepTracker, wgpu, winit::dpi::PhysicalSize};
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
pub struct WindowSize(pub(crate) PhysicalSize<u32>);

#[derive(Unique)]
pub struct Window(pub(crate) brackens_tools::winit::window::Window);

//===============================================================

#[derive(Unique, Default)]
pub struct KeyManagerUnique(input::KeyManager);

#[derive(Unique, Default)]
pub struct MouseKeyManagerUnique(input::MouseKeyManager);

#[derive(Unique, Default)]
pub struct MousePositionManagerUnique(input::MousePositionManager);

#[derive(Unique, Default)]
pub struct UpkeepTrackerUnique(UpkeepTracker);

//===============================================================
