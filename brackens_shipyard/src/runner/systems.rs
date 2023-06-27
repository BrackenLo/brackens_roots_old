//===============================================================

use brackens_renderer::Size;
use shipyard::AllStoragesView;

use super::uniques::ResizeEvent;

//===============================================================

pub fn resize(size: Size<u32>, all_storages: AllStoragesView) {
    all_storages.run_with_data(crate::renderer::resize, size);
    all_storages.add_unique(ResizeEvent::new(size));
}

pub fn sys_remove_resize(all_storages: AllStoragesView) {
    all_storages.remove_unique::<ResizeEvent>().unwrap();
}

//===============================================================
