//===============================================================

use shipyard::Workload;

mod components;
mod systems;
mod uniques;

//===============================================================

pub use components::*;
pub use systems::*;
pub use uniques::*;

//===============================================================

pub fn workload_tools_startup() -> Workload {
    Workload::new("")
        .with_system(sys_setup_upkeep)
        .with_system(sys_setup_input_managers)
}

//===============================================================
