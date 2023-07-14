//===============================================================

mod components;
mod systems;
mod uniques;

//===============================================================

pub use components::*;
pub use systems::*;
pub use uniques::*;

//===============================================================

#[cfg(feature = "runner")]
use shipyard::Workload;

#[cfg(feature = "runner")]
pub struct ToolsWorkload;
#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for ToolsWorkload {
    fn setup(&self) -> Workload {
        Workload::new("").with_system(setup_tools)
    }

    fn start(&self) -> Workload {
        Workload::new("").with_system(sys_update_upkeep)
    }

    fn pre_update(&self) -> Workload {
        Workload::new("")
            .with_system(sys_tick_timers)
            .with_system(sys_process_input_events)
    }

    fn post_update(&self) -> Workload {
        Workload::new("")
            .with_system(sys_reset_key_input)
            .with_system(sys_reset_mouse_input)
            .with_system(sys_reset_mouse_pos)
    }
}

//===============================================================
