//===============================================================

use brackens_renderer::render_tools;

use brackens_tools::glam::{Mat4, Vec3};
use shipyard::{Borrow, BorrowInfo, Component, IntoBorrow, IntoIter, IntoWithId, Unique, View};

pub use brackens_renderer::{
    renderer_2d::RendererTexture, renderer_2d::TextureDrawCall as FinalTextureDrawCall,
};

use crate::{prelude::GlobalTransform, tool_components::Active};

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
pub struct Camera {
    pub projection: CameraProjection,
}

pub enum CameraProjection {
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
    Perspective {
        target: Vec3,
        up: Vec3,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    },
}

pub struct OrthographicCameraDescriptor {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}
impl Default for OrthographicCameraDescriptor {
    fn default() -> Self {
        Self {
            left: 0.,
            right: 1920.,
            bottom: 0.,
            top: 1080.,
            near: 0.,
            far: 100.,
        }
    }
}

pub struct PerspectiveCameraDescriptor {
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Default for PerspectiveCameraDescriptor {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 1.77777777778,
            fovy: 45.,
            znear: 0.1,
            zfar: 100.,
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
            near,
            far,
        }: OrthographicCameraDescriptor,
    ) -> Self {
        Self {
            projection: CameraProjection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            },
        }
    }

    pub fn new_perspective(
        PerspectiveCameraDescriptor {
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }: PerspectiveCameraDescriptor,
    ) -> Self {
        Self {
            projection: CameraProjection::Perspective {
                target,
                up,
                aspect,
                fovy,
                znear,
                zfar,
            },
        }
    }
}

pub struct CameraBundleView<'v> {
    v_global_transform: View<'v, GlobalTransform>,
    v_camera: View<'v, Camera>,
    v_active: View<'v, Active>,
}
impl<'v> CameraBundleView<'v> {
    pub fn has_camera(&self) -> bool {
        // Get iterator for camera, active and global transform and check if an entity exists with both
        (&self.v_camera, &self.v_active, &self.v_global_transform)
            .iter()
            .next()
            .is_some()
    }

    pub fn has_changed(&self) -> bool {
        if let Some((id, _)) = (&self.v_camera, &self.v_active, &self.v_global_transform)
            .iter()
            .with_id()
            .next()
        {
            return self.v_camera.is_modified(id)
                || self.v_active.is_modified(id)
                || self.v_global_transform.is_modified(id);
        }

        false
    }

    pub fn get_projection(&self) -> Mat4 {
        if let Some((camera, _, global_transform)) =
            (&self.v_camera, &self.v_active, &self.v_global_transform)
                .iter()
                .next()
        {
            return match camera.projection {
                CameraProjection::Orthographic {
                    left,
                    right,
                    top,
                    bottom,
                    near,
                    far,
                } => {
                    let projection_matrix =
                        Mat4::orthographic_rh(left, right, bottom, top, near, far);

                    let transform_position = *global_transform.translation();
                    let transform_rotation = *global_transform.rotation();

                    let transform_matrix =
                        Mat4::from_rotation_translation(transform_rotation, -transform_position);

                    projection_matrix * transform_matrix
                }
                CameraProjection::Perspective {
                    target,
                    up,
                    aspect,
                    fovy,
                    znear,
                    zfar,
                } => {
                    let transform_position = *global_transform.translation();

                    let view = Mat4::look_at_rh(transform_position, target, up);
                    let proj = Mat4::perspective_rh_gl(fovy, aspect, znear, zfar);

                    proj * view
                }
            };
        } else {
            panic!("Error: No cameras available to set projection with");
        }
    }
}

pub struct CameraBundleViewBorrower;
impl<'v> IntoBorrow for CameraBundleView<'_> {
    type Borrow = CameraBundleViewBorrower;
}

type CameraBundleViewComponents<'v> = (
    View<'v, GlobalTransform>,
    View<'v, Camera>,
    View<'v, Active>,
);

impl<'v> Borrow<'v> for CameraBundleViewBorrower {
    type View = CameraBundleView<'v>;

    fn borrow(
        world: &'v shipyard::World,
        last_run: Option<u32>,
        current: u32,
    ) -> Result<Self::View, shipyard::error::GetStorage> {
        let (v_global_transform, v_camera, v_active) =
            <CameraBundleViewComponents as IntoBorrow>::Borrow::borrow(world, last_run, current)?;

        Ok(CameraBundleView {
            v_global_transform,
            v_camera,
            v_active,
        })
    }
}

unsafe impl BorrowInfo for CameraBundleView<'_> {
    fn borrow_info(info: &mut Vec<shipyard::info::TypeInfo>) {
        <CameraBundleViewComponents>::borrow_info(info);
    }
}

//===============================================================

//===============================================================
