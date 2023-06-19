//===============================================================

use brackens_renderer::tools::{CameraOrthographic, CameraPerspective};
use brackens_tools::glam::{Mat4, Vec3};
use shipyard::{
    Borrow, BorrowInfo, EntitiesViewMut, EntityId, IntoBorrow, IntoIter, IntoWithId, View, ViewMut,
};

use crate::{
    prelude::{GlobalTransform, Transform},
    spatial_tools::TransformBundleViewMut,
    tool_components::AutoUpdate,
};

use super::components::{Camera, CameraActive, CameraProjection};

//===============================================================

pub trait CameraBundleViewTools {
    fn has_camera(&self) -> bool;
    fn get_camera(&self) -> Option<EntityId>;
    fn camera_changed(&self) -> bool;
    fn get_projection(&self) -> Mat4;
}

pub type CameraBundleViewComponents<'v> = (
    View<'v, GlobalTransform>,
    View<'v, Camera>,
    View<'v, CameraActive>,
);

pub type CameraBundleViewComponentsRef<'v> = (
    &'v View<'v, GlobalTransform>,
    &'v View<'v, Camera>,
    &'v View<'v, CameraActive>,
);

//===============================================================

impl<'v> CameraBundleViewTools for CameraBundleViewComponents<'v> {
    #[inline]
    fn has_camera(&self) -> bool {
        (&self.0, &self.1, &self.2).has_camera()
    }

    #[inline]
    fn get_camera(&self) -> Option<EntityId> {
        (&self.0, &self.1, &self.2).get_camera()
    }

    #[inline]
    fn camera_changed(&self) -> bool {
        (&self.0, &self.1, &self.2).camera_changed()
    }

    #[inline]
    fn get_projection(&self) -> Mat4 {
        (&self.0, &self.1, &self.2).get_projection()
    }
}

impl<'v> CameraBundleViewTools for CameraBundleViewComponentsRef<'v> {
    fn has_camera(&self) -> bool {
        let (v_camera, v_active, v_global_transform) = *self;

        // Get iterator for camera, active and global transform and check if an entity exists with both
        (v_camera, v_active, v_global_transform)
            .iter()
            .next()
            .is_some()
    }

    fn get_camera(&self) -> Option<EntityId> {
        let (v_camera, v_active, v_global_transform) = *self;
        match (v_camera, v_active, v_global_transform)
            .iter()
            .with_id()
            .next()
        {
            Some((id, _)) => Some(id),
            None => None,
        }
    }

    fn camera_changed(&self) -> bool {
        let (v_camera, v_active, v_global_transform) = *self;

        if let Some((id, _)) = (v_camera, v_active, v_global_transform)
            .iter()
            .with_id()
            .next()
        {
            return v_camera.is_modified(id)
                || v_active.is_modified(id)
                || v_global_transform.is_modified(id);
        }

        false
    }

    fn get_projection(&self) -> Mat4 {
        let (v_global_transform, v_camera, v_active) = *self;

        if let Some((camera, _, global_transform)) =
            (v_camera, v_active, v_global_transform).iter().next()
        {
            return match &camera.projection {
                // CameraProjection::Orthographic(CameraOrthographic {
                //     left,
                //     right,
                //     top,
                //     bottom,
                //     z_near,
                //     z_far,
                // }) => {
                //     let projection_matrix =
                //         Mat4::orthographic_lh(left, right, bottom, top, z_near, z_far);

                //     let transform_position = *global_transform.translation();
                //     let transform_rotation = *global_transform.rotation();

                //     let transform_matrix =
                //         Mat4::from_rotation_translation(transform_rotation, -transform_position);

                //     projection_matrix * transform_matrix
                // }
                CameraProjection::Orthographic(orthographic) => {
                    let transform_position = *global_transform.translation();
                    let transform_rotation = *global_transform.rotation();

                    orthographic.get_projection_transform(transform_position, transform_rotation)
                }

                // CameraProjection::PerspectiveTarget(
                //     CameraPerspective {
                //         up,
                //         aspect,
                //         fovy,
                //         z_near,
                //         z_far,
                //     },
                //     target,
                // ) => {
                //     let position = *global_transform.translation();

                //     let view = Mat4::look_at_lh(position, target, up);
                //     let proj = Mat4::perspective_lh(fovy, aspect, z_near, z_far);

                //     proj * view
                // }
                CameraProjection::PerspectiveTarget(perspective, target) => {
                    let position = *global_transform.translation();
                    perspective.get_projection_target(position, *target)
                }

                // CameraProjection::Perspective(CameraPerspective {
                //     up,
                //     aspect,
                //     fovy,
                //     z_near,
                //     z_far,
                // }) => {
                //     let position = *global_transform.translation();
                //     let direction = global_transform.forward().normalize();

                //     let view = Mat4::look_at_lh(position, position + direction, up);
                //     let proj = Mat4::perspective_lh(fovy, aspect, z_near, z_far);

                //     proj * view
                // }
                CameraProjection::Perspective(perspective) => {
                    let position = *global_transform.translation();
                    let rotation = *global_transform.rotation();

                    perspective.get_projection_transform(position, rotation)
                }
            };
        } else {
            panic!("Error: No cameras available to set projection with");
        }
    }
}

