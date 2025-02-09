//===============================================================

use brackens_tools::{
    general::{Timer as TimerInner, Transform as TransformInner},
    glam::{Mat4, Quat, Vec3},
};
use shipyard::Component;

pub use brackens_tools::window::FullscreenMode;

//===============================================================

#[derive(Component, Default, Clone)]
#[track(All)]
pub struct Transform(TransformInner);

impl Transform {
    //--------------------------------------------------

    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner {
            translation,
            rotation,
            scale,
        })
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self(TransformInner::from_translation(translation))
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self(TransformInner::from_rotation(rotation))
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self(TransformInner::from_scale(scale))
    }

    #[inline]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self(TransformInner::from_translation_rotatation(
            translation,
            rotation,
        ))
    }
    #[inline]
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Self(TransformInner::from_translation_scale(translation, scale))
    }

    #[inline]
    pub fn from_rotation_scale(rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner::from_rotation_scale(rotation, scale))
    }

    #[inline]
    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner::from_translation_rotatation_scale(
            translation,
            rotation,
            scale,
        ))
    }

    //--------------------------------------------------

    #[inline]
    pub fn translation(&self) -> &Vec3 {
        &self.0.translation
    }
    #[inline]
    pub fn rotation(&self) -> &Quat {
        &self.0.rotation
    }
    #[inline]
    pub fn scale(&self) -> &Vec3 {
        &self.0.scale
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    #[inline]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    #[inline]
    pub fn scale_mut(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }

    //--------------------------------------------------

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    //--------------------------------------------------

    #[inline]
    pub fn lerp(&mut self, target: &Transform, s: f32) {
        self.0.lerp(&target.0, s);
    }

    //--------------------------------------------------

    #[inline]
    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        self.0.to_mat4()
    }

    //--------------------------------------------------
}
impl std::ops::Add for Transform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Transform> for &Transform {
    type Output = Transform;

    #[inline]
    fn add(self, rhs: &Transform) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Transform {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

//--------------------------------------------------

impl std::ops::AddAssign<&LocalTransform> for Transform {
    #[inline]
    fn add_assign(&mut self, rhs: &LocalTransform) {
        self.0 += rhs.0
    }
}

impl std::ops::Add<LocalTransform> for Transform {
    type Output = Transform;

    #[inline]
    fn add(self, rhs: LocalTransform) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Add<&LocalTransform> for &Transform {
    type Output = Transform;

    #[inline]
    fn add(self, rhs: &LocalTransform) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

//--------------------------------------------------

impl std::ops::Sub for Transform {
    type Output = Transform;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Sub<&Transform> for &Transform {
    type Output = Transform;

    #[inline]
    fn sub(self, rhs: &Transform) -> Self::Output {
        Transform(self.0 - rhs.0)
    }
}

//--------------------------------------------------

impl Into<LocalTransform> for Transform {
    fn into(self) -> LocalTransform {
        LocalTransform(self.0)
    }
}

//===============================================================

#[derive(Component, Default, Clone)]
#[track(All)]
pub struct LocalTransform(TransformInner);

impl LocalTransform {
    //--------------------------------------------------

    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner {
            translation,
            rotation,
            scale,
        })
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self(TransformInner::from_translation(translation))
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self(TransformInner::from_rotation(rotation))
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self(TransformInner::from_scale(scale))
    }

    #[inline]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self(TransformInner::from_translation_rotatation(
            translation,
            rotation,
        ))
    }
    #[inline]
    pub fn from_translation_scale(translation: Vec3, scale: Vec3) -> Self {
        Self(TransformInner::from_translation_scale(translation, scale))
    }

    #[inline]
    pub fn from_rotation_scale(rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner::from_rotation_scale(rotation, scale))
    }

    #[inline]
    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self(TransformInner::from_translation_rotatation_scale(
            translation,
            rotation,
            scale,
        ))
    }

    //--------------------------------------------------

    #[inline]
    pub fn translation(&self) -> &Vec3 {
        &self.0.translation
    }
    #[inline]
    pub fn rotation(&self) -> &Quat {
        &self.0.rotation
    }
    #[inline]
    pub fn scale(&self) -> &Vec3 {
        &self.0.scale
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    #[inline]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    #[inline]
    pub fn scale_mut(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }

    //--------------------------------------------------

    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.0.forward()
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.0.right()
    }

    //--------------------------------------------------

    #[inline]
    pub fn lerp(&mut self, target: &Transform, s: f32) {
        self.0.lerp(&target.0, s);
    }

    //--------------------------------------------------

    #[inline]
    pub fn to_raw(&self) -> [f32; 16] {
        self.0.to_raw()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        self.0.to_mat4()
    }

    //--------------------------------------------------
}

//--------------------------------------------------

impl std::ops::Add for LocalTransform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Add<&LocalTransform> for &LocalTransform {
    type Output = LocalTransform;

    #[inline]
    fn add(self, rhs: &LocalTransform) -> Self::Output {
        LocalTransform(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign for LocalTransform {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::ops::Add<Transform> for LocalTransform {
    type Output = Transform;

    fn add(self, rhs: Transform) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Transform> for &LocalTransform {
    type Output = Transform;

    fn add(self, rhs: &Transform) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

//--------------------------------------------------

impl Into<Transform> for LocalTransform {
    fn into(self) -> Transform {
        Transform(self.0)
    }
}

//===============================================================

#[derive(Component)]
pub struct Timer(pub(crate) TimerInner);
impl Timer {
    //--------------------------------------------------

    pub fn new(duration: f32, repeating: bool) -> Self {
        Self(TimerInner::new(duration, repeating))
    }

    #[inline]
    pub fn restart(&mut self) {
        self.0.restart()
    }

    #[inline]
    pub fn progress(&self) -> f32 {
        self.0.progress()
    }

    //--------------------------------------------------

    #[inline]
    pub fn duration(&self) -> f32 {
        self.0.duration
    }

    #[inline]
    pub fn repeating(&self) -> bool {
        self.0.repeating
    }

    #[inline]
    pub fn paused(&self) -> bool {
        self.0.paused
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.0.is_finished()
    }

    //--------------------------------------------------

    #[inline]
    pub fn set_duration(&mut self, val: f32) {
        self.0.duration = val;
    }

    #[inline]
    pub fn set_repeating(&mut self, val: bool) {
        self.0.repeating = val;
    }

    #[inline]
    pub fn set_paused(&mut self, val: bool) {
        self.0.paused = val;
    }

    //--------------------------------------------------

    #[inline]
    pub(crate) fn tick(&mut self, delta: f32) {
        self.0.tick(delta)
    }

    //--------------------------------------------------
}

//===============================================================

//===============================================================
