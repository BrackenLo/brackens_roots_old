//===============================================================

use std::collections::HashSet;

use rayon::prelude::ParallelIterator;
use shipyard::{
    Contains, EntitiesViewMut, EntityId, Get, IntoIter, IntoWithId, View, ViewMut, Workload,
};

use crate::spatial_components::*;

//===============================================================

pub(crate) fn workload_update_tranforms() -> Workload {
    Workload::new("UpdateTransformWorkload")
        .with_system(sys_update_transforms)
        .with_system(sys_check_dirty_transforms)
        .with_system(sys_update_dirty_transforms)
    // .with_system(sys_check_modified)
    // .with_system(sys_update_hierarchy_transforms)
}

//--------------------------------------------------

/// Update all transforms that don't have parents
pub(crate) fn sys_update_transforms(
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
}

//--------------------------------------------------

pub(crate) fn sys_check_dirty_transforms(
    entities: EntitiesViewMut,
    v_transform: View<Transform>,
    v_global_transform: View<GlobalTransform>,
    v_child: View<Child>,
    v_parent: View<Parent>,
    v_use_transform: View<UseParentTransform>,
    mut vm_transform_dirty: ViewMut<TransformDirty>,
) {
    let parent_ids = (
        v_transform.inserted_or_modified(),
        &v_global_transform,
        &v_parent,
        !&v_child,
    )
        .iter()
        .with_id()
        .map(|(id, _)| id);

    let child_ids = (
        v_transform.inserted_or_modified(),
        &v_global_transform,
        &v_child,
    )
        .iter()
        .with_id()
        .filter_map(|(id, _)| {
            let mut found_new = false;

            if v_use_transform.contains(id) {
                for ancestor_id in (&v_parent, &v_child).ancestors(id) {
                    if !(&v_transform, &v_global_transform).contains(ancestor_id) {
                        break;
                    }
                    if v_transform.is_inserted_or_modified(ancestor_id) {
                        found_new = true;
                        break;
                    }

                    if !v_use_transform.contains(ancestor_id) {
                        break;
                    }
                }
            }

            match found_new {
                true => None,
                false => Some(id),
            }
        });

    let to_update = parent_ids.chain(child_ids).collect::<HashSet<_>>();

    to_update.into_iter().for_each(|id| {
        if let Ok(child) = v_child.get(id) {
            entities.add_component(
                id,
                &mut vm_transform_dirty,
                TransformDirty(0, Some(child.parent())),
            );
        } else {
            entities.add_component(id, &mut vm_transform_dirty, TransformDirty(0, None));
        }

        (&v_parent, &v_child)
            .descendants(id)
            .for_each(|(id, parent, depth)| {
                entities.add_component(
                    id,
                    &mut vm_transform_dirty,
                    TransformDirty(depth as u8, Some(parent)),
                );
            });
    });
    vm_transform_dirty.sort_unstable();
}

pub(crate) fn sys_update_dirty_transforms(
    v_transform_dirty: View<TransformDirty>,
    v_transform: View<Transform>,
    mut vm_global_transform: ViewMut<GlobalTransform>,
) {
    (&v_transform_dirty)
        .iter()
        .with_id()
        .for_each(|(id, dirty)| {
            // Check if entity has parent or not
            match dirty.1 {
                Some(parent_id) => {
                    vm_global_transform[id] =
                        GlobalTransform(v_transform[id]) + vm_global_transform[parent_id]
                }
                None => {
                    GlobalTransform(v_transform[id]);
                }
            }
        });
}

//--------------------------------------------------

