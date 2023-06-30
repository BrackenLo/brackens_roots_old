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

pub fn sys_reset_input_manager(mut input_manager: UniqueViewMut<InputManager>) {
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

/// Call with world::run_with_data
pub fn input_manage_window_event(
    event: &WindowEvent,
    mut input_manager: UniqueViewMut<InputManager>,
) -> bool {
    input_manager.manage_window_event(event)
}

/// Call with world::run_with_data
pub fn input_manage_device_event(
    event: &DeviceEvent,
    mut input_manager: UniqueViewMut<InputManager>,
) -> bool {
    input_manager.manage_device_event(event)
}

//===============================================================

#[cfg(feature = "runner")]
pub fn sys_process_input_events(
    input_events: UniqueView<crate::runner::uniques::InputEventManager>,
    mut input_manager: UniqueViewMut<InputManager>,
) {
    input_events.iter().for_each(|val| match val {
        crate::runner::uniques::InputEvent::KeyboardInput {
            key_code, state, ..
        } => input_manager.set_keyboard_key(*state, Some(*key_code)),
        crate::runner::uniques::InputEvent::CursorMoved { position, .. } => {
            input_manager.set_mouse_position((*position).into())
        }
        crate::runner::uniques::InputEvent::CursorEntered { .. } => {
            input_manager.set_mouse_on_screen(true)
        }
        crate::runner::uniques::InputEvent::CursorLeft { .. } => {
            input_manager.set_mouse_on_screen(false)
        }
        crate::runner::uniques::InputEvent::MouseInput { state, button, .. } => {
            input_manager.set_mouse_key(*state, Some(*button));
        }
        crate::runner::uniques::InputEvent::RawMouseMotion { delta, .. } => {
            input_manager.add_mouse_movement(*delta)
        }
        // crate::runner::uniques::InputEvent::MouseWheel { device_id, delta, phase, } => todo!(),
        // crate::runner::uniques::InputEvent::TouchpadMagnify => todo!(),
        // crate::runner::uniques::InputEvent::SmartMagnify => todo!(),
        // crate::runner::uniques::InputEvent::TouchpadRotate => todo!(),
        // crate::runner::uniques::InputEvent::TouchpadPressure => todo!(),
        // crate::runner::uniques::InputEvent::AxisMotion => todo!(),
        // crate::runner::uniques::InputEvent::Touch => todo!(),
        // crate::runner::uniques::InputEvent::RawMouseWheel { device_id, delta } => todo!(),
        _ => {}
    });
}

//===============================================================
