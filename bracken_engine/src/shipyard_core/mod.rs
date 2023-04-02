//===============================================================

use brackens_tools::{
    renderer::{RenderComponents, RenderPrefs},
    runner::{Runner, RunnerCore, RunnerLoopEvent},
    winit::{
        event::{DeviceEvent, DeviceId, WindowEvent},
        event_loop::{EventLoop, EventLoopProxy},
        window::{Window, WindowBuilder},
    },
};

use core_components::*;

pub mod core_components;

//===============================================================

#[derive(Default)]
pub struct ShipyardRunner {
    pub window_builder: WindowBuilder,
}
impl ShipyardRunner {
    pub fn run<GS: ShipyardGameState + 'static>(self) {
        let runner = Runner::new(self.window_builder);
        runner.run::<ShipyardCore<GS>>();
    }
}

//===============================================================

pub trait ShipyardGameState {
    fn new(world: &mut shipyard::World) -> Self;
    fn resize(&mut self, new_size: (u32, u32)) {}
    fn update(&mut self, world: &mut shipyard::World);
    fn render(&mut self, world: &mut shipyard::World);
}

pub struct ShipyardCore<GS: ShipyardGameState> {
    game_state: GS,
    proxy: EventLoopProxy<RunnerLoopEvent>,
    world: shipyard::World,
}
impl<GS: ShipyardGameState> RunnerCore for ShipyardCore<GS> {
    fn new(
        window: Window,
        event_loop: &EventLoop<brackens_tools::runner::RunnerLoopEvent>,
    ) -> Self {
        //--------------------------------------------------

        let mut world = shipyard::World::new();
        let render_components = RenderComponents::new(RenderPrefs::default(), &window);

        //--------------------------------------------------

        world.add_unique(Device(render_components.device));
        world.add_unique(Queue(render_components.queue));

        world.add_unique(Surface(render_components.surface));
        world.add_unique(SurfaceConfig(render_components.config));

        world.add_unique(WindowSize(window.inner_size()));
        world.add_unique(Window(window));

        //--------------------------------------------------

        world.add_unique(KeyManagerUnique::default());
        world.add_unique(MouseKeyManagerUnique::default());
        world.add_unique(MousePositionManagerUnique::default());

        world.add_unique(UpkeepTrackerUnique::default());

        //--------------------------------------------------

        let game_state = GS::new(&mut world);

        Self {
            world,
            game_state,
            proxy: event_loop.create_proxy(),
        }

        //--------------------------------------------------
    }

    fn input(&mut self, event: WindowEvent) {
        todo!()
    }

    fn device_input(&mut self, device_id: DeviceId, event: DeviceEvent) {
        todo!()
    }

    fn main_events_cleared(&mut self) {
        todo!()
    }

    fn tick(&mut self) {
        todo!()
    }
}

//===============================================================
