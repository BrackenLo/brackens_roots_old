//===============================================================

use shipyard::{Borrow, BorrowInfo, EntitiesViewMut, EntityId, IntoBorrow, ViewMut};

use crate::prelude::{GlobalTransform, Transform};

//===============================================================

pub struct TransformBundleViewMut<'v> {
    vm_transform: ViewMut<'v, Transform>,
    vm_global_transform: ViewMut<'v, GlobalTransform>,
}
impl<'v> TransformBundleViewMut<'v> {
    pub fn create_transform(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
    ) -> EntityId {
        entities.add_entity(
            (&mut self.vm_transform, &mut self.vm_global_transform),
            (transform, GlobalTransform::default()),
        )
    }

    pub fn add_transform(
        &mut self,
        entities: &mut EntitiesViewMut,
        entity: EntityId,
        transform: Transform,
    ) {
        entities.add_component(
            entity,
            (&mut self.vm_transform, &mut self.vm_global_transform),
            (transform, GlobalTransform::default()),
        );
    }

    pub fn add_global_transform(
        &mut self,
        entities: &mut EntitiesViewMut,
        entity: EntityId,
        global_transform: GlobalTransform,
    ) {
        entities.add_component(
            entity,
            (&mut self.vm_transform, &mut self.vm_global_transform),
            (global_transform.0, global_transform),
        );
    }
}

pub struct TransformBundleViewMutBorrower;
impl<'v> IntoBorrow for TransformBundleViewMut<'_> {
    type Borrow = TransformBundleViewMutBorrower;
}

type TransformBundleViewMutComponents<'v> = (ViewMut<'v, Transform>, ViewMut<'v, GlobalTransform>);

impl<'v> Borrow<'v> for TransformBundleViewMutBorrower {
    type View = TransformBundleViewMut<'v>;

    fn borrow(
        world: &'v shipyard::World,
        last_run: Option<u32>,
        current: u32,
    ) -> Result<Self::View, shipyard::error::GetStorage> {
        let (vm_transform, vm_global_transform) =
            <TransformBundleViewMutComponents as IntoBorrow>::Borrow::borrow(
                world, last_run, current,
            )?;

        Ok(TransformBundleViewMut {
            vm_transform,
            vm_global_transform,
        })
    }
}

unsafe impl BorrowInfo for TransformBundleViewMut<'_> {
    fn borrow_info(info: &mut Vec<shipyard::info::TypeInfo>) {
        <TransformBundleViewMutComponents>::borrow_info(info);
    }
}

//===============================================================
