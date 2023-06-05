//===============================================================

// use std::{any::TypeId, collections::HashMap};

use brackens_tools::general;
use shipyard::Component;

//===============================================================

#[derive(Component)]
#[track(Modification)]
pub struct Activated;

#[derive(Component)]
pub struct AutoUpdate;

//===============================================================

#[derive(Component)]
pub struct Timer(pub(crate) general::Timer);
impl Timer {
    //--------------------------------------------------

    pub fn new(duration: f32, repeating: bool) -> Self {
        Self(general::Timer::new(duration, repeating))
    }
    pub fn restart(&mut self) {
        self.0.restart()
    }
    pub fn progress(&self) -> f32 {
        self.0.progress()
    }

    //--------------------------------------------------

    pub fn duration(&self) -> f32 {
        self.0.duration
    }

    pub fn repeating(&self) -> bool {
        self.0.repeating
    }

    pub fn paused(&self) -> bool {
        self.0.paused
    }

    pub fn finished(&self) -> bool {
        self.0.is_finished()
    }

    //--------------------------------------------------

    pub fn set_duration(&mut self, val: f32) {
        self.0.duration = val;
    }

    pub fn set_repeating(&mut self, val: bool) {
        self.0.repeating = val;
    }

    pub fn set_paused(&mut self, val: bool) {
        self.0.paused = val;
    }

    //--------------------------------------------------
}

//===============================================================
