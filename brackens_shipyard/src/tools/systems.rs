//===============================================================

use shipyard::{AllStoragesView, IntoIter, UniqueView, UniqueViewMut, ViewMut};

use crate::runner::uniques::InputEventManager;

use super::{KeyManager, MouseKeyManager, MousePositionManager, Timer, UpkeepTracker};

//===============================================================

pub fn setup_tools(all_storages: AllStoragesView) {
    all_storages.add_unique(UpkeepTracker::new());

    all_storages.add_unique(KeyManager::default());
    all_storages.add_unique(MouseKeyManager::default());
    all_storages.add_unique(MousePositionManager::default());
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

pub fn sys_reset_key_input(mut key_manager: UniqueViewMut<KeyManager>) {
    key_manager.reset();
}

pub fn sys_reset_mouse_input(mut mouse_key_manager: UniqueViewMut<MouseKeyManager>) {
    mouse_key_manager.reset();
}

pub fn sys_reset_mouse_pos(mut mouse_pos_manager: UniqueViewMut<MousePositionManager>) {
    mouse_pos_manager.reset();
}

//===============================================================

#[cfg(feature = "runner")]
pub fn sys_process_input_events(
    input_events: UniqueView<InputEventManager>,
    mut key_manager: UniqueViewMut<KeyManager>,
    mut mouse_key_manager: UniqueViewMut<MouseKeyManager>,
    mut mouse_pos_manager: UniqueViewMut<MousePositionManager>,
) {
    input_events.iter().for_each(|event| match event {
        crate::runner::uniques::InputEvent::KeyboardInput {
            key_code, state, ..
        } => key_manager.manage_input(*state, Some(*key_code)),
        crate::runner::uniques::InputEvent::CursorMoved { position, .. } => {
            mouse_pos_manager.set_position((*position).into());
        }
        crate::runner::uniques::InputEvent::MouseInput { state, button, .. } => {
            mouse_key_manager.manage_input(*state, *button);
        }
        crate::runner::uniques::InputEvent::RawMouseMotion { delta, .. } => {
            mouse_pos_manager.add_movement(*delta);
        }
        _ => {}
    });
}

//===============================================================
