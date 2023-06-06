//===============================================================

use std::collections::HashSet;
use std::hash::Hash;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

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
    position: PhysicalPosition<f64>,
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
    pub fn set_position(&mut self, position: PhysicalPosition<f64>) {
        self.position = position;
        self.moved = true;
    }

    //----------------------------------------------

    pub fn position(&self) -> PhysicalPosition<f64> {
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
