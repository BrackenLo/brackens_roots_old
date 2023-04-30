//===============================================================

use brackens_tools::{
    general,
    glam::{Quat, Vec3},
};
use shipyard::Component;

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct Transform(pub(crate) general::Transform);
impl Transform {
    pub fn translation(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    pub fn scale(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }
}

#[derive(Component)]
pub struct GlobalTransform(pub(crate) general::Transform);
impl GlobalTransform {
    pub fn translation(&mut self) -> &mut Vec3 {
        &mut self.0.translation
    }
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.0.rotation
    }
    pub fn scale(&mut self) -> &mut Vec3 {
        &mut self.0.scale
    }
}

//===============================================================
