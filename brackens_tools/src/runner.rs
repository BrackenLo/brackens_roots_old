//===============================================================

use log::info;
use winit::{
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::Window,
};

//===============================================================

pub trait RunnerCore {
    fn new(window: Window, event_loop: &EventLoop<RunnerLoopEvent>) -> Self;

    fn input(&mut self, event: WindowEvent);
    fn device_input(&mut self, device_id: DeviceId, event: DeviceEvent);

    fn main_events_cleared(&mut self);
    fn tick(&mut self);
}

pub enum RunnerLoopEvent {
    Exit,
}

//===============================================================

#[derive(Default)]
pub struct Runner {
    pub window_builder: winit::window::WindowBuilder,
}
impl Runner {
    pub fn new(window_builder: winit::window::WindowBuilder) -> Self {
        Self { window_builder }
    }

    pub fn run<RC: RunnerCore + 'static>(self) {
        //----------------------------------------------

        env_logger::init();
        info!("Initializing runner");

        //----------------------------------------------

        let event_loop = EventLoopBuilder::with_user_event().build();
        let window = self.window_builder.build(&event_loop).unwrap();

        let mut core = RC::new(window, &event_loop);

        //----------------------------------------------

        info!("Starting Event Loop");

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent { event, .. } => core.input(event),
            winit::event::Event::DeviceEvent { device_id, event } => {
                core.device_input(device_id, event)
            }
            winit::event::Event::MainEventsCleared => core.main_events_cleared(),
            winit::event::Event::RedrawRequested(_) => {
                core.tick();
            }
            winit::event::Event::UserEvent(event) => match event {
                RunnerLoopEvent::Exit => {
                    info!("Loop event exit recieved. Terminating Event Loop");
                    *control_flow = ControlFlow::Exit;
                }
            },

            // winit::event::Event::NewEvents(_) => todo!(),
            // winit::event::Event::Suspended => todo!(),
            // winit::event::Event::Resumed => todo!(),
            // winit::event::Event::RedrawEventsCleared => todo!(),
            // winit::event::Event::LoopDestroyed => todo!(),
            _ => {}
        });

        //----------------------------------------------
    }
}

//===============================================================
