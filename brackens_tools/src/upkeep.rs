//===============================================================

use instant::Instant;

//===============================================================

pub const MAX_FPS_RECORD_SIZE: usize = 6;

pub struct UpkeepTracker {
    elapsed_time: Instant,
    last_frame_instant: Instant,

    total_frame_count: u64,
    second_tracker: f32,
    frame_count_this_second: u16,

    delta: f32,

    fps_list: [u16; MAX_FPS_RECORD_SIZE],
    fps_instance_counter: usize,
    fps_sum: f64,
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

            fps_list: [0; MAX_FPS_RECORD_SIZE],
            fps_instance_counter: 0,
            fps_sum: 0.,
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
            self.fps_sum -= self.fps_list[self.fps_instance_counter] as f64;
            self.fps_sum += self.frame_count_this_second as f64;
            self.fps_list[self.fps_instance_counter] = self.frame_count_this_second;
            self.fps_instance_counter = (self.fps_instance_counter + 1) % MAX_FPS_RECORD_SIZE;

            self.frame_count_this_second = 0;
            self.second_tracker -= 1.;
        }
    }

    //----------------------------------------------

    pub fn fps(&self) -> u16 {
        self.fps_list[self.fps_instance_counter]
    }
    pub fn avg_fps(&self) -> f32 {
        (self.fps_sum / MAX_FPS_RECORD_SIZE as f64) as f32
    }
    pub fn delta(&self) -> f32 {
        self.delta
    }
    pub fn elapsed(&self) -> instant::Duration {
        self.elapsed_time.elapsed()
    }
}

//===============================================================
