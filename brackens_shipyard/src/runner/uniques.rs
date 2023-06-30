//===============================================================

use std::{slice::Iter, vec::Drain};

use brackens_renderer::Size;
use brackens_tools::{
    input::{KeyCode, MouseButton},
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, MouseScrollDelta, TouchPhase},
    },
    DeviceEvent, DeviceId, WindowEvent,
};
use shipyard::Unique;

//===============================================================

pub enum WindowEventTypes {
    Resize(ResizeEvent),
    Misc(MiscEvent),
    Input(InputEvent),
    None,
}

pub fn generate_device_event(event: DeviceEvent, device_id: DeviceId) -> WindowEventTypes {
    match event {
        DeviceEvent::MouseMotion { delta } => {
            WindowEventTypes::Input(InputEvent::RawMouseMotion { device_id, delta })
        }
        DeviceEvent::MouseWheel { delta } => {
            WindowEventTypes::Input(InputEvent::RawMouseWheel { device_id, delta })
        }
        // DeviceEvent::Added => todo!(),
        // DeviceEvent::Removed => todo!(),
        // DeviceEvent::Motion { axis, value } => todo!(),
        // DeviceEvent::Button { button, state } => todo!(),
        // DeviceEvent::Key(_) => todo!(),
        // DeviceEvent::Text { codepoint } => todo!(),
        _ => WindowEventTypes::None,
    }
}

pub fn generate_window_event(event: WindowEvent) -> WindowEventTypes {
    match event {
        //--------------------------------------------------
        WindowEvent::Resized(new_size)
        | WindowEvent::ScaleFactorChanged {
            new_inner_size: &mut new_size,
            ..
        } => {
            if new_size.width == 0 || new_size.height == 0 {
                return WindowEventTypes::None;
            }
            WindowEventTypes::Resize(ResizeEvent::new(new_size.into()))
        }

        //--------------------------------------------------
        WindowEvent::Moved(_) => WindowEventTypes::Misc(MiscEvent::Moved),
        WindowEvent::CloseRequested => WindowEventTypes::Misc(MiscEvent::CloseRequested),
        WindowEvent::Destroyed => WindowEventTypes::Misc(MiscEvent::Destroyed),
        WindowEvent::DroppedFile(_) => WindowEventTypes::Misc(MiscEvent::DroppedFile),
        WindowEvent::HoveredFile(_) => WindowEventTypes::Misc(MiscEvent::HoveredFile),
        WindowEvent::HoveredFileCancelled => {
            WindowEventTypes::Misc(MiscEvent::HoveredFileCancelled)
        }
        WindowEvent::ReceivedCharacter(_) => WindowEventTypes::Misc(MiscEvent::ReceivedCharacter),
        WindowEvent::Focused(_) => WindowEventTypes::Misc(MiscEvent::Focused),
        WindowEvent::ModifiersChanged(_) => WindowEventTypes::Misc(MiscEvent::ModifiersChanged),
        WindowEvent::Ime(_) => WindowEventTypes::Misc(MiscEvent::Ime),
        WindowEvent::ThemeChanged(_) => WindowEventTypes::Misc(MiscEvent::ThemeChanged),
        WindowEvent::Occluded(_) => WindowEventTypes::Misc(MiscEvent::Occluded),

        //--------------------------------------------------
        WindowEvent::KeyboardInput {
            device_id, input, ..
        } => {
            if input.virtual_keycode.is_none() {
                return WindowEventTypes::None;
            }
            WindowEventTypes::Input(InputEvent::KeyboardInput {
                device_id,
                key_code: input.virtual_keycode.unwrap(),
                state: input.state,
            })
        }
        WindowEvent::CursorMoved {
            device_id,
            position,
            ..
        } => WindowEventTypes::Input(InputEvent::CursorMoved {
            device_id,
            position,
        }),
        WindowEvent::CursorEntered { device_id } => {
            WindowEventTypes::Input(InputEvent::CursorEntered { device_id })
        }
        WindowEvent::CursorLeft { device_id } => {
            WindowEventTypes::Input(InputEvent::CursorLeft { device_id })
        }
        WindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
            ..
        } => WindowEventTypes::Input(InputEvent::MouseWheel {
            device_id,
            delta,
            phase,
        }),
        WindowEvent::MouseInput {
            device_id,
            state,
            button,
            ..
        } => WindowEventTypes::Input(InputEvent::MouseInput {
            device_id,
            state,
            button,
        }),
        WindowEvent::TouchpadMagnify { .. } => WindowEventTypes::Input(InputEvent::TouchpadMagnify),
        WindowEvent::SmartMagnify { .. } => WindowEventTypes::Input(InputEvent::SmartMagnify),
        WindowEvent::TouchpadRotate { .. } => WindowEventTypes::Input(InputEvent::TouchpadMagnify),
        WindowEvent::TouchpadPressure { .. } => {
            WindowEventTypes::Input(InputEvent::TouchpadPressure)
        }
        WindowEvent::AxisMotion { .. } => WindowEventTypes::Input(InputEvent::AxisMotion),
        WindowEvent::Touch(_) => WindowEventTypes::Input(InputEvent::Touch),
        //--------------------------------------------------
    }
}

//--------------------------------------------------

#[derive(Unique)]
#[track(Insertion)]
pub struct ResizeEvent(Size<u32>);
impl ResizeEvent {
    pub fn new(size: Size<u32>) -> Self {
        Self(size)
    }
    #[inline]
    pub fn inner(&self) -> Size<u32> {
        self.0
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height
    }
}

//--------------------------------------------------

pub enum MiscEvent {
    Moved,
    CloseRequested,
    Destroyed,
    DroppedFile,
    HoveredFile,
    HoveredFileCancelled,
    ReceivedCharacter,
    Focused,
    ModifiersChanged,
    Ime,
    ThemeChanged,
    Occluded,
}

//--------------------------------------------------

pub enum InputEvent {
    KeyboardInput {
        device_id: DeviceId,
        key_code: KeyCode,
        state: ElementState,
    },
    CursorMoved {
        device_id: DeviceId,
        position: PhysicalPosition<f64>,
    },
    CursorEntered {
        device_id: DeviceId,
    },
    CursorLeft {
        device_id: DeviceId,
    },
    MouseWheel {
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
    MouseInput {
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    },
    TouchpadMagnify,
    SmartMagnify,
    TouchpadRotate,
    TouchpadPressure,
    AxisMotion,
    Touch,
    RawMouseMotion {
        device_id: DeviceId,
        delta: (f64, f64),
    },
    RawMouseWheel {
        device_id: DeviceId,
        delta: MouseScrollDelta,
    },
}

//===============================================================

#[derive(Unique, Default)]
#[track(Modification)]
pub struct MiscEventManager(pub(crate) Vec<MiscEvent>);
impl MiscEventManager {
    pub fn iter(&self) -> Iter<MiscEvent> {
        self.0.iter()
    }
}

#[derive(Unique, Default)]
#[track(Modification)]
pub struct InputEventManager(pub(crate) Vec<InputEvent>);
impl InputEventManager {
    pub fn iter(&self) -> Iter<InputEvent> {
        self.0.iter()
    }
}

//===============================================================

#[derive(Unique, Default)]
pub struct RunnerErrorManager(Vec<RunnerError>);
impl RunnerErrorManager {
    pub fn add_error(&mut self, error: RunnerError) {
        self.0.push(error);
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<RunnerError> {
        self.0.drain(..)
    }
}

pub enum RunnerError {
    ForceResize,
}

//===============================================================
