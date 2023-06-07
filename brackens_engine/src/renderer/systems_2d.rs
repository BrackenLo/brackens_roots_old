//===============================================================

use std::collections::HashMap;

use brackens_renderer::renderer_2d::{RawTextureInstance, RendererTexture, TextureID};
use rayon::prelude::ParallelIterator;
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, View};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue, SurfaceConfig, WindowSize},
    spatial_components::GlobalTransform,
};

use super::{
    components::{RenderPassTools, Visible},
    components_2d::*,
    tools::CameraBundleView,
};

#[cfg(feature = "debug")]
use crate::tool_components::TimingsDebug;

//===============================================================

// Texture Stuff

pub fn sys_setup_texture_renderer(
    all_storages: AllStoragesView,
    device: UniqueView<Device>,
    config: UniqueView<SurfaceConfig>,
    window_size: UniqueView<WindowSize>,
) {
    all_storages.add_unique(TextureRenderer::new(&device.0, &config.0, window_size.0));
}

//--------------------------------------------------

pub fn sys_add_new_textures(
    mut renderer: UniqueViewMut<TextureRenderer>,
    texture_storage: UniqueView<AssetStorage<RendererTexture>>,
) {
    for new in texture_storage.get_just_added() {
        renderer.add_texture(new);
    }
}

pub fn sys_remove_unloaded_textures(
    texture_storage: UniqueView<AssetStorage<RendererTexture>>,
    mut renderer: UniqueViewMut<TextureRenderer>,
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

    v_camera_bundle: CameraBundleView,
) {
    if !v_camera_bundle.has_camera() {
        renderer.resize_both(&device.0, &queue.0, window_size.size());
        return;
    }
    renderer.resize_depth(&device.0, window_size.size());
}

pub fn sys_update_camera(
    queue: UniqueView<Queue>,
    mut renderer: UniqueViewMut<TextureRenderer>,
    v_camera_bundle: CameraBundleView,
) {
    if v_camera_bundle.has_changed() {
        renderer.resize_projection(&queue.0, &v_camera_bundle.get_projection());
    }
}

//--------------------------------------------------

pub fn sys_process_textures(
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,

    mut renderer: UniqueViewMut<TextureRenderer>,
    v_texture: View<Texture>,
    v_visible: View<Visible>,
    v_global_transform: View<GlobalTransform>,

    #[cfg(feature = "debug")] mut debug_log: UniqueViewMut<TimingsDebug>,
) {
    #[cfg(feature = "debug")]
    let instant = std::time::Instant::now();

    //--------------------------------------------------

    // let result: HashMap<TextureID, Vec<RawTextureInstance>>
    renderer.unprocessed_draw_data = (&v_texture, &v_visible, &v_global_transform)
        .par_iter()
        .fold(
            || HashMap::<TextureID, Vec<RawTextureInstance>>::new(),
            |mut acc, (texture, visible, transform)| {
                if visible.visible {
                    let instance = RawTextureInstance {
                        transform: (GlobalTransform::from_scale(texture.size.extend(1.))
                            + transform)
                            .to_raw(),
                        color: texture.color,
                    };

                    acc.entry(texture.handle.id())
                        .or_insert(vec![])
                        .push(instance);
                }
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

    //--------------------------------------------------

    // (&v_texture, &v_visible, &v_global_transform)
    //     .iter()
    //     .for_each(|(texture, visible, transform)| {
    //         if !visible.visible {
    //             ()
    //         }

    //         let instance = RawTextureInstance {
    //             transform: (GlobalTransform::from_scale(texture.size.extend(1.)) + transform)
    //                 .to_raw(),
    //             color: texture.color,
    //         };

    //         renderer.draw_texture(texture.handle.id(), instance);

    //         ()
    //     });

    //--------------------------------------------------

    #[cfg(feature = "debug")]
    debug_log.add_log(
        "Process textures initial loop time".into(),
        instant.elapsed().as_secs_f32(),
        Some(colored::Color::BrightRed),
    );

    #[cfg(feature = "debug")]
    let instant = std::time::Instant::now();

    renderer.process_texture(&device.0, &queue.0);

    #[cfg(feature = "debug")]
    debug_log.add_log(
        "renderer processing textures time".into(),
        instant.elapsed().as_secs_f32(),
        Some(colored::Color::BrightRed),
    );
}

pub fn sys_render_textures(
    mut renderer: UniqueViewMut<TextureRenderer>,
    mut render_tools: UniqueViewMut<RenderPassTools>,
) {
    renderer.render(&mut render_tools.0);
}

//===============================================================
