//===============================================================

use brackens_renderer::{image::DynamicImage, textures::RendererTexture};

use brackens_assets::Asset;
use shipyard::{
    AllStoragesView, IntoIter, IntoWorkload, UniqueView, UniqueViewMut, ViewMut, Workload,
};

use super::{core_components::UpkeepTracker, tool_components::*};

//===============================================================

pub fn sys_setup_asset_storage(all_storages: AllStoragesView) {
    register_asset_storage::<DynamicImage>(&all_storages);
    register_asset_storage::<RendererTexture>(&all_storages);
    // register_asset_storage::<FontArc>(&all_storages);
}

pub fn register_asset_storage<T: Asset>(all_storages: &AllStoragesView) {
    all_storages.add_unique(AssetStorage::<T>::new());
}

pub fn wl_reset_asset_storage() -> Workload {
    (
        sys_reset_asset_storage::<DynamicImage>,
        sys_reset_asset_storage::<RendererTexture>,
        // sys_reset_asset_storage::<FontArc>,
    )
        .into_workload()
}

pub fn sys_reset_asset_storage<T: Asset>(mut asset_storage: UniqueViewMut<AssetStorage<T>>) {
    asset_storage.0.tick();
}

//===============================================================

pub fn sys_tick_timers(upkeep: UniqueView<UpkeepTracker>, mut timers: ViewMut<Timer>) {
    let delta = upkeep.delta();
    for timer in (&mut timers).iter() {
        timer.0.tick(delta);
    }
}

//===============================================================
