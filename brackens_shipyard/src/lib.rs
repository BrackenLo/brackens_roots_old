//===============================================================

use shipyard::{Workload, World};

//===============================================================

pub use shipyard;

#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "tools")]
pub mod tools;

//===============================================================

//===============================================================

pub fn run_startup_systems(world: &mut World) {
    world.add_workload(startup_systems);
    world.run_workload(startup_systems).unwrap();
}

pub fn startup_systems() -> Workload {
    let workload = Workload::new("");

    #[cfg(feature = "assets")]
    let workload = workload.merge(&mut assets::workload_asset_storage_startup());

    #[cfg(feature = "tools")]
    let workload = workload.merge(&mut tools::workload_tools_startup());

    workload
}

//===============================================================
