//===============================================================

use std::collections::HashMap;

use brackens_renderer::{
    render_tools,
    renderer_2d::{RawTextureInstance, TextureID},
    wgpu::{PresentMode, SurfaceError},
    RenderComponents, RenderPrefs, Size,
};
use rayon::prelude::ParallelIterator;
use shipyard::{
    AllStoragesView, EntitiesView, Get, IntoIter, IntoWithId, UniqueView, UniqueViewMut, View,
    ViewMut,
};

use super::{
    CameraActive, CameraOrthographic, CameraPerspective, ClearColor, Device, Queue,
    RenderPassTools, Renderer2D, Surface, SurfaceConfig, Texture2D,
};
#[cfg(feature = "runner")]
use crate::runner::uniques::{ResizeEvent, RunnerErrorManager};
use crate::{
    assets::AssetStorage,
    tools::{Transform, Window},
};

//===============================================================

pub fn setup_renderer(all_storages: AllStoragesView, window: UniqueView<Window>) {
    let RenderComponents {
        device,
        queue,
        surface,
        config,
    } = RenderComponents::new_winit(
        RenderPrefs {
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        },
        window.inner(),
    );

    all_storages.add_unique(Device::new(device));
    all_storages.add_unique(Queue::new(queue));
    all_storages.add_unique(Surface::new(surface));
    all_storages.add_unique(SurfaceConfig::new(config));

    all_storages.add_unique(ClearColor::new(0.3, 0.3, 0.3));
}

//===============================================================

/// Use with run_with_data.
/// New size must not have 0 as width or height
pub fn resize(
    new_size: Size<u32>,
    device: UniqueView<Device>,
    surface: UniqueView<Surface>,
    mut config: UniqueViewMut<SurfaceConfig>,
) {
    config.set_size(new_size);
    surface.inner().configure(device.inner(), config.inner());
}

#[cfg(feature = "runner")]
pub fn sys_resize(
    resize: UniqueView<ResizeEvent>,
    device: UniqueView<Device>,
    surface: UniqueView<Surface>,
    mut config: UniqueViewMut<SurfaceConfig>,
) {
    config.set_size(resize.inner());
    surface.inner().configure(device.inner(), config.inner());
}

//===============================================================

#[cfg(feature = "runner")]
pub fn sys_start_render_pass(
    all_storages: AllStoragesView,
    device: UniqueView<Device>,
    surface: UniqueView<Surface>,
) {
    match render_tools::start_render_pass(device.inner(), surface.inner()) {
        Ok(tools) => {
            all_storages.add_unique(RenderPassTools::new(tools));
        }
        Err(e) => match e {
            SurfaceError::Lost => {
                let mut val = all_storages
                    .borrow::<UniqueViewMut<RunnerErrorManager>>()
                    .unwrap();
                val.add_error(crate::runner::uniques::RunnerError::ForceResize);
            }
            SurfaceError::OutOfMemory => panic!("Error: Surface out of memory"),
            _ => {}
        },
    }
}

pub fn sys_end_render_pass(all_storages: AllStoragesView, queue: UniqueView<Queue>) {
    if let Ok(tools) = all_storages.remove_unique::<RenderPassTools>() {
        render_tools::end_render_pass(queue.inner(), tools.destroy());
    }
}

pub fn sys_clear_background(
    mut render_pass: UniqueViewMut<RenderPassTools>,
    clear_color: UniqueView<ClearColor>,
) {
    render_tools::clear_background(render_pass.inner_mut(), clear_color.inner());
}

//===============================================================

pub fn sys_setup_renderer_2d(
    all_storages: AllStoragesView,
    device: UniqueView<Device>,
    config: UniqueView<SurfaceConfig>,
    window: UniqueView<Window>,
) {
    all_storages.add_unique(Renderer2D::new(
        device.inner(),
        config.inner(),
        window.size(),
    ));
}

//--------------------------------------------------

