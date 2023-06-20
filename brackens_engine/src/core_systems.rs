//===============================================================

use brackens_tools::winit::event::{ElementState, KeyboardInput, MouseButton};
use shipyard::{UniqueView, UniqueViewMut};

use super::core_components::*;

//===============================================================

pub fn sys_update_tracker(mut tracker: UniqueViewMut<UpkeepTracker>) {
    tracker.0.tick()
}

//===============================================================

pub fn sys_manage_keyboard_input(
    KeyboardInput {
        state,
        virtual_keycode,
        ..
    }: KeyboardInput,
    mut key_manager: UniqueViewMut<KeyManager>,
) {
    key_manager.0.manage_input(state, virtual_keycode);
}

pub fn sys_manager_mouse_key_input(
    (state, input_button): (ElementState, MouseButton),
    mut mouse_key_manager: UniqueViewMut<MouseKeyManager>,
) {
    mouse_key_manager.0.manage_input(state, Some(input_button));
}

pub fn sys_manage_mouse_movement(
    input: (f64, f64),
    mut mouse_pos_manager: UniqueViewMut<MousePositionManager>,
) {
    mouse_pos_manager.0.add_movement(input);
}

pub fn sys_manager_mouse_position(
    mut input: (f64, f64),
    mut mouse_pos: UniqueViewMut<MousePositionManager>,
    screen_size: UniqueView<WindowSize>,
) {
    input.1 = screen_size.0.height as f64 - input.1;
    mouse_pos.0.set_position(input);
}

pub fn sys_reset_input(
    mut key_manager: UniqueViewMut<KeyManager>,
    mut mouse_key_manager: UniqueViewMut<MouseKeyManager>,
    mut mouse_pos: UniqueViewMut<MousePositionManager>,
) {
    key_manager.0.reset();
    mouse_key_manager.0.reset();
    mouse_pos.0.reset();
}

//===============================================================
