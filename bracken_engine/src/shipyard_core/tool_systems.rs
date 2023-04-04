//===============================================================

use brackens_tools::{
    asset_manager::Asset,
    exports::{DynamicImage, FontArc},
    renderer::texture::LoadedTexture,
};
use shipyard::{AllStoragesView, IntoWorkload, UniqueViewMut, Workload};

use super::tool_components::*;

//===============================================================

pub fn sys_setup_asset_storage(all_storages: AllStoragesView) {
    register_asset_storage::<DynamicImage>(&all_storages);
    register_asset_storage::<LoadedTexture>(&all_storages);
    register_asset_storage::<FontArc>(&all_storages);
}

pub fn register_asset_storage<T: Asset>(all_storages: &AllStoragesView) {
    all_storages.add_unique(AssetStorage::<T>::new());
}

pub fn wl_reset_asset_storage() -> Workload {
    (
        sys_reset_asset_storage::<DynamicImage>,
        sys_reset_asset_storage::<LoadedTexture>,
        sys_reset_asset_storage::<FontArc>,
    )
        .into_workload()
}

pub fn sys_reset_asset_storage<T: Asset>(mut asset_storage: UniqueViewMut<AssetStorage<T>>) {
    asset_storage.0.tick();
}

//===============================================================
