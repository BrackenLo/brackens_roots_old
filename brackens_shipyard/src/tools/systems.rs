//===============================================================

use brackens_tools::{
    input::MouseButton,
    winit::event::{ElementState, KeyboardInput},
    DeviceEvent, WindowEvent,
};
use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, ViewMut};

use crate::tools::InputManager;

use super::{KeyManager, MouseKeyManager, MousePositionManager, Timer, UpkeepTracker};

//===============================================================

pub fn setup_tools(all_storages: AllStoragesView) {
    all_storages.add_unique(UpkeepTracker::new());
    all_storages.add_unique(InputManager::new());
}

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

//--------------------------------------------------

pub fn sys_update_all_input_managers(
    mut key_manager: UniqueViewMut<KeyManager>,
    mut mouse_key_manager: UniqueViewMut<MouseKeyManager>,
    mut mouse_pos_manager: UniqueViewMut<MousePositionManager>,
) {
    key_manager.reset();
    mouse_key_manager.reset();
    mouse_pos_manager.reset();
}

pub fn sys_update_input_manager(mut input_manager: UniqueViewMut<InputManager>) {
    input_manager.reset();
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

//--------------------------------------------------

pub fn input_manage_window_event(
    event: &WindowEvent,
    mut input_manager: UniqueViewMut<InputManager>,
) -> bool {
    input_manager.manage_window_event(event)
}

pub fn input_manage_device_event(
    event: &DeviceEvent,
    mut input_manager: UniqueViewMut<InputManager>,
) -> bool {
    input_manager.manage_device_event(event)
}

//===============================================================
