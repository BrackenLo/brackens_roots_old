//===============================================================

use brackens_tools::{
    input::{
        KeyCode, KeyManager as KeyManagerInner, MouseButton,
        MouseKeyManager as MouseKeyManagerInner, MousePositionManager as MousePositionManagerInner,
    },
    upkeep::UpkeepTracker as UpkeepTrackerInner,
    window::WindowManager,
    winit::{dpi::PhysicalPosition, event::ElementState},
};
use shipyard::Unique;

pub use brackens_tools::window::FullscreenMode;

//===============================================================

//===============================================================

#[derive(Unique, Default)]
pub struct KeyManager(KeyManagerInner);
impl KeyManager {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.0.pressed(key)
    }
    #[inline]
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.0.just_pressed(key)
    }
    #[inline]
    pub fn just_released(&self, key: KeyCode) -> bool {
        self.0.just_released(key)
    }
    #[inline]
    pub fn manage_input(&mut self, state: ElementState, keycode: Option<KeyCode>) {
        self.0.manage_input(state, keycode);
    }
    #[inline]
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Unique, Default)]
pub struct MouseKeyManager(MouseKeyManagerInner);
impl MouseKeyManager {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.0.pressed(button)
    }
    #[inline]
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.0.just_pressed(button)
    }
    #[inline]
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.0.just_released(button)
    }

    #[inline]
    pub fn manage_input(&mut self, state: ElementState, button: MouseButton) {
        self.0.manage_input(state, Some(button));
    }
    #[inline]
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Unique, Default)]
pub struct MousePositionManager(MousePositionManagerInner);
impl MousePositionManager {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn position(&self) -> (f64, f64) {
        self.0.position()
    }
    #[inline]
    pub fn movement(&self) -> (f64, f64) {
        self.0.movement()
    }
    #[inline]
    pub fn moved(&self) -> bool {
        self.0.moved()
    }

    #[inline]
    pub fn add_movement(&mut self, movement: (f64, f64)) {
        self.0.add_movement(movement);
    }
    #[inline]
    pub fn set_position(&mut self, position: (f64, f64)) {
        self.0.set_position(position);
    }
    #[inline]
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

//===============================================================

#[derive(Unique, Default)]
pub struct UpkeepTracker(UpkeepTrackerInner);
impl UpkeepTracker {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn fps(&self) -> u16 {
        self.0.fps()
    }

    #[inline]
    pub fn avg_fps(&self) -> f32 {
        self.0.avg_fps()
    }

    #[inline]
    pub fn delta(&self) -> f32 {
        self.0.delta()
    }

    #[inline]
    pub fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }

    #[inline]
    pub(crate) fn tick(&mut self) {
        self.0.tick()
    }
}

//===============================================================

#[derive(Unique)]
pub struct Window(WindowManager);
impl Window {
    pub fn new(window: brackens_tools::winit::window::Window) -> Self {
        Self(WindowManager::new(window))
    }

    #[inline]
    pub fn inner(&self) -> &brackens_tools::winit::window::Window {
        self.0.inner()
    }

    #[inline]
    pub fn request_redraw(&self) {
        self.0.request_redraw();
    }

    //--------------------------------------------------

    #[cfg(feature = "renderer")]
    #[inline]
    pub fn size(&self) -> brackens_renderer::Size<u32> {
        self.0.size().into()
    }

    #[cfg(not(feature = "renderer"))]
    #[inline]
    pub fn size(&self) -> brackens_tools::winit::dpi::PhysicalSize<u32> {
        self.0.size()
    }

    //--------------------------------------------------

    #[cfg(feature = "renderer")]
    #[inline]
    pub fn size_f32(&self) -> brackens_renderer::Size<f32> {
        self.0.size_f32().into()
    }

    #[cfg(not(feature = "renderer"))]
    #[inline]
    pub fn size_f32(&self) -> brackens_tools::winit::dpi::PhysicalSize<f32> {
        self.0.size_f32()
    }

    //--------------------------------------------------

    #[cfg(feature = "renderer")]
    pub fn set_window_size(&self, size: brackens_renderer::Size<f32>) {
        let new_size: brackens_tools::winit::dpi::PhysicalSize<f32> = size.into();
        self.0.set_window_size(new_size);
    }

    #[cfg(not(feature = "renderer"))]
    #[inline]
    pub fn set_window_size(&self, size: brackens_tools::winit::dpi::PhysicalSize<f32>) {
        self.0.set_window_size(size);
    }

    //--------------------------------------------------

    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width()
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    //--------------------------------------------------

    #[inline]
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    #[inline]
    pub fn move_window(&self, position: (i32, i32)) {
        self.0
            .move_window(PhysicalPosition::new(position.0, position.1));
    }

    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
        self.0.set_maximized(maximized);
    }
    #[inline]
    pub fn set_minimized(&self, minimized: bool) {
        self.0.set_minimized(minimized);
    }

    #[inline]
    pub fn set_fullscreen_mode(&self, mode: FullscreenMode) {
        self.0.set_fullscreen_mode(mode);
    }
}

//===============================================================
