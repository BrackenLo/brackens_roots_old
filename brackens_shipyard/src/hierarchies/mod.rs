//===============================================================

mod components;
mod systems;
mod uniques;

pub use components::*;
pub use systems::*;
pub use uniques::*;

//===============================================================

#[cfg(feature = "runner")]
use shipyard::Workload;

#[cfg(feature = "runner")]
pub struct HierarchyWorkload;
#[cfg(feature = "runner")]
impl crate::runner::RunnerWorkloads for HierarchyWorkload {
    fn setup(&self) -> Workload {
        Workload::new("")
    }

    fn post_update(&self) -> Workload {
        Workload::new("")
            .with_system(sys_update_transforms)
            .with_system(sys_update_hierarchy_transforms)
            .with_system(sys_update_local_hierarchy_transforms)
    }
}

//===============================================================
