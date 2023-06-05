//===============================================================

use brackens_tools::glam::Mat4;
use shipyard::{Borrow, BorrowInfo, IntoBorrow, IntoIter, IntoWithId, View};

use crate::{prelude::GlobalTransform, tool_components::Active};

use super::components::{Camera, CameraProjection};

//===============================================================

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
                    z_near,
                    z_far,
                } => {
                    let projection_matrix =
                        Mat4::orthographic_rh(left, right, bottom, top, z_near, z_far);

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
                    z_near,
                    z_far,
                } => {
                    let transform_position = *global_transform.translation();

                    let view = Mat4::look_at_rh(transform_position, target, up);
                    let proj = Mat4::perspective_rh_gl(fovy, aspect, z_near, z_far);

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
