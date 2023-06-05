//===============================================================

use brackens_renderer::render_tools;

use brackens_renderer::wgpu::SurfaceError;
use shipyard::{IntoIter, UniqueView, UniqueViewMut, View, ViewMut, World};

use crate::{
    core_components::{Device, Queue, Surface, WindowSize},
    tool_components::AutoUpdate,
    ClearColor,
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

pub fn sys_resize_camera(
    window_size: UniqueView<WindowSize>,
    mut vm_camera: ViewMut<Camera>,
    v_auto_update: View<AutoUpdate>,
) {
    for (mut camera, _) in (&mut vm_camera, &v_auto_update).iter() {
        match &mut camera.projection {
            CameraProjection::Perspective { aspect, .. } => {
                *aspect = window_size.width() as f32 / window_size.height() as f32;
            }
            CameraProjection::Orthographic {
                left,
                right,
                bottom,
                top,
                ..
            } => {
                *left = window_size.width() as f32 / -2.;
                *right = window_size.width() as f32 / 2.;
                *bottom = window_size.height() as f32 / -2.;
                *top = window_size.height() as f32 / 2.;
            }
        }
    }
}

//===============================================================
