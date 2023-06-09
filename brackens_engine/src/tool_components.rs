//===============================================================

// use std::{any::TypeId, collections::HashMap};

use brackens_tools::general;
use shipyard::Component;

//===============================================================

#[cfg(feature = "debug")]
use {crate::prelude::shipyard::Unique, colored::Colorize};

#[cfg(feature = "debug")]
#[derive(Unique)]
pub struct TimingsDebug {
    pub(crate) timings: Vec<(String, f32, colored::Color)>,
    pub(crate) frame_time: std::time::Instant,
    pub(crate) timer: std::time::Instant,
}

#[cfg(feature = "debug")]
impl Default for TimingsDebug {
    fn default() -> Self {
        Self {
            timings: vec![],
            frame_time: std::time::Instant::now(),
            timer: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "debug")]
impl TimingsDebug {
    pub fn add_time(&mut self, label: String, time: f32, color: Option<colored::Color>) {
        let color = match color {
            Some(val) => val,
            None => colored::Color::Black,
        };
        self.timings.push((label, time, color));
    }

    pub fn record_time(&mut self, label: String, color: Option<colored::Color>) {
        let elapsed = self.timer.elapsed().as_secs_f32();
        self.add_time(label, elapsed, color);
    }

    pub fn record_time_and_reset(&mut self, label: String, color: Option<colored::Color>) {
        self.record_time(label, color);
        self.timer = std::time::Instant::now();
    }

    pub fn print_log(&self) {
        println!("========================================");
        self.timings
            .iter()
            .for_each(|(label, time, color)| println!("{} - {}", label.color(*color), time));
        println!(
            "Total Frame Time = {}",
            self.frame_time.elapsed().as_secs_f32()
        );
    }

    pub fn reset_timer(&mut self) {
        self.timer = std::time::Instant::now();
    }

    pub fn clear(&mut self) {
        self.frame_time = std::time::Instant::now();
        self.timings.clear();
    }
}

//===============================================================

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
