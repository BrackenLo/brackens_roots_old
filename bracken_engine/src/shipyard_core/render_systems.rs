//===============================================================

use std::collections::HashMap;

use brackens_tools::{
    asset_manager::HandleID,
    bytemuck,
    renderer::{render_tools, texture_renderer::RawTextureInstance},
    wgpu::{self, util::DeviceExt, SurfaceError},
};
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, View, World};

use crate::shipyard_core::render_components::*;

use super::{
    core_components::{Device, Queue, Surface, SurfaceConfig},
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

pub fn sys_setup_texture_renderer(
    all_storages: AllStoragesView,
    device: UV<Device>,
    config: UV<SurfaceConfig>,
) {
    all_storages.add_unique(TextureRenderer::new(&device.0, &config.0));
}

pub fn sys_process_textures(
    device: UV<Device>,
    queue: UV<Queue>,

    mut renderer: UVM<TextureRenderer>,
    textures: View<Texture>,
    visible: View<Visible>,
    global_transforms: View<GlobalTransform>,
) {
    // Create a hashmap of all the textures used this frame
    let mut texture_calls: HashMap<HandleID, TextureDrawCall> = HashMap::new();

    for (texture, visible, transform) in (&textures, &visible, &global_transforms).iter() {
        // If a texture is invisible, ignore it
        if !visible.0 {
            continue;
        }

        let id = texture.handle.id();

        // Check if the texture has been used already this frame
        if let Some(draw_calls) = texture_calls.get_mut(&id) {
            // If so, we can just add this texture data to the already existing data
            draw_calls.to_draw.push(RawTextureInstance {
                transform: transform.0.to_raw(),
                color: [1., 1., 1., 1.],
            });
            continue;
        }

        // Otherwise, add the texture data to the hashmap of textures used this frame
        texture_calls.insert(
            id,
            TextureDrawCall {
                handle: texture.handle.clone_weak(),
                to_draw: vec![RawTextureInstance {
                    transform: transform.0.to_raw(),
                    color: [1., 1., 1., 1.],
                }],
            },
        );
    }

    for vals in texture_calls {
        renderer.should_render.push(vals.0);
        if let Some(mut draw_call) = renderer.final_draw_calls.get_mut(&vals.0) {
            if draw_call.instance_count >= vals.1.to_draw.len() as u32 {
                queue.0.write_buffer(
                    &draw_call.instances,
                    0,
                    bytemuck::cast_slice(&vals.1.to_draw),
                );
                continue;
            }

            //else increase size of buffer
            draw_call.instances = device
                .0
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Texture Instance Buffer - Naming TODO"),
                    contents: bytemuck::cast_slice(&vals.1.to_draw),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            draw_call.instance_count = vals.1.to_draw.len() as u32;
        } else {
            renderer.final_draw_calls.insert(
                vals.0,
                FinalTextureDrawCall {
                    texture_bind_group: vals.1.handle.get().bind_group,
                    instances: todo!(),
                    instance_count: todo!(),
                },
            );
        }
    }

    // if renderer.final_draw_calls.contains_key()

    todo!()
}

pub fn sys_remove_unloaded_textures(
    texture_storage: UV<AssetStorage<LoadedTexture>>,
    mut renderer: UVM<TextureRenderer>,
) {
    for handle in texture_storage.0.get_removed_assets() {
        renderer.remove_texture(*handle);
    }
}

pub fn sys_render_textures(
    mut renderer: UVM<TextureRenderer>,
    mut render_tools: UVM<RenderPassTools>,
) {
    renderer.render(&mut render_tools.0);
    todo!()
}

//===============================================================
