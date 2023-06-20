//===============================================================

use shipyard::{AllStoragesView, UniqueViewMut};

use super::components::AssetStorage;

//===============================================================

pub fn setup_asset_storage(all_storages: AllStoragesView) {
    all_storages.add_unique(AssetStorage::new());
}

pub fn sys_reset_asset_storage(mut asset_storage: UniqueViewMut<AssetStorage>) {
    asset_storage.tick();
}

//===============================================================
