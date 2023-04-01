//===============================================================

use winit::{dpi::PhysicalSize, window::Window};

//==============================================================

pub struct WindowManager {
    window: Window,
}
impl WindowManager {
    //----------------------------------------------

    pub fn new(window: Window) -> Self {
        Self { window }
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }

    //----------------------------------------------

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }
    pub fn size_f32(&self) -> (f32, f32) {
        (
            self.window.inner_size().width as f32,
            self.window.inner_size().height as f32,
        )
    }
    pub fn width(&self) -> u32 {
        self.window.inner_size().width
    }
    pub fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    //----------------------------------------------

    pub fn raw(&self) -> &Window {
        &self.window
    }
    pub fn raw_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    //----------------------------------------------
}

//===============================================================
