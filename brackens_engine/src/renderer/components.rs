//===============================================================

use brackens_renderer::render_tools;

use brackens_tools::glam::Vec3;
use shipyard::{Component, Unique};

pub use brackens_renderer::{
    renderer_2d::RendererTexture,
    renderer_2d::TextureDrawBuffer as FinalTextureDrawCall,
    tools::{CameraOrthographic, CameraPerspective},
};

//===============================================================
// Core rendering Uniques

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================
// Shared Rendering Components

#[derive(Component)]
pub struct Visible {
    pub visible: bool,
}
impl Default for Visible {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Component)]
#[track(Modification)]
pub struct CameraActive;

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct Camera {
    pub projection: CameraProjection,
}
impl Camera {
    pub fn orthographic(camera_orthographic: CameraOrthographic) -> Self {
        Self {
            projection: CameraProjection::Orthographic(camera_orthographic),
        }
    }

    pub fn perspective(camera_perspective: CameraPerspective) -> Self {
        Self {
            projection: CameraProjection::Perspective(camera_perspective),
        }
    }

    pub fn perspective_target(camera_perspective: CameraPerspective, target: Vec3) -> Self {
        Self {
            projection: CameraProjection::PerspectiveTarget(camera_perspective, target),
        }
    }
}

pub enum CameraProjection {
    Orthographic(CameraOrthographic),
    Perspective(CameraPerspective),
    PerspectiveTarget(CameraPerspective, Vec3),
}

//===============================================================
