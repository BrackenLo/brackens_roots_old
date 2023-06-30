//===============================================================

use log::info;
use winit::{
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

//===============================================================

pub trait RunnerCore {
    fn new(window: Window, event_loop: &EventLoop<RunnerLoopEvent>) -> Self;

    fn input(&mut self, event: WindowEvent);
    fn device_input(&mut self, device_id: DeviceId, event: DeviceEvent);

    fn main_events_cleared(&mut self);
    fn tick(&mut self);
}

pub trait RunnerDataCore<T>: RunnerCore {
    fn new_data(window: Window, event_loop: &EventLoop<RunnerLoopEvent>, data: T) -> Self;
}

//===============================================================

#[derive(Debug)]
pub enum RunnerLoopEvent {
    Exit,
}

//===============================================================

#[derive(Default)]
pub struct Runner;
impl Runner {
    //===============================================================

    pub fn run<RC: RunnerCore + 'static>(window_builder: WindowBuilder) {
        env_logger::init();
        info!("Initializing runner");

        let event_loop = EventLoopBuilder::with_user_event().build();
        let window = window_builder.build(&event_loop).unwrap();

        let core = RC::new(window, &event_loop);

        Self::run_loop(event_loop, core);
    }

    //===============================================================

    pub fn run_with_data<T, RDC: RunnerDataCore<T> + 'static>(
        window_builder: WindowBuilder,
        data: T,
    ) {
        env_logger::init();
        info!("Initializing runner");

        let event_loop = EventLoopBuilder::with_user_event().build();
        let window = window_builder.build(&event_loop).unwrap();

        let core = RDC::new_data(window, &event_loop, data);

        Self::run_loop(event_loop, core);
    }

    //===============================================================

    fn run_loop<RC: RunnerCore + 'static>(event_loop: EventLoop<RunnerLoopEvent>, mut core: RC) {
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
    }
}

//===============================================================
