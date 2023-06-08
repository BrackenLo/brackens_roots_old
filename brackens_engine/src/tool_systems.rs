//===============================================================

use brackens_renderer::{image::DynamicImage, renderer_2d::RendererTexture};

use brackens_assets::Asset;
use shipyard::{
    AllStoragesView, IntoIter, IntoWorkload, UniqueView, UniqueViewMut, ViewMut, Workload,
};

use crate::assets::AssetStorage;

use super::{core_components::UpkeepTracker, tool_components::*};

//===============================================================

#[cfg(feature = "debug")]
pub fn sys_start_timer(mut timings: UniqueViewMut<TimingsDebug>) {
    timings.timer = std::time::Instant::now();
}
#[cfg(feature = "debug")]
pub fn sys_record_time(
    data: (String, Option<colored::Color>),
    mut timings: UniqueViewMut<TimingsDebug>,
) {
    let (label, color) = data;
    let elapsed = timings.timer.elapsed().as_secs_f32();
    timings.add_log(label, elapsed, color);
}

#[cfg(feature = "debug")]
pub fn sys_record_time_and_reset(
    data: (String, Option<colored::Color>),
    mut timings: UniqueViewMut<TimingsDebug>,
) {
    let (label, color) = data;
    let elapsed = timings.timer.elapsed().as_secs_f32();
    timings.add_log(label, elapsed, color);

    timings.timer = std::time::Instant::now();
}

#[cfg(feature = "debug")]
pub fn sys_add_time(
    data: (String, f32, Option<colored::Color>),
    mut timings: UniqueViewMut<TimingsDebug>,
) {
    let (label, time, color) = data;
    timings.add_log(label, time, color);
}

//===============================================================

pub fn sys_setup_asset_storage(all_storages: AllStoragesView) {
    register_asset_storage::<DynamicImage>(&all_storages);
    register_asset_storage::<RendererTexture>(&all_storages);
}

pub fn register_asset_storage<T: Asset>(all_storages: &AllStoragesView) {
    all_storages.add_unique(AssetStorage::<T>::new());
}

pub fn wl_reset_asset_storage() -> Workload {
    (
        sys_reset_asset_storage::<DynamicImage>,
        sys_reset_asset_storage::<RendererTexture>,
    )
        .into_workload()
}

pub fn sys_reset_asset_storage<T: Asset>(mut asset_storage: UniqueViewMut<AssetStorage<T>>) {
    asset_storage.tick();
}

//===============================================================

pub fn sys_tick_timers(upkeep: UniqueView<UpkeepTracker>, mut vm_timer: ViewMut<Timer>) {
    let delta = upkeep.delta();
    for timer in (&mut vm_timer).iter() {
        timer.0.tick(delta);
    }
}

//===============================================================
