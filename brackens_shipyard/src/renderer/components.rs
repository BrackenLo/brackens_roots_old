//===============================================================

use brackens_assets::Handle;
use brackens_renderer::renderer_2d::RendererTexture;
use brackens_tools::glam::Vec2;
use shipyard::Component;

pub use brackens_renderer::tools::{
    CameraOrthographic as CameraOrthographicInner, CameraPerspective as CameraPerspectiveInner,
};

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct CameraActive;

//--------------------------------------------------

#[derive(Component)]
#[track(All)]
pub struct CameraOrthographic(pub CameraOrthographicInner);
impl CameraOrthographic {
    #[inline]
    pub fn new_sized(width: f32, height: f32, z_near: f32, z_far: f32) -> Self {
        Self(CameraOrthographicInner::new_sized(
            width, height, z_near, z_far,
        ))
    }
    #[inline]
    pub fn update(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.0.update(left, right, bottom, top);
    }
    #[inline]
    pub fn update_sized(&mut self, width: f32, height: f32) {
        self.0.update_sized(width, height);
    }

    #[inline]
    #[cfg(feature = "tools")]
    pub fn get_projection(&self) -> brackens_tools::glam::Mat4 {
        self.0.get_projection()
    }
    #[inline]
    #[cfg(feature = "tools")]
    pub fn get_projection_transform(
        &self,
        pos: brackens_tools::glam::Vec3,
        rotation: brackens_tools::glam::Quat,
    ) -> brackens_tools::glam::Mat4 {
        self.0.get_projection_transform(pos, rotation)
    }
}

//--------------------------------------------------

#[derive(Component)]
#[track(All)]
pub struct CameraPerspective(pub CameraPerspectiveInner);
impl CameraPerspective {
    #[inline]
    #[cfg(feature = "tools")]
    pub fn get_projection(&self) -> brackens_tools::glam::Mat4 {
        self.0.get_projection()
    }
    #[inline]
    #[cfg(feature = "tools")]
    pub fn get_projection_transform(
        &self,
        position: brackens_tools::glam::Vec3,
        rotation: brackens_tools::glam::Quat,
    ) -> brackens_tools::glam::Mat4 {
        self.0.get_projection_transform(position, rotation)
    }
    #[inline]
    #[cfg(feature = "tools")]
    pub fn get_projection_target(
        &self,
        position: brackens_tools::glam::Vec3,
        target: brackens_tools::glam::Vec3,
    ) -> brackens_tools::glam::Mat4 {
        self.0.get_projection_target(position, target)
    }
}

//===============================================================

#[derive(Component, Clone)]
pub struct Texture2D {
    pub size: Vec2,
    pub handle: Handle<RendererTexture>,
    pub color: [f32; 4],
}
impl Texture2D {
    pub fn new(handle: Handle<RendererTexture>, width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            handle,
            color: [1., 1., 1., 1.],
        }
    }

    pub fn new_color(
        handle: Handle<RendererTexture>,
        width: f32,
        height: f32,
        color: [f32; 4],
    ) -> Self {
        Self {
            size: Vec2::new(width, height),
            handle,
            color,
        }
    }
}

//===============================================================
