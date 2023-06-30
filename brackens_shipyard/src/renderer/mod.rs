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
use shipyard::{Workload, WorkloadModificator};

#[cfg(feature = "runner")]
pub struct RendererWorkload;
#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for RendererWorkload {
    fn pre_setup(&self, world: &shipyard::World) {
        world.run(setup_renderer);
    }
    fn setup(&self, _world: &shipyard::World) {}

    fn start(&self) -> Workload {
        Workload::new("").with_system(sys_resize)
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
            .with_system(sys_start_render_pass.skip_if_missing_unique::<RenderPassTools>())
    }
}

//===============================================================
