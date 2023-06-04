//===============================================================

use brackens_renderer::renderer_3d::{RawMeshInstance, RendererMaterial, RendererMesh};
use shipyard::{IntoIter, UniqueView, UniqueViewMut, View};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue, WindowSize},
    spatial_components::GlobalTransform,
};

use super::{
    components::{RenderPassTools, Visible},
    components_3d::{Model, ModelRenderer},
};

//===============================================================

pub fn sys_resize_pipeline(
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
    window_size: UniqueView<WindowSize>,
    mut renderer: UniqueViewMut<ModelRenderer>,
) {
    renderer.resize(&device.0, &queue.0, window_size.0);
}

//===============================================================

pub fn sys_process_models(
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
    mesh_storage: UniqueView<AssetStorage<RendererMesh>>,
    material_storage: UniqueView<AssetStorage<RendererMaterial>>,

    mut renderer: UniqueViewMut<ModelRenderer>,
    v_model: View<Model>,
    v_visible: View<Visible>,
    v_transform: View<GlobalTransform>,
) {
    for (model, visible, transform) in (&v_model, &v_visible, &v_transform).iter() {
        if !visible.visible {
            continue;
        }

        let instance = RawMeshInstance {
            transform: transform.to_raw(),
        };
        renderer.render_model(model, instance);
    }

    renderer.process_data(&device.0, &queue.0, &mesh_storage, &material_storage)
}

pub fn sys_render_models(
    mut renderer: UniqueViewMut<ModelRenderer>,
    mut render_tools: UniqueViewMut<RenderPassTools>,
) {
    renderer.render(&mut render_tools.0)
}

//===============================================================

// pub fn sys_check_models(meshes: View<Mesh>, materials: View<Material>) {
//     let mut checked = vec![];
//     for (id, (mesh, material)) in (meshes.modified(), &materials).iter().with_id() {
//         checked.push(id);
//     }

//     for (id, (mesh, material)) in (&meshes, materials.modified()).iter().with_id() {
//         if checked.contains(&id) {
//             continue;
//         }
//     }
// }

//===============================================================
