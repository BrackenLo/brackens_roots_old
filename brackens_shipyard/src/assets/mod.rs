//===============================================================

use shipyard::Workload;

mod systems;
mod uniques;

//===============================================================

pub use systems::*;
pub use uniques::*;

//===============================================================

pub fn workload_asset_storage_startup() -> Workload {
    Workload::new("").with_system(setup_asset_storage)
}

//===============================================================