//===============================================================

pub struct CameraBundleView<'v> {
    v_global_transform: View<'v, GlobalTransform>,
    v_camera: View<'v, Camera>,
    v_active: View<'v, CameraActive>,
}
impl<'v> CameraBundleView<'v> {
    #[inline]
    pub fn inner(
        &self,
    ) -> (
        &View<'v, GlobalTransform>,
        &View<'v, Camera>,
        &View<'v, CameraActive>,
    ) {
        (&self.v_global_transform, &self.v_camera, &self.v_active)
    }

    #[inline]
    pub fn has_camera(&self) -> bool {
        (&self.v_global_transform, &self.v_camera, &self.v_active).has_camera()
    }

    #[inline]
    pub fn get_camera(&self) -> Option<EntityId> {
        self.inner().get_camera()
    }

    #[inline]
    pub fn camera_changed(&self) -> bool {
        self.inner().camera_changed()
    }

    pub fn get_projection(&self) -> Mat4 {
        self.inner().get_projection()
    }
}

//===============================================================

pub struct CameraBundleViewBorrower;
impl<'v> IntoBorrow for CameraBundleView<'_> {
    type Borrow = CameraBundleViewBorrower;
}

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

pub struct CameraBundleViewMut<'v> {
    vm_transform_bundle: TransformBundleViewMut<'v>,
    vm_camera: ViewMut<'v, Camera>,
    vm_active: ViewMut<'v, CameraActive>,
    vm_auto_update: ViewMut<'v, AutoUpdate>,
}

impl<'v> CameraBundleViewMut<'v> {
    pub fn create_orographic(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
        orthographic: CameraOrthographic,
        is_active: bool,
        auto_updated: bool,
    ) -> EntityId {
        self.create_base(
            entities,
            transform,
            is_active,
            auto_updated,
            Camera {
                projection: CameraProjection::Orthographic(orthographic),
            },
        )
    }

    pub fn create_perspective(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
        perspective: CameraPerspective,
        is_active: bool,
        auto_updated: bool,
    ) -> EntityId {
        self.create_base(
            entities,
            transform,
            is_active,
            auto_updated,
            Camera {
                projection: CameraProjection::Perspective(perspective),
            },
        )
    }

    pub fn create_perspective_target(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
        perspective: CameraPerspective,
        target: Vec3,
        is_active: bool,
        auto_updated: bool,
    ) -> EntityId {
        self.create_base(
            entities,
            transform,
            is_active,
            auto_updated,
            Camera {
                projection: CameraProjection::PerspectiveTarget(perspective, target),
            },
        )
    }

    fn create_base(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
        is_active: bool,
        auto_updated: bool,
        camera: Camera,
    ) -> EntityId {
        let id = self
            .vm_transform_bundle
            .create_transform(entities, transform);

        entities.add_component(id, &mut self.vm_camera, camera);

        if is_active {
            self.vm_active.clear();
            entities.add_component(id, &mut self.vm_active, CameraActive);
        }
        if auto_updated {
            entities.add_component(id, &mut self.vm_auto_update, AutoUpdate);
        }
        id
    }

    pub fn activate_camera(&mut self, entities: &mut EntitiesViewMut, camera_id: EntityId) {
        if !self.vm_camera.contains(camera_id) {
            panic!("Error activating camera. Provided entity doesn't have the camera component.");
        }

        self.vm_active.clear();
        entities.add_component(camera_id, &mut self.vm_active, CameraActive);
    }
}

//===============================================================

pub struct CameraBundleViewMutBorrower;
impl<'v> IntoBorrow for CameraBundleViewMut<'_> {
    type Borrow = CameraBundleViewMutBorrower;
}

type CameraBundleViewMutComponents<'v> = (
    TransformBundleViewMut<'v>,
    ViewMut<'v, Camera>,
    ViewMut<'v, CameraActive>,
    ViewMut<'v, AutoUpdate>,
);

impl<'v> Borrow<'v> for CameraBundleViewMutBorrower {
    type View = CameraBundleViewMut<'v>;

    fn borrow(
        world: &'v shipyard::World,
        last_run: Option<u32>,
        current: u32,
    ) -> Result<Self::View, shipyard::error::GetStorage> {
        let (vm_transform_bundle, vm_camera, vm_active, vm_auto_update) =
            <CameraBundleViewMutComponents as IntoBorrow>::Borrow::borrow(
                world, last_run, current,
            )?;

        Ok(CameraBundleViewMut {
            vm_transform_bundle,
            vm_camera,
            vm_active,
            vm_auto_update,
        })
    }
}

//===============================================================
