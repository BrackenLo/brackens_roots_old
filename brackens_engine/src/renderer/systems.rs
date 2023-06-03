//===============================================================

use brackens_renderer::{
    render_tools,
    renderer_2d::{RawTextureInstance, RendererTexture},
    renderer_3d::{RawMeshInstance, RendererMaterial, RendererMesh},
};

use brackens_renderer::wgpu::SurfaceError;
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, View, World};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue, Surface, SurfaceConfig, WindowSize},
    spatial_components::GlobalTransform,
    ClearColor, UV, UVM,
};

use super::components::*;

//===============================================================

pub fn start_render_pass(world: &mut World) -> Result<(), SurfaceError> {
    match world.run(sys_start_render_pass) {
        Ok(render_tools) => {
            world.add_unique(RenderPassTools(render_tools));
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn sys_start_render_pass(
    device: UniqueView<Device>,
    surface: UniqueView<Surface>,
) -> Result<render_tools::RenderPassTools, SurfaceError> {
    render_tools::start_render_pass(&device.0, &surface.0)
}

pub fn sys_clear_background(
    mut render_pass: UniqueViewMut<RenderPassTools>,
    clear_color: UniqueView<ClearColor>,
) {
    render_tools::clear_background(&mut render_pass.0, clear_color.0);
}

pub fn sys_end_render_pass(world: &mut World) {
    if let Ok(render_pass) = world.remove_unique::<RenderPassTools>() {
        world.run(|queue: UniqueView<Queue>| {
            render_tools::end_render_pass(&queue.0, render_pass.0);
        });
    }
}

//===============================================================
// Texture Stuff

pub fn sys_setup_texture_renderer(
    all_storages: AllStoragesView,
    device: UV<Device>,
    config: UV<SurfaceConfig>,
) {
    all_storages.add_unique(TextureRenderer::new(&device.0, &config.0));
}

//--------------------------------------------------

pub fn sys_add_new_textures(
    mut renderer: UVM<TextureRenderer>,
    texture_storage: UV<AssetStorage<RendererTexture>>,
) {
    for new in texture_storage.get_just_added() {
        renderer.add_texture(new);
    }
}

pub fn sys_remove_unloaded_textures(
    texture_storage: UV<AssetStorage<RendererTexture>>,
    mut renderer: UVM<TextureRenderer>,
) {
    for handle in texture_storage.get_removed_assets() {
        renderer.remove_texture(*handle);
    }
}

//--------------------------------------------------

pub fn sys_resize_pipeline(
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
    window_size: UniqueView<WindowSize>,
    mut renderer: UniqueViewMut<TextureRenderer>,
    mut renderer2: UniqueViewMut<ModelRenderer>,
) {
    renderer.resize(&device.0, &queue.0, window_size.0);
    renderer2.resize(&device.0, &queue.0, window_size.0);
}

//--------------------------------------------------

pub fn sys_process_textures(
    device: UV<Device>,
    queue: UV<Queue>,

    mut renderer: UVM<TextureRenderer>,
    textures: View<Texture>,
    visible: View<Visible>,
    global_transforms: View<GlobalTransform>,
) {
    for (texture, visible, transform) in (&textures, &visible, &global_transforms).iter() {
        // If a texture is invisible, ignore it
        if !visible.visible {
            continue;
        }

        let transform = GlobalTransform::from_scale(texture.size.extend(1.)) + transform;

        let instance = RawTextureInstance {
            transform: transform.to_raw(),
            color: texture.color,
        };

        renderer.draw_texture(texture.handle.id(), instance);
    }

    renderer.process_texture(&device.0, &queue.0);
}

pub fn sys_render_textures(
    mut renderer: UVM<TextureRenderer>,
    mut render_tools: UVM<RenderPassTools>,
) {
    renderer.render(&mut render_tools.0);
}

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
