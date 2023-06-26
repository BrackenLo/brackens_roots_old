//===============================================================

use brackens_renderer::Size;
use brackens_tools::{EventLoopProxy, RunnerCore, RunnerLoopEvent, WindowEvent};
use shipyard::{UniqueView, UniqueViewMut, World};

use crate::{
    assets::setup_assets,
    renderer::setup_renderer,
    tools::{
        input_manage_device_event, input_manage_window_event, setup_tools, ResizeEvent, Window,
    },
};

//===============================================================

// shipyard game state
// #[allow(unused)]
pub trait ShipyardCore {
    fn new(world: &World, event_proxy: EventLoopProxy<RunnerLoopEvent>) -> Self;
    #[cfg(feature = "renderer")]
    fn resize(&mut self, world: &World, new_size: Size<u32>) {}
    #[cfg(not(feature = "renderer"))]
    fn resize(&mut self, new_size: PhysicalSize<u32>) {}
    fn update(&mut self, world: &World);
    fn render(&mut self, world: &World);
    fn input(&mut self, world: &World, event: WindowEvent);
}

// shipyard core
pub struct ShipyardRunner<Core: ShipyardCore> {
    core: Core,
    world: World,
}

impl<Core: ShipyardCore> RunnerCore for ShipyardRunner<Core> {
    fn new(
        window: brackens_tools::Window,
        event_loop: &brackens_tools::EventLoop<brackens_tools::RunnerLoopEvent>,
    ) -> Self {
        //--------------------------------------------------

        let world = World::new();

        world.add_unique(Window::new(window));

        #[cfg(feature = "assets")]
        world.run(setup_assets);

        #[cfg(feature = "renderer")]
        world.run(setup_renderer);

        #[cfg(feature = "tools")]
        world.run(setup_tools);

        //--------------------------------------------------

        let core = Core::new(&world, event_loop.create_proxy());

        //--------------------------------------------------

        Self { core, world }

        //--------------------------------------------------
    }

    fn input(&mut self, event: brackens_tools::WindowEvent) {
        if self.world.run_with_data(input_manage_window_event, &event) {
            return;
        }

        match event {
            WindowEvent::Resized(new_size)
            | WindowEvent::ScaleFactorChanged {
                new_inner_size: &mut new_size,
                ..
            } => self.resize(new_size.into()),
            _ => {
                self.core.input(&self.world, event);
            }
        }
    }

    fn device_input(
        &mut self,
        _device_id: brackens_tools::DeviceId,
        event: brackens_tools::DeviceEvent,
    ) {
        self.world.run_with_data(input_manage_device_event, &event);
    }

    fn main_events_cleared(&mut self) {
        self.world
            .run(|window: UniqueView<Window>| window.request_redraw());
    }

    fn tick(&mut self) {
        self.core.update(&self.world);

        self.render();
        self.core.render(&self.world);
        self.end_render();
    }
}
impl<Core: ShipyardCore> ShipyardRunner<Core> {
    pub fn resize(
        &mut self,
        #[cfg(feature = "renderer")] new_size: Size<u32>,
        #[cfg(not(feature = "renderer"))] new_size: PhysicalSize<u32>,
    ) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        #[cfg(feature = "renderer")]
        self.world.run(
            |device: UniqueView<crate::renderer::Device>,
             surface: UniqueView<crate::renderer::Surface>,
             mut config: UniqueViewMut<crate::renderer::SurfaceConfig>| {
                config.set_size(new_size);
                surface.inner().configure(device.inner(), config.inner());
            },
        );

        self.world.add_unique(ResizeEvent::new(new_size));
        self.core.resize(&self.world, new_size);
    }

    pub fn render(&mut self) {}
    pub fn end_render(&mut self) {}
}

//===============================================================

//===============================================================
