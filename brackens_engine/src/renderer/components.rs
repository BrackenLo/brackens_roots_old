//===============================================================

use brackens_renderer::render_tools;

use brackens_tools::glam::Vec3;
use shipyard::{Component, Unique};

pub use brackens_renderer::{
    renderer_2d::RendererTexture, renderer_2d::TextureDrawCall as FinalTextureDrawCall,
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

#[derive(Component)]
#[track(Modification)]
pub struct Camera {
    pub projection: CameraProjection,
}

pub enum CameraProjection {
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        z_near: f32,
        z_far: f32,
    },
    Perspective {
        target: Vec3,
        up: Vec3,
        aspect: f32,
        fovy: f32,
        z_near: f32,
        z_far: f32,
    },
}

pub struct OrthographicCameraDescriptor {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub z_near: f32,
    pub z_far: f32,
}
impl Default for OrthographicCameraDescriptor {
    fn default() -> Self {
        Self {
            left: 0.,
            right: 1920.,
            bottom: 0.,
            top: 1080.,
            z_near: 0.,
            z_far: 1000.,
        }
    }
}

pub struct PerspectiveCameraDescriptor {
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,
}
impl Default for PerspectiveCameraDescriptor {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 1.77777777778,
            fovy: 45.,
            z_near: 0.1,
            z_far: 1000.,
        }
    }
}

impl Camera {
    pub fn new_orthographic(
        OrthographicCameraDescriptor {
            left,
            right,
            bottom,
            top,
            z_near,
            z_far,
        }: OrthographicCameraDescriptor,
    ) -> Self {
        Self {
            projection: CameraProjection::Orthographic {
                left,
                right,
                bottom,
                top,
                z_near,
                z_far,
            },
        }
    }

    pub fn new_perspective(
        PerspectiveCameraDescriptor {
            target,
            up,
            aspect,
            fovy,
            z_near,
            z_far,
        }: PerspectiveCameraDescriptor,
    ) -> Self {
        Self {
            projection: CameraProjection::Perspective {
                target,
                up,
                aspect,
                fovy,
                z_near,
                z_far,
            },
        }
    }
}

//===============================================================

//===============================================================
