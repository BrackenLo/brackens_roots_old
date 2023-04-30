//===============================================================

use brackens_tools::{
    asset_manager::Handle,
    renderer::{render_tools, texture_renderer::RawTextureInstance},
    wgpu::SurfaceError,
};
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, View, World};

use crate::shipyard_core::render_components::*;

use super::{
    core_components::{Device, Queue, Surface, SurfaceConfig, WindowSize},
    render_components::{ClearColor, RenderPassTools},
    spatial_components::GlobalTransform,
    tool_components::AssetStorage,
    UV, UVM,
};

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
    texture_storage: UV<AssetStorage<LoadedTexture>>,
) {
    for new in texture_storage.0.get_just_added() {
        renderer.add_texture(new);
    }
}

pub fn sys_remove_unloaded_textures(
    texture_storage: UV<AssetStorage<LoadedTexture>>,
    mut renderer: UVM<TextureRenderer>,
) {
    for handle in texture_storage.0.get_removed_assets() {
        renderer.remove_texture(*handle);
    }
}

//--------------------------------------------------

pub fn sys_resize_pipeline(
    queue: UniqueView<Queue>,
    window_size: UniqueView<WindowSize>,
    mut renderer: UniqueViewMut<TextureRenderer>,
) {
    renderer.resize(&queue.0, window_size.0);
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
// Functions for loading textures

pub fn load_texture<T: AsRef<str>>(
    world: &mut World,
    path: T,
    label: T,
    sampler: Option<brackens_tools::wgpu::SamplerDescriptor>,
) -> Handle<LoadedTexture> {
    world.run_with_data(
        |data: (T, T),
         mut texture_storage: UVM<AssetStorage<LoadedTexture>>,
         renderer: UV<TextureRenderer>,
         device: UV<Device>,
         queue: UV<Queue>| {
            //--------------------------------------------------

            let layout = renderer.get_layout();
            let sampler = match sampler {
                Some(val) => val,
                None => brackens_tools::wgpu::SamplerDescriptor::default(),
            };

            let texture = brackens_tools::renderer::texture::Texture::from_file(
                &device.0,
                &queue.0,
                data.0.as_ref(),
                data.1.as_ref(),
                &sampler,
            )
            .unwrap();

            let loaded_texture = LoadedTexture::load(&device.0, texture, layout);
            texture_storage.0.load_asset(loaded_texture)

            //--------------------------------------------------
        },
        (path, label),
    )
}

#[allow(unused)]
pub fn load_texture_bytes<T: AsRef<str>>(
    world: &mut World,
    bytes: &[u8],
    label: T,
    sampler: Option<brackens_tools::wgpu::SamplerDescriptor>,
) -> Handle<LoadedTexture> {
    world.run_with_data(
        |data: (&[u8], T),
         mut texture_storage: UVM<AssetStorage<LoadedTexture>>,
         renderer: UV<TextureRenderer>,
         device: UV<Device>,
         queue: UV<Queue>| {
            //--------------------------------------------------

            let layout = renderer.get_layout();
            let sampler = match sampler {
                Some(val) => val,
                None => brackens_tools::wgpu::SamplerDescriptor::default(),
            };

            let texture = brackens_tools::renderer::texture::Texture::from_bytes(
                &device.0,
                &queue.0,
                data.0,
                data.1.as_ref(),
                &sampler,
            )
            .unwrap();

            let loaded_texture = LoadedTexture::load(&device.0, texture, layout);
            texture_storage.0.load_asset(loaded_texture)

            //--------------------------------------------------
        },
        (bytes, label),
    )
}

//===============================================================
