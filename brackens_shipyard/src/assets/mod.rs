//===============================================================

mod systems;
mod uniques;

//===============================================================

pub use systems::*;
pub use uniques::*;

//===============================================================

#[cfg(feature = "runner")]
use shipyard::Workload;

#[cfg(feature = "runner")]
pub struct AssetsWorkload;
#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for AssetsWorkload {
    fn pre_setup(&self) -> Workload {
        Workload::new("").with_system(setup_assets)
    }

    fn setup(&self) -> Workload {
        Workload::new("")
    }

    fn end(&self) -> Workload {
        Workload::new("").with_system(sys_reset_asset_storage)
    }
}

//===============================================================
