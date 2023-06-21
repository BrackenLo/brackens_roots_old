//===============================================================

use shipyard::Workload;

mod components;
pub mod systems;

//===============================================================

pub use components::*;
pub use systems::*;

//===============================================================

pub fn workload_tools_startup() -> Workload {
    Workload::new("")
        .with_system(sys_setup_upkeep)
        .with_system(sys_setup_input_managers)
}

//===============================================================
