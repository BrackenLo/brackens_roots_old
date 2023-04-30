//===============================================================

use instant::Instant;

//===============================================================

pub struct UpkeepTracker {
    elapsed_time: Instant,
    last_frame_instant: Instant,

    total_frame_count: u64,
    second_tracker: f32,
    frame_count_this_second: u16,

    delta: f32,
    fps: u16,
}

impl Default for UpkeepTracker {
    fn default() -> Self {
        Self {
            elapsed_time: Instant::now(),
            last_frame_instant: Instant::now(),
            total_frame_count: 0,
            second_tracker: 0.,
            frame_count_this_second: 0,
            delta: 0.,
            fps: 0,
        }
    }
}
impl UpkeepTracker {
    //----------------------------------------------

    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&mut self) {
        self.delta = self.last_frame_instant.elapsed().as_secs_f32();

        self.last_frame_instant = Instant::now();

        self.total_frame_count += 1;
        self.frame_count_this_second += 1;

        self.second_tracker += self.delta;

        if self.second_tracker > 1. {
            self.fps = self.frame_count_this_second;
            self.frame_count_this_second = 0;
            self.second_tracker -= 1.;
        }
    }

    //----------------------------------------------

    pub fn fps(&self) -> u16 {
        self.fps
    }
    pub fn avg_fps(&self) -> f32 {
        self.total_frame_count as f32 / self.elapsed_time.elapsed().as_secs_f32()
    }
    pub fn delta(&self) -> f32 {
        self.delta
    }
    pub fn elapsed(&self) -> instant::Duration {
        self.elapsed_time.elapsed()
    }
}

//===============================================================
