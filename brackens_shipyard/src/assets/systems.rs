//===============================================================

use shipyard::{AllStoragesView, UniqueViewMut};

use super::AssetStorage;

//===============================================================

pub fn setup_assets(all_storages: AllStoragesView) {
    all_storages.add_unique(AssetStorage::new());
}

pub fn sys_reset_asset_storage(mut asset_storage: UniqueViewMut<AssetStorage>) {
    asset_storage.tick();
}

//===============================================================