pub(crate) fn sys_check_modified(
    entities: EntitiesViewMut,
    v_transform: View<Transform>,
    mut vm_global_transform: ViewMut<GlobalTransform>,
    v_child: View<Child>,
    v_parent: View<Parent>,
    v_use_transform: View<UseParentTransform>,
    mut vm_transform_modified: ViewMut<TransformModified>,

    #[cfg(feature = "debug")] mut debug_log: shipyard::UniqueViewMut<
        crate::tool_components::TimingsDebug,
    >,
) {
    #[cfg(feature = "debug")]
    debug_log.reset_timer();

    let parent_ids = (
        v_transform.inserted_or_modified(),
        &vm_global_transform,
        &v_parent,
        !&v_child,
    )
        .iter()
        .with_id()
        .map(|(id, _)| id);

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Get Parent Updates".into(), Some(colored::Color::Yellow));

    // On first iteration this returns an iterator 65000 spaces long
    let child_ids = (
        v_transform.inserted_or_modified(),
        &vm_global_transform,
        &v_child,
    )
        .iter()
        .with_id()
        .filter_map(|(id, _)| {
            let mut found_new = false;

            if v_use_transform.contains(id) {
                for ancestor_id in (&v_parent, &v_child).ancestors(id) {
                    if !(&v_transform, &vm_global_transform).contains(ancestor_id) {
                        break;
                    }
                    if v_transform.is_inserted_or_modified(ancestor_id) {
                        found_new = true;
                        break;
                    }

                    if !v_use_transform.contains(ancestor_id) {
                        break;
                    }
                }
            }

            match found_new {
                true => None,
                false => Some(id),
            }
        });

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Get Child Updates".into(), Some(colored::Color::Yellow));

    let to_update = parent_ids.chain(child_ids).collect::<HashSet<_>>();
    // println!("Update len = {}", to_update.len());

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Merge Updates".into(), Some(colored::Color::Yellow));

    // No transforms to update - we can just stop here
    if to_update.len() == 0 {
        return;
    }

    // Iterate through all that needs updating
    to_update.into_iter().for_each(|id| {
        // Check if entity is child that will inherit parents transform
        let parent_transform = match v_child.get(id) {
            // If entity is a child, check to see if parent has a global_transform
            Ok(child) => match vm_global_transform.get(child.parent()) {
                Ok(global_transform) => Some(*global_transform),
                Err(_) => None,
            },
            Err(_) => None,
        };

        // Add the transform modified component to the changed entity with its parents transform
        entities.add_component(
            id,
            &mut vm_transform_modified,
            TransformModified(id, parent_transform),
        );
    });

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset(
        "Add modified component to entities".into(),
        Some(colored::Color::Yellow),
    );

    // Check to make sure we don't infinite loop
    let mut iterations = 0;
    loop {
        // Update all entities with the TransformModified component and get a list of all children that need updating
        let to_update = update_stuff(
            &v_transform,
            &mut vm_global_transform,
            &vm_transform_modified,
            &v_parent,
            &v_child,
        );

        #[cfg(feature = "debug")]
        debug_log
            .record_time_and_reset("Iteration Update Done".into(), Some(colored::Color::Yellow));

        // Clear the old list of TransformModifed as these have been dealt with
        vm_transform_modified.clear();

        // If no entities to update, we can now quit
        if to_update.len() == 0 {
            break;
        }

        // For each entity that needs updating still, attach the TransformModified component
        to_update.into_iter().for_each(|(id, modified)| {
            entities.add_component(id, &mut vm_transform_modified, modified);
        });

        #[cfg(feature = "debug")]
        debug_log.record_time_and_reset(
            "Iteration Add Components Done".into(),
            Some(colored::Color::Yellow),
        );

        iterations += 1;
        if iterations >= 100 {
            panic!("Spatial Systems check modified stuck in update loop after 100 iterations");
        }
    }

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Finished Iterating".into(), Some(colored::Color::Yellow));
}

fn update_stuff(
    v_transform: &View<Transform>,
    vm_global_transform: &mut ViewMut<GlobalTransform>,
    vm_transform_modified: &ViewMut<TransformModified>,

    v_parent: &View<Parent>,
    v_child: &View<Child>,
) -> Vec<(EntityId, TransformModified)> {
    // Iterate over all entities with both transforms and have been modified
    (v_transform, vm_global_transform, vm_transform_modified)
        .par_iter()
        .map(|(transform, mut global_transform, modified_id)| {
            // Update global transform with new transform
            *global_transform = GlobalTransform(match modified_id.1 {
                // Entity has parent to inherit transform from
                Some(parent_transform) => *transform + parent_transform.0,
                // Entity doesn't have parent. Just use own transform.
                None => *transform,
            });

            match v_parent.contains(modified_id.0) {
                // If entity is a parent, iterate through children and add TransformModified component
                true => (v_parent, v_child)
                    .children(modified_id.0)
                    .map(|child_id| {
                        (
                            child_id,
                            TransformModified(child_id, Some(*global_transform)),
                        )
                    })
                    .collect(),
                // Otherwise, return nothing
                false => vec![],
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

//--------------------------------------------------

/// Update all transforms in a hierarchy
pub(crate) fn sys_update_hierarchy_transforms(
    v_transform: View<Transform>,
    mut vm_global_transform: ViewMut<GlobalTransform>,
    v_child: View<Child>,
    v_parent: View<Parent>,
    v_use_transform: View<UseParentTransform>,

    #[cfg(feature = "debug")] mut debug_log: shipyard::UniqueViewMut<
        crate::tool_components::TimingsDebug,
    >,
) {
    //--------------------------------------------------

    #[cfg(feature = "debug")]
    debug_log.reset_timer();

    // Iterate through modified parent entities that aren't children but are parents. These all need to be updated
    let mut to_update = (
        v_transform.inserted_or_modified(),
        &vm_global_transform,
        &v_parent,
        !&v_child,
    )
        .iter()
        .with_id()
        .map(|(id, _)| id)
        .collect::<HashSet<_>>();

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Get Parent Updates".into(), Some(colored::Color::Yellow));

    //--------------------------------------------------

    // Iterate through modified children. We check their parents for changes also and only update
    // the highest up the tree as their change will cascade onto all their children.
    (
        v_transform.inserted_or_modified(),
        &vm_global_transform,
        &v_child,
    )
        .iter()
        .with_id()
        .for_each(|(id, _)| {
            let mut found_new = false;

            if v_use_transform.contains(id) {
                for ancestor_id in (&v_parent, &v_child).ancestors(id) {
                    if !(&v_transform, &vm_global_transform).contains(ancestor_id) {
                        break;
                    }
                    if v_transform.is_inserted_or_modified(ancestor_id) {
                        found_new = true;
                        break;
                    }

                    if !v_use_transform.contains(ancestor_id) {
                        break;
                    }
                }
            }

            if !found_new {
                to_update.insert(id);
            }
        });

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Get Child Updates".into(), Some(colored::Color::Yellow));

    //--------------------------------------------------

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

    #[cfg(feature = "debug")]
    debug_log.record_time_and_reset("Update all transforms".into(), Some(colored::Color::Yellow));

    //--------------------------------------------------
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
