//===============================================================

use glam::{Mat4, Quat, Vec3};

//===============================================================

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    //--------------------------------------------------

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    pub fn from_translation_rotatation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
            ..Default::default()
        }
    }

    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Self {
            translation,
            scale,
            ..Default::default()
        }
    }

    pub fn from_rotation_scale(rotation: Quat, scale: Vec3) -> Self {
        Self {
            rotation,
            scale,
            ..Default::default()
        }
    }

    pub fn from_translation_rotatation_scale(
        translation: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    //--------------------------------------------------

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    //--------------------------------------------------

    pub fn lerp(&mut self, target: &Transform, s: f32) {
        self.translation = self.translation.lerp(target.translation, s);
        self.rotation = self.rotation.lerp(target.rotation, s);
        self.scale = self.scale.lerp(target.scale, s);
    }

    //--------------------------------------------------

    #[inline]
    pub fn to_raw(&self) -> [f32; 16] {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
            .to_cols_array()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    //--------------------------------------------------
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::default(),
            scale: Vec3::ONE,
        }
    }
}

//--------------------------------------------------

impl std::ops::Add for Transform {
    type Output = Self;

    fn add(mut self, rhs: Transform) -> Self::Output {
        self.translation += rhs.translation;
        self.rotation = self.rotation.mul_quat(rhs.rotation);
        self.scale *= rhs.scale;
        self
    }
}

impl std::ops::AddAssign for Transform {
    fn add_assign(&mut self, rhs: Self) {
        self.translation += rhs.translation;
        self.rotation = self.rotation.mul_quat(rhs.rotation);
        self.scale *= rhs.scale;
    }
}

impl std::ops::Sub for Transform {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.translation -= rhs.translation;
        self.rotation = self.rotation.mul_quat(rhs.rotation.inverse());
        self.scale /= rhs.scale;

        self
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

pub fn lerp(start: f32, end: f32, s: f32) -> f32 {
    start + (end - start) * s
}

//===============================================================
