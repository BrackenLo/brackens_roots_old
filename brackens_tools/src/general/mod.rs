//===============================================================

//===============================================================

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}
impl Transform {
    pub fn to_raw(&self) -> [f32; 16] {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
            .to_cols_array()
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: glam::Vec3::ZERO,
            rotation: glam::Quat::default(),
            scale: glam::Vec3::ONE,
        }
    }
}
impl std::ops::Add for Transform {
    type Output = Self;

    fn add(self, rhs: Transform) -> Self::Output {
        let mut output = self;
        output.translation += rhs.translation;
        output.rotation = output.rotation.mul_quat(rhs.rotation);
        output.scale *= rhs.scale;
        output
    }
}
impl std::ops::AddAssign for Transform {
    fn add_assign(&mut self, rhs: Self) {
        self.translation += rhs.translation;
        self.rotation = self.rotation.mul_quat(rhs.rotation);
        self.scale *= rhs.scale;
    }
}

//===============================================================

#[derive(Default)]
pub struct Timer {
    pub(crate) elapsed: f32, // How much time has passed
    pub duration: f32,       // How long the timer is for
    pub paused: bool,
    pub repeating: bool,
    pub(crate) finished: bool,
}
impl Timer {
    pub fn new(duration: f32, repeating: bool) -> Self {
        Self {
            duration,
            repeating,
            ..Default::default()
        }
    }
    pub fn tick(&mut self, delta: f32) {
        match (self.paused, self.repeating, self.finished) {
            //Timer is repeating and finished - restart it and keep going
            (false, true, true) => {
                self.elapsed += delta;
                self.finished = false;
            }
            //Timer is repeating but not finished - keep going
            (false, true, false) => {
                self.elapsed += delta;
                if self.elapsed > self.duration {
                    self.finished = true;
                    self.elapsed -= self.duration;
                }
            }
            //Timer is not repeating and is finished - Don't do anything
            (false, false, true) => {}
            //Timer is not repeating and not finished - keep going
            (false, false, false) => {
                self.elapsed += delta;
                if self.elapsed > self.duration {
                    self.finished = true;
                    self.elapsed = self.duration;
                }
            }
            // Ignore all paused timers
            _ => {}
        }
    }
    pub fn restart(&mut self) {
        self.elapsed = 0.;
    }
    pub fn is_finished(&self) -> bool {
        return self.finished;
    }
    pub fn progress(&self) -> f32 {
        return self.elapsed / self.duration;
    }
}

//===============================================================

//===============================================================
