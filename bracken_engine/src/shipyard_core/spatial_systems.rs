//===============================================================

use shipyard::{IntoIter, View, ViewMut};

use super::spatial_components::*;

//===============================================================

pub fn sys_update_transforms(
    transforms: View<Transform>,
    mut global_transforms: ViewMut<GlobalTransform>,
) {
    for (transform, mut global_transform) in
        (transforms.inserted_or_modified(), &mut global_transforms).iter()
    {
        global_transform.0 = transform.0;
    }
}

//===============================================================
