//===============================================================

use brackens_tools::{
    general::{Timer as TimerInner, Transform as TransformInner},
    glam::{Mat4, Quat, Vec3},
    input::{
        KeyCode, KeyManager as KeyManagerInner, MouseButton,
        MouseKeyManager as MouseKeyManagerInner, MousePositionManager as MousePositionManagerInner,
    },
    upkeep::UpkeepTracker as UpkeepTrackerInner,
    winit::event::ElementState,
};
use shipyard::{Component, Unique};

//===============================================================

#[derive(Component)]
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
        Transform(self.0 + rhs.0)
    }
}
impl std::ops::Add<&Self> for Transform {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Transform(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Transform {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
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

#[derive(Unique, Default)]
pub struct KeyManager(pub(crate) KeyManagerInner);
impl KeyManager {
    #[inline]
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.0.pressed(key)
    }
    #[inline]
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.0.just_pressed(key)
    }
    #[inline]
    pub fn just_released(&self, key: KeyCode) -> bool {
        self.0.just_released(key)
    }

    #[inline]
    pub(crate) fn manage_input(&mut self, state: ElementState, keycode: Option<KeyCode>) {
        self.0.manage_input(state, keycode);
    }
    #[inline]
    pub(crate) fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Unique, Default)]
pub struct MouseKeyManager(pub(crate) MouseKeyManagerInner);
impl MouseKeyManager {
    #[inline]
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.0.pressed(button)
    }
    #[inline]
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.0.just_pressed(button)
    }
    #[inline]
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.0.just_released(button)
    }

    #[inline]
    pub(crate) fn manage_input(&mut self, state: ElementState, button: MouseButton) {
        self.0.manage_input(state, Some(button));
    }
    #[inline]
    pub(crate) fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Unique, Default)]
pub struct MousePositionManager(pub(crate) MousePositionManagerInner);
impl MousePositionManager {
    #[inline]
    pub fn position(&self) -> (f64, f64) {
        self.0.position()
    }
    #[inline]
    pub fn movement(&self) -> (f64, f64) {
        self.0.movement()
    }
    #[inline]
    pub fn moved(&self) -> bool {
        self.0.moved()
    }

    #[inline]
    pub(crate) fn add_movement(&mut self, movement: (f64, f64)) {
        self.0.add_movement(movement);
    }
    #[inline]
    pub(crate) fn reset(&mut self) {
        self.0.reset();
    }
}

//===============================================================

#[derive(Unique, Default)]
pub struct UpkeepTracker(UpkeepTrackerInner);
impl UpkeepTracker {
    #[inline]
    pub fn fps(&self) -> u16 {
        self.0.fps()
    }

    #[inline]
    pub fn avg_fps(&self) -> f32 {
        self.0.avg_fps()
    }

    #[inline]
    pub fn delta(&self) -> f32 {
        self.0.delta()
    }

    #[inline]
    pub fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }

    #[inline]
    pub(crate) fn tick(&mut self) {
        self.0.tick()
    }
}

//===============================================================
