//===============================================================

use std::vec::Drain;

use shipyard::Unique;

//===============================================================

#[derive(Unique)]
#[track(Insertion)]
#[cfg(feature = "renderer")]
pub struct ResizeEvent(brackens_renderer::Size<u32>);
impl ResizeEvent {
    #[cfg(feature = "renderer")]
    pub fn new(size: brackens_renderer::Size<u32>) -> Self {
        Self(size)
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height
    }
}

//===============================================================

#[derive(Unique, Default)]
pub struct RunnerErrorManager(Vec<RunnerError>);
impl RunnerErrorManager {
    pub fn add_error(&mut self, error: RunnerError) {
        self.0.push(error);
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<RunnerError> {
        self.0.drain(..)
    }
}

pub enum RunnerError {
    ForceResize,
}

//===============================================================
