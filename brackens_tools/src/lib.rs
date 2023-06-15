//===============================================================

pub mod general;
pub mod input;
pub mod runner;
pub mod upkeep;
pub mod window;

//===============================================================

pub use glam;

pub use winit::{
    self,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

pub use input::{InputManager, KeyManager, MouseKeyManager, MousePositionManager};
pub use runner::{Runner, RunnerCore, RunnerLoopEvent};

//===============================================================
//----------------------------------------------
//________________________________________
