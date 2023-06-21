//===============================================================

use std::collections::HashSet;
use std::hash::Hash;
use winit::event::ElementState;

pub use winit::event::MouseButton;

//===============================================================

pub struct ButtonManager<T> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}
impl<T> Default for ButtonManager<T> {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

impl<T> ButtonManager<T>
where
    T: Eq + Hash + Clone + Copy,
{
    pub fn new() -> Self {
        Self::default()
    }

    //----------------------------------------------

    fn add_pressed(&mut self, key: T) {
        self.pressed.insert(key);
        self.just_pressed.insert(key);
    }
    fn remove_pressed(&mut self, key: T) {
        self.pressed.remove(&key);
        self.just_released.insert(key);
    }
    pub fn reset(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    //----------------------------------------------

    pub fn manage_input(&mut self, state: ElementState, button: Option<T>) {
        match (state, button) {
            (winit::event::ElementState::Pressed, Some(key)) => self.add_pressed(key),
            (winit::event::ElementState::Released, Some(key)) => self.remove_pressed(key),
            _ => {}
        }
    }
    pub fn pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }
    pub fn just_pressed(&self, button: T) -> bool {
        self.just_pressed.contains(&button)
    }
    pub fn just_released(&self, button: T) -> bool {
        self.just_released.contains(&button)
    }

    //----------------------------------------------
}

//===============================================================

pub type KeyCode = winit::event::VirtualKeyCode;

pub type KeyManager = ButtonManager<KeyCode>;
pub type MouseKeyManager = ButtonManager<MouseButton>;

//===============================================================

#[derive(Default)]
pub struct MousePositionManager {
    position: (f64, f64),
    movement: (f64, f64),
    moved: bool,
}
impl MousePositionManager {
    pub fn reset(&mut self) {
        self.movement = (0.0, 0.0);
        self.moved = false;
    }
    pub fn add_movement(&mut self, movement: (f64, f64)) {
        self.movement.0 += movement.0;
        self.movement.1 += movement.1;
        self.moved = true;
    }
    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
        self.moved = true;
    }

    //----------------------------------------------

    pub fn position(&self) -> (f64, f64) {
        self.position
    }
    pub fn movement(&self) -> (f64, f64) {
        self.movement
    }
    pub fn moved(&self) -> bool {
        self.moved
    }

    //----------------------------------------------
}

//===============================================================

#[derive(Default)]
pub struct InputManager {
    keys: KeyManager,
    mouse_keys: MouseKeyManager,
    mouse_pos: MousePositionManager,
    mouse_on_screen: bool,
}
impl InputManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn manage_device_event(&mut self, event: &winit::event::DeviceEvent) -> bool {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => self.mouse_pos.add_movement(*delta),
            // winit::event::DeviceEvent::MouseWheel { delta } => todo!(),
            _ => return false,
        }
        true
    }

    pub fn manage_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput { input, .. } => {
                self.keys.manage_input(input.state, input.virtual_keycode)
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos.set_position((*position).into())
            }
            winit::event::WindowEvent::CursorEntered { .. } => self.mouse_on_screen = true,
            winit::event::WindowEvent::CursorLeft { .. } => self.mouse_on_screen = false,
            // winit::event::WindowEvent::MouseWheel { delta, phase, .. } => todo!(),
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_keys.manage_input(*state, Some(*button))
            }
            // winit::event::WindowEvent::ModifiersChanged(_) => todo!(),
            // winit::event::WindowEvent::TouchpadMagnify {
            //     device_id,
            //     delta,
            //     phase,
            // } => todo!(),
            // winit::event::WindowEvent::SmartMagnify { device_id } => todo!(),
            // winit::event::WindowEvent::TouchpadRotate {
            //     device_id,
            //     delta,
            //     phase,
            // } => todo!(),
            // winit::event::WindowEvent::TouchpadPressure {
            //     device_id,
            //     pressure,
            //     stage,
            // } => todo!(),
            // winit::event::WindowEvent::AxisMotion {
            //     device_id,
            //     axis,
            //     value,
            // } => todo!(),
            // winit::event::WindowEvent::Touch(_) => todo!(),
            _ => return false,
        }
        return true;
    }

    pub fn keys(&self) -> &KeyManager {
        &self.keys
    }
    pub fn mouse_buttons(&self) -> &MouseKeyManager {
        &self.mouse_keys
    }
    pub fn mouse_position(&self) -> &MousePositionManager {
        &self.mouse_pos
    }
}

//===============================================================
