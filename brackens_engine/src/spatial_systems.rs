//===============================================================

use shipyard::{
    Contains, EntityId, Get, IntoIter, IntoWithId, IntoWorkload, View, ViewMut, Workload,
};

use crate::spatial_components::{
    Child, GlobalTransform, HierarchyIter, Parent, Transform, UseParentTransform,
};

//===============================================================

pub fn workload_update_tranforms() -> Workload {
    (sys_update_transforms, sys_update_hierarchy_transforms).into_workload()
}

/// Update all transforms that don't have parents
pub fn sys_update_transforms(
    v_transform: View<Transform>,
    mut vm_global_transform: ViewMut<GlobalTransform>,
    v_child: View<Child>,
) {
    (
        v_transform.inserted_or_modified(),
        &mut vm_global_transform,
        !&v_child,
    )
        .iter()
        .for_each(|(transform, mut global_transform, _)| global_transform.0 = *transform);

    // (
    //     v_transform.inserted_or_modified(),
    //     &mut vm_global_transform,
    //     !&v_child,
    // )
    //     .par_iter()
    //     .for_each(|(transform, mut global_transform, _)| {
    //         global_transform.0 = transform.clone();
    //     });
}

/// Update all transforms in a hierarchy
pub fn sys_update_hierarchy_transforms(
    v_transform: View<Transform>,
    mut vm_global_transform: ViewMut<GlobalTransform>,
    v_child: View<Child>,
    v_parent: View<Parent>,
    v_use_transform: View<UseParentTransform>,
) {
    let mut to_update = std::collections::HashSet::new();

    // Iterate through modified parent entities that aren't children. These all need to be updated
    for (id, _) in (
        v_transform.inserted_or_modified(),
        &vm_global_transform,
        &v_parent,
        !&v_child,
    )
        .iter()
        .with_id()
    {
        to_update.insert(id);
    }

    // Iterate through modified children. We check their parents for changes also and only update
    // the highest up the tree as their change will cascade onto all their children.
    for (id, _) in (v_transform.modified(), &vm_global_transform, &v_child)
        .iter()
        .with_id()
    {
        let mut to_add = id;

        // Only check entities ancestors if it uses their transforms.
        if v_use_transform.contains(id) {
            for ancestor in (&v_parent, &v_child).ancestors(id) {
                // If the ancestor doesn't have the components for transform or global transform then
                // we stop there
                if !(&v_transform, &vm_global_transform).contains(ancestor) {
                    break;
                }
                // If the ancestor is modified, is should be used instead as it is higher in the tree
                if v_transform.is_modified(ancestor) {
                    to_add = ancestor;
                }

                // If the ancestor doesn't use its parents transform, we don't need to go any further
                if !v_use_transform.contains(ancestor) {
                    break;
                }
            }
        }
        to_update.insert(to_add);
    }

    for update in to_update {
        // Check to see if we should use the parents transform data
        let parent_transform = if let Ok((child, _)) = (&v_child, &v_use_transform).get(update) {
            if let Ok(global_transform) = (&vm_global_transform).get(child.parent()) {
                global_transform.0
            } else {
                Transform::default()
            }
        } else {
            Transform::default()
        };

        vm_global_transform[update] = GlobalTransform(parent_transform + v_transform[update]);
        let current_transform = vm_global_transform[update].0;

        for child in (&v_parent, &v_child).children(update) {
            update_child_transform(
                &v_child,
                &v_parent,
                &v_use_transform,
                &v_transform,
                &mut vm_global_transform,
                child,
                current_transform,
            );
        }
    }
}

fn update_child_transform(
    v_child: &View<Child>,
    v_parent: &View<Parent>,
    v_use_transform: &View<UseParentTransform>,
    v_transforms: &View<Transform>,
    vm_global_transform: &mut ViewMut<GlobalTransform>,
    current_entity: EntityId,
    current_transform: Transform,
) {
    let current_transform = match (v_transforms, v_use_transform).get(current_entity) {
        Ok((transform, _)) => *transform + current_transform,
        Err(_) => return,
    };

    for child in (v_parent, v_child).children(current_entity) {
        update_child_transform(
            v_child,
            v_parent,
            v_use_transform,
            v_transforms,
            vm_global_transform,
            child,
            current_transform,
        );
    }

    vm_global_transform[current_entity] = GlobalTransform(current_transform);
}

//===============================================================

//===============================================================
