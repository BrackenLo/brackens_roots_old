//===============================================================

use brackens_renderer::Size;
use shipyard::{AllStoragesView, UniqueViewMut};

use super::uniques::{InputEventManager, MiscEventManager, ResizeEvent};

//===============================================================

pub fn resize(size: Size<u32>, all_storages: AllStoragesView) {
    all_storages.run_with_data(crate::renderer::resize, size);
    all_storages.add_unique(ResizeEvent::new(size));
}

pub fn sys_remove_resize(all_storages: AllStoragesView) {
    all_storages.remove_unique::<ResizeEvent>().unwrap();
}

pub fn sys_clear_input_events(mut events: UniqueViewMut<InputEventManager>) {
    events.0.clear();
}

pub fn sys_clear_misc_events(mut events: UniqueViewMut<MiscEventManager>) {
    events.0.clear();
}

//===============================================================
