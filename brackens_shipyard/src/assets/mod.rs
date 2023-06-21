//===============================================================

use crate::WorkloadStage;
use shipyard::Workload;

mod components;
mod systems;

//===============================================================

pub use components::AssetStorage;
pub use systems::{setup_asset_storage, sys_reset_asset_storage};

//===============================================================

pub fn workload_asset_storage_startup() -> Workload {
    Workload::new("").with_system(setup_asset_storage)
}

pub fn workload_asset_storage_main() -> Workload {
    let end = Workload::new(WorkloadStage::End).with_system(sys_reset_asset_storage);

    end
}

//===============================================================