pub fn sys_resize_renderer_2d(
    resize: UniqueView<ResizeEvent>,

    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
    mut renderer: UniqueViewMut<Renderer2D>,

    v_orthographic: View<CameraOrthographic>,
    v_perspective: View<CameraPerspective>,
    v_active: View<CameraActive>,
) {
    if (&v_orthographic, &v_active).iter().next().is_none()
        && (&v_perspective, &v_active).iter().next().is_none()
    {
        renderer.resize_depth_and_projection(device.inner(), queue.inner(), resize.inner());
        return;
    }

    // Just resize depth if camera is active
    renderer.resize_depth(device.inner(), resize.inner());
}

pub fn sys_update_camera_active(entities: EntitiesView, mut v_active: ViewMut<CameraActive>) {
    // Look for component with camera active just added. Return if none found
    let id = match v_active.inserted().iter().with_id().next() {
        Some((id, _)) => id,
        None => return,
    };

    // Remove all camera active components. This should usually be just the new component and
    // the previously active component.
    v_active.clear();
    // Re-add the component
    entities.add_component(id, &mut v_active, CameraActive);

    println!("DEBUG: Camera active inserted - Make sure this is only printed once");
}

pub fn sys_renderer2d_update_camera(
    queue: UniqueView<Queue>,
    mut renderer: UniqueViewMut<Renderer2D>,

    v_orthographic: View<CameraOrthographic>,
    v_perspective: View<CameraPerspective>,
    v_active: View<CameraActive>,
    v_transform: View<Transform>,
) {
    // Check to see if orthographic camera with active component exists. Get the first one if so.
    if let Some((id, (camera, _))) = (&v_orthographic, &v_active).iter().with_id().next() {
        // Check to see if active camera has been modified
        if v_orthographic.is_inserted_or_modified(id)
            || v_active.is_inserted(id)
            || v_transform.is_inserted_or_modified(id)
        {
            if let Ok(transform) = v_transform.get(id) {
                // Camera has a transform component
                renderer.resize_projection(
                    queue.inner(),
                    &camera
                        .get_projection_transform(*transform.translation(), *transform.rotation()),
                );
                return;
            }
            // Camera without transform component
            renderer.resize_projection(queue.inner(), &camera.get_projection());
        }
        return;
    }

    // If no active orthographic cameras, check to see if perspective camera with active component exists.
    // Get the first one if so.
    if let Some((id, (camera, _))) = (&v_perspective, &v_active).iter().with_id().next() {
        if v_perspective.is_inserted_or_modified(id)
            || v_active.is_inserted(id)
            || v_transform.is_inserted_or_modified(id)
        {
            if let Ok(transform) = v_transform.get(id) {
                // Camera has a transform component
                renderer.resize_projection(
                    queue.inner(),
                    &camera
                        .get_projection_transform(*transform.translation(), *transform.rotation()),
                );
                return;
            }
        }
        return;
    }
}

//--------------------------------------------------

pub fn sys_renderer2d_process_textures(
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,

    mut renderer: UniqueViewMut<Renderer2D>,
    v_texture: View<Texture2D>,
    v_transform: View<Transform>,
) {
    *renderer.get_unprocessed_mut() = (&v_texture, &v_transform)
        .par_iter()
        .fold(
            || HashMap::<TextureID, Vec<RawTextureInstance>>::new(),
            |mut acc, (texture, transform)| {
                // Add RawTextureInstance to hashmap of values to be rendered
                acc.entry(texture.handle.id())
                    // Insert empty vec if texture not already present
                    .or_insert(vec![])
                    // Add instance of texture to new or existing hashmap entry
                    .push(RawTextureInstance {
                        transform: transform.to_raw(),
                        color: texture.color,
                        ..Default::default()
                    });
                acc
            },
        )
        .reduce_with(|mut m1, m2| {
            for (k, v) in m2 {
                m1.entry(k).or_insert(vec![]).extend(v);
            }
            m1
        })
        .unwrap();

    renderer.process_texture(device.inner(), queue.inner());
}

pub fn sys_renderer2d_render_textures(
    mut renderer: UniqueViewMut<Renderer2D>,
    mut render_tools: UniqueViewMut<RenderPassTools>,
    asset_storage: UniqueView<AssetStorage>,
) {
    renderer.render(&asset_storage, render_tools.inner_mut());
}

//===============================================================
