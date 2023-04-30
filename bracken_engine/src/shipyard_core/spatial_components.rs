//===============================================================

use brackens_tools::{
    general,
    glam::{Quat, Vec3},
};
use shipyard::Component;

//===============================================================

#[derive(Component, Default)]
#[track(All)]
pub struct Transform(pub(crate) general::Transform);
impl Transform {
    //--------------------------------------------------

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(general::Transform {
            translation,
            rotation,
            scale,
        })
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self(general::Transform::from_translation(translation))
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self(general::Transform::from_rotation(rotation))
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self(general::Transform::from_scale(scale))
    }

    //--------------------------------------------------

    pub fn translation(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    pub fn scale(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }

    //--------------------------------------------------

    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    //--------------------------------------------------
}
impl std::ops::Add for Transform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Self> for Transform {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Transform {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Component, Default)]
pub struct GlobalTransform(pub(crate) general::Transform);
impl GlobalTransform {
    //--------------------------------------------------

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(general::Transform {
            translation,
            rotation,
            scale,
        })
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self(general::Transform::from_translation(translation))
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self(general::Transform::from_rotation(rotation))
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self(general::Transform::from_scale(scale))
    }

    //--------------------------------------------------

    pub fn translation(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    pub fn scale(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }

    //--------------------------------------------------

    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    //--------------------------------------------------
}

impl std::ops::Add for GlobalTransform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        GlobalTransform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Self> for GlobalTransform {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        GlobalTransform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for GlobalTransform {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

//===============================================================
