//===============================================================

mod components;
mod systems;
mod uniques;

//===============================================================

pub use components::*;
use shipyard::SystemModificator;
pub use systems::*;
pub use uniques::*;

//===============================================================

#[cfg(feature = "runner")]
use {
    crate::runner::uniques::ResizeEvent,
    shipyard::{Workload, WorkloadModificator},
};

//--------------------------------------------------

#[cfg(feature = "runner")]
pub struct RendererWorkload;

#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for RendererWorkload {
    fn pre_setup(&self, world: &mut shipyard::World) {
        world.run(setup_renderer);
    }
    fn setup(&self, _world: &mut shipyard::World) {}

    fn start(&self) -> Workload {
        Workload::new("").with_system(sys_resize.skip_if_missing_unique::<ResizeEvent>())
    }

    fn pre_render(&self) -> Workload {
        Workload::new("").with_system(sys_start_render_pass).merge(
            &mut Workload::new("")
                .skip_if_missing_unique::<RenderPassTools>()
                .after_all(sys_start_render_pass)
                .with_system(sys_clear_background),
        )
    }

    fn post_render(&self) -> Workload {
        Workload::new("")
            .with_system(sys_end_render_pass.skip_if_missing_unique::<RenderPassTools>())
    }
}

//--------------------------------------------------

#[cfg(feature = "runner")]
pub struct Renderer2dWorkload;

#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for Renderer2dWorkload {
    fn setup(&self, world: &mut shipyard::World) {
        world.run(sys_setup_renderer_2d);
    }

    fn post_update(&self) -> Workload {
        Workload::new("")
            .with_system(sys_resize_renderer_2d.skip_if_missing_unique::<ResizeEvent>())
            .with_system(sys_update_camera_active)
            .with_system(sys_renderer2d_update_camera.after_all(sys_update_camera_active))
    }

    fn render(&self) -> Workload {
        Workload::new("")
            .with_system(sys_renderer2d_process_textures)
            .with_system(
                sys_renderer2d_render_textures
                    .after_all(sys_renderer2d_process_textures)
                    .skip_if_missing_unique::<RenderPassTools>(),
            )
    }
}

//===============================================================
