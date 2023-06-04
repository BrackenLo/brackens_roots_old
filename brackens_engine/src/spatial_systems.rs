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
    transforms: View<Transform>,
    mut global_transforms: ViewMut<GlobalTransform>,
    children: View<Child>,
) {
    for (transform, mut global_transform, _) in (
        transforms.inserted_or_modified(),
        &mut global_transforms,
        !&children,
    )
        .iter()
    {
        global_transform.0 = transform.clone();
    }
}

/// Update all transforms in a hierarchy
pub fn sys_update_hierarchy_transforms(
    transforms: View<Transform>,
    mut global_transforms: ViewMut<GlobalTransform>,
    children: View<Child>,
    parents: View<Parent>,
    use_transforms: View<UseParentTransform>,
) {
    let mut to_update = std::collections::HashSet::new();

    // Iterate through modified parent entities that aren't children. These all need to be updated
    for (id, _) in (
        transforms.inserted_or_modified(),
        &global_transforms,
        &parents,
        !&children,
    )
        .iter()
        .with_id()
    {
        to_update.insert(id);
    }

    // Iterate through modified children. We check their parents for changes also and only update
    // the highest up the tree as their change will cascade onto all their children.
    for (id, _) in (transforms.modified(), &global_transforms, &children)
        .iter()
        .with_id()
    {
        let mut to_add = id;

        // Only check entities ancestors if it uses their transforms.
        if use_transforms.contains(id) {
            for ancestor in (&parents, &children).ancestors(id) {
                // If the ancestor doesn't have the components for transform or global transform then
                // we stop there
                if !(&transforms, &global_transforms).contains(ancestor) {
                    break;
                }
                // If the ancestor is modified, is should be used instead as it is higher in the tree
                if transforms.is_modified(ancestor) {
                    to_add = ancestor;
                }

                // If the ancestor doesn't use its parents transform, we don't need to go any further
                if !use_transforms.contains(ancestor) {
                    break;
                }
            }
        }
        to_update.insert(to_add);
    }

    for update in to_update {
        // Check to see if we should use the parents transform data
        let parent_transform = if let Ok((child, _)) = (&children, &use_transforms).get(update) {
            if let Ok(global_transform) = (&global_transforms).get(child.parent()) {
                global_transform.0
            } else {
                Transform::default()
            }
        } else {
            Transform::default()
        };

        global_transforms[update] = GlobalTransform(parent_transform + transforms[update]);
        let current_transform = global_transforms[update].0;

        for child in (&parents, &children).children(update) {
            update_child_transform(
                &children,
                &parents,
                &use_transforms,
                &transforms,
                &mut global_transforms,
                child,
                current_transform,
            );
        }
    }
}

fn update_child_transform(
    children: &View<Child>,
    parents: &View<Parent>,
    use_transforms: &View<UseParentTransform>,
    transforms: &View<Transform>,
    global_transforms: &mut ViewMut<GlobalTransform>,
    current_entity: EntityId,
    current_transform: Transform,
) {
    let current_transform = match (transforms, use_transforms).get(current_entity) {
        Ok((transform, _)) => *transform + current_transform,
        Err(_) => return,
    };

    for child in (parents, children).children(current_entity) {
        update_child_transform(
            children,
            parents,
            use_transforms,
            transforms,
            global_transforms,
            child,
            current_transform,
        );
    }

    global_transforms[current_entity] = GlobalTransform(current_transform);
}

//===============================================================

//===============================================================
