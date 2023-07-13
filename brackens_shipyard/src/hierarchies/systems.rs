//===============================================================

use shipyard::{Contains, EntityId, Get, IntoIter, IntoWithId, View, ViewMut};

use crate::{
    hierarchies::components::HierarchyIter,
    tools::{LocalTransform, Transform},
};

use super::components::{Child, Parent};

//===============================================================

pub fn sys_update_transforms(
    mut vm_transform: ViewMut<Transform>,
    v_local_transform: View<LocalTransform>,
    v_child: View<Child>,
) {
    (
        v_local_transform.inserted_or_modified(),
        &mut vm_transform,
        !&v_child,
    )
        .iter()
        .for_each(|(local_transform, mut transform, _)| *transform += local_transform);
}

pub fn sys_update_hierarchy_transforms(
    v_transform: View<Transform>,
    mut vm_local_transform: ViewMut<LocalTransform>,
    v_child: View<Child>,
) {
    (
        v_transform.inserted_or_modified(),
        &mut vm_local_transform,
        &v_child,
    )
        .iter()
        .for_each(|(transform, mut local_transform, child)| {
            if let Ok(parent_transform) = (v_transform).get(child.parent()) {
                *local_transform = (transform - parent_transform).into();
            }

            match (v_transform).get(child.parent()) {
                Ok(parent_transform) => *local_transform = (transform - parent_transform).into(),
                Err(_) => {}
            };
        });
}

pub fn sys_update_local_hierarchy_transforms(
    mut vm_transform: ViewMut<Transform>,
    v_local_transform: View<LocalTransform>,
    v_child: View<Child>,
    v_parent: View<Parent>,
) {
    let to_update = (
        v_local_transform.inserted_or_modified(),
        &vm_transform,
        &v_parent,
        &v_child,
    )
        .iter()
        .with_id()
        .filter_map(|(id, _)| {
            let mut found_new = false;
            for (ancestor_id, _) in (&v_parent, &v_child).ancestors(id) {
                if !(&vm_transform, &v_local_transform).contains(ancestor_id) {
                    break;
                }
                if v_local_transform.is_inserted_or_modified(ancestor_id) {
                    found_new = true;
                    break;
                }
            }

            match found_new {
                true => None,
                false => Some(id),
            }
        })
        .collect::<Vec<_>>();

    for update in to_update {
        // Check to see if we should use the parents transform data
        let parent_transform = if let Ok(child) = (&v_child).get(update) {
            if let Ok(transform) = (&vm_transform).get(child.parent()) {
                transform.clone()
            } else {
                Transform::default()
            }
        } else {
            Transform::default()
        };

        vm_transform[update] = &parent_transform + &v_local_transform[update];
        let current_transform = vm_transform[update].clone();

        for child in (&v_parent, &v_child).children(update) {
            update_child_transform_recursive(
                &v_child,
                &v_parent,
                &mut vm_transform,
                &v_local_transform,
                child,
                current_transform.clone(),
            )
        }
    }
}

fn update_child_transform_recursive(
    v_child: &View<Child>,
    v_parent: &View<Parent>,
    vm_transform: &mut ViewMut<Transform>,
    v_local_transform: &View<LocalTransform>,

    current_entity: EntityId,
    current_transform: Transform,
) {
    let current_transform = match (vm_transform).get(current_entity) {
        Ok(transform) => &*transform + &current_transform,
        Err(_) => return,
    };

    for child in (v_parent, v_child).children(current_entity) {
        update_child_transform_recursive(
            v_child,
            v_parent,
            vm_transform,
            v_local_transform,
            child,
            current_transform.clone(),
        );
    }

    vm_transform[current_entity] = current_transform;
}

//===============================================================
