//===============================================================

use brackens_tools::{
    input::MouseButton,
    winit::event::{ElementState, KeyboardInput},
};
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, ViewMut};

use super::{KeyManager, MouseKeyManager, MousePositionManager, Timer, UpkeepTracker};

//===============================================================

pub fn sys_setup_upkeep(all_storages: AllStoragesView) {
    all_storages.add_unique(UpkeepTracker::new());
}

pub fn sys_setup_input_managers(all_storages: AllStoragesView) {
    all_storages.add_unique(KeyManager::new());
    all_storages.add_unique(MouseKeyManager::new());
    all_storages.add_unique(MousePositionManager::new());
}

//===============================================================

pub fn sys_update_upkeep(mut upkeep: UniqueViewMut<UpkeepTracker>) {
    upkeep.tick();
}

pub fn sys_tick_timers(upkeep: UniqueView<UpkeepTracker>, mut vm_timer: ViewMut<Timer>) {
    let delta = upkeep.delta();
    for timer in (&mut vm_timer).iter() {
        timer.tick(delta);
    }
}

//===============================================================

/// Call with world::run_with_data
pub fn manage_keyboard_input(
    KeyboardInput {
        state,
        virtual_keycode,
        ..
    }: KeyboardInput,
    mut key_manager: UniqueViewMut<KeyManager>,
) {
    key_manager.manage_input(state, virtual_keycode);
}

/// Call with world::run_with_data
pub fn manage_mouse_key_input(
    (state, input_button): (ElementState, MouseButton),
    mut mouse_key_manager: UniqueViewMut<MouseKeyManager>,
) {
    mouse_key_manager.manage_input(state, input_button);
}

/// Call with world::run_with_data
pub fn manage_mouse_input(
    input: (f64, f64),
    mut mouse_pos_manager: UniqueViewMut<MousePositionManager>,
) {
    mouse_pos_manager.add_movement(input);
}

//===============================================================
