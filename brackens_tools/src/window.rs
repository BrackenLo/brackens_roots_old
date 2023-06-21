//===============================================================

use winit::{
    dpi::{PhysicalSize, Position, Size},
    monitor::MonitorHandle,
    window::{Fullscreen, Window},
};

//==============================================================

/// Leaving None for Fullscreen modes will select current monitor.
/// Leaving a value will try to use the monitor with that specific index.
pub enum FullscreenMode {
    Windowed,
    Fullscreen(Option<usize>),
    FullscreenBorderless(Option<usize>),
}

//==============================================================

pub struct WindowManager(Window);
impl WindowManager {
    //----------------------------------------------

    pub fn new(window: Window) -> Self {
        Self(window)
    }
    pub fn request_redraw(&self) {
        self.0.request_redraw()
    }

    //----------------------------------------------

    pub fn size(&self) -> PhysicalSize<u32> {
        self.0.inner_size()
    }
    pub fn size_f32(&self) -> PhysicalSize<f32> {
        PhysicalSize::new(
            self.0.inner_size().width as f32,
            self.0.inner_size().height as f32,
        )
    }
    pub fn width(&self) -> u32 {
        self.0.inner_size().width
    }
    pub fn height(&self) -> u32 {
        self.0.inner_size().height
    }

    //----------------------------------------------

    pub fn raw(&self) -> &Window {
        &self.0
    }
    pub fn raw_mut(&mut self) -> &mut Window {
        &mut self.0
    }

    //----------------------------------------------

    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    pub fn set_window_size<S: Into<Size>>(&self, size: S) {
        self.0.set_inner_size(size);
    }

    pub fn move_window<P: Into<Position>>(&self, position: P) {
        self.0.set_outer_position(position);
    }

    //----------------------------------------------

    pub fn set_maximized(&self, maximized: bool) {
        self.0.set_maximized(maximized);
    }

    pub fn set_minimized(&self, minimized: bool) {
        self.0.set_minimized(minimized);
    }

    pub fn set_fullscreen_mode(&self, mode: FullscreenMode) {
        let fullscreen_mode = match mode {
            FullscreenMode::Windowed => None,
            FullscreenMode::Fullscreen(window) => {
                let monitor = self.get_monitor(window).unwrap_or_else(|| {
                    panic!("Error setting fullscreen mode: Monitor cannot be found.")
                });

                let video_mode = monitor.video_modes().next().unwrap_or_else(|| {
                    panic!("Error setting fullscreen mode: Monitor doesn't have any video modes.")
                });

                Some(Fullscreen::Exclusive(video_mode))
            }
            FullscreenMode::FullscreenBorderless(window) => {
                let monitor = self.get_monitor(window);
                Some(Fullscreen::Borderless(monitor))
            }
        };

        self.0.set_fullscreen(fullscreen_mode);
    }

    fn get_monitor(&self, monitor: Option<usize>) -> Option<MonitorHandle> {
        match monitor {
            Some(index) => self.0.available_monitors().skip(index).next(),
            None => self.0.current_monitor(),
        }
    }

    //----------------------------------------------
}

//===============================================================
