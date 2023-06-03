//===============================================================

use brackens_renderer::renderer_3d::{RawMeshInstance, RendererMaterial, RendererMesh};
use shipyard::{IntoIter, View};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue},
    spatial_components::GlobalTransform,
    UV, UVM,
};

use super::{
    components::{RenderPassTools, Visible},
    components_3d::{Model, ModelRenderer},
};

//===============================================================

pub fn sys_process_models(
    device: UV<Device>,
    queue: UV<Queue>,
    mesh_storage: UV<AssetStorage<RendererMesh>>,
    material_storage: UV<AssetStorage<RendererMaterial>>,

    mut renderer: UVM<ModelRenderer>,
    models: View<Model>,
    visible: View<Visible>,
    transforms: View<GlobalTransform>,
) {
    for (model, visible, transform) in (&models, &visible, &transforms).iter() {
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

pub fn sys_render_models(mut renderer: UVM<ModelRenderer>, mut render_tools: UVM<RenderPassTools>) {
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
