//===============================================================

use brackens_renderer::renderer_2d::{RawTextureInstance, RendererTexture};
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, View};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue, SurfaceConfig, WindowSize},
    spatial_components::GlobalTransform,
    UV, UVM,
};

use super::{
    components::{RenderPassTools, Visible},
    components_2d::*,
};

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
) {
    renderer.resize(&device.0, &queue.0, window_size.0);
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
