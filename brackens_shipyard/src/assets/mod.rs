//===============================================================

use shipyard::Workload;

mod components;
mod systems;

//===============================================================

pub use components::*;
pub use systems::*;

//===============================================================

pub fn workload_asset_storage_startup() -> Workload {
    Workload::new("").with_system(setup_asset_storage)
}

//===============================================================
