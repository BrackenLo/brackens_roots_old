//===============================================================

use brackens_renderer::{
    render_tools,
    wgpu::{PresentMode, SurfaceError},
    RenderComponents, RenderPrefs,
};
use shipyard::{AllStoragesView, UniqueView, UniqueViewMut};

use crate::tools::Window;

use super::{ClearColor, Device, Queue, RenderPassTools, Surface, SurfaceConfig};

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

pub fn sys_start_render_pass(
    all_storages: AllStoragesView,
    device: UniqueView<Device>,
    surface: UniqueView<Surface>,
) -> Result<(), SurfaceError> {
    match render_tools::start_render_pass(device.inner(), surface.inner()) {
        Ok(tools) => {
            all_storages.add_unique(RenderPassTools::new(tools));
            Ok(())
        }
        Err(e) => Err(e),
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
