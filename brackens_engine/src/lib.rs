//===============================================================

use brackens_renderer::{RenderComponents, RenderPrefs, Size};

use brackens_tools::{
    runner::{Runner, RunnerCore, RunnerLoopEvent},
    winit::{
        self,
        event::{DeviceEvent, DeviceId, WindowEvent},
        event_loop::{EventLoop, EventLoopProxy},
        window::WindowBuilder,
    },
};

use core_components::*;
use log::{error, info, warn};
use prelude::ClearColor;
use shipyard::{AllStoragesViewMut, UniqueView, UniqueViewMut};

//===============================================================

pub mod prelude {
    pub use crate::core_components::{UpkeepTracker, WindowSize};
    pub use crate::renderer::components::{
        Camera, ClearColor, OrthographicCameraDescriptor, PerspectiveTargetCameraDescriptor,
        Visible,
    };
    pub use crate::spatial_components::{
        GlobalTransform, HierarchyBundle, HierarchyBundleTools, Transform, UseParentTransform,
    };
    pub use crate::{ShipyardGameState, ShipyardRunner};

    pub use brackens_tools::{
        glam,
        glam::{Mat4, Quat, Vec2, Vec3},
        input::KeyCode,
    };
    pub use shipyard::{
        self, Component, IntoIter, IntoWithId, Unique, UniqueView, UniqueViewMut, View, ViewMut,
    };

    #[cfg(feature = "2d")]
    pub use crate::renderer::{components_2d::Texture, tools_2d::load_texture};
}

//===============================================================

pub mod assets;
pub mod core_components;
mod core_systems;
pub mod renderer;
pub mod spatial_components;
mod spatial_systems;
pub mod spatial_tools;
pub mod tool_components;
mod tool_systems;

//===============================================================

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

#[allow(unused_variables)]
pub trait ShipyardGameState {
    fn new(world: &mut shipyard::World) -> Self;
    fn resize(&mut self, new_size: Size<u32>) {}
    fn update(&mut self, world: &mut shipyard::World);
    fn render(&mut self, world: &mut shipyard::World) {}
}

pub struct ShipyardCore<GS: ShipyardGameState> {
    game_state: GS,
    proxy: EventLoopProxy<RunnerLoopEvent>,
    world: shipyard::World,
}
impl<GS: ShipyardGameState> RunnerCore for ShipyardCore<GS> {
    fn new(
        window: winit::window::Window,
        event_loop: &EventLoop<brackens_tools::runner::RunnerLoopEvent>,
    ) -> Self {
        info!("Staring Shipyard core");

        //--------------------------------------------------

        let mut world = shipyard::World::new();

        let inner_size = brackens_renderer::Size {
            width: window.inner_size().width,
            height: window.inner_size().height,
        };
        let render_components = RenderComponents::new(
            RenderPrefs {
                // backends: todo!(),
                // dx12_compiler: todo!(),
                // power_preferences: todo!(),
                // features: todo!(),
                // limits: todo!(),
                present_mode: brackens_renderer::wgpu::PresentMode::Mailbox,
                ..Default::default()
            },
            &window,
            inner_size,
        );

        //--------------------------------------------------

        world.add_unique(Device(render_components.device));
        world.add_unique(Queue(render_components.queue));

        world.add_unique(Surface(render_components.surface));
        world.add_unique(SurfaceConfig(render_components.config));

        world.add_unique(WindowSize::from(window.inner_size()));
        world.add_unique(Window(window));

        //--------------------------------------------------

        world.add_unique(KeyManager::default());
        world.add_unique(MouseKeyManager::default());
        world.add_unique(MousePositionManager::default());

        world.add_unique(UpkeepTracker::default());

        //--------------------------------------------------

        world.add_unique(ClearColor([0.5, 0.4, 0.4]));

        //--------------------------------------------------

        world.run(tool_systems::sys_setup_asset_storage);
        world.add_workload(tool_systems::wl_reset_asset_storage);

        world.add_workload(spatial_systems::workload_update_tranforms);
        world.add_workload(renderer::workload_post_update_systems);

        //--------------------------------------------------

        renderer::run_startup_systems(&mut world);

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
        match event {
            //--------------------------------------------------
            // Screen size changed
            WindowEvent::Resized(new_size)
            | WindowEvent::ScaleFactorChanged {
                new_inner_size: &mut new_size,
                ..
            } => self.resize(Size {
                width: new_size.width,
                height: new_size.height,
            }),

            //--------------------------------------------------
            // Close pressed or requested
            WindowEvent::CloseRequested => self.proxy.send_event(RunnerLoopEvent::Exit).unwrap(),

            //--------------------------------------------------
            // Key pressed or released
            WindowEvent::KeyboardInput { input, .. } => self
                .world
                .run_with_data(core_systems::sys_manage_keyboard_input, input),

            //--------------------------------------------------
            // Mouse Input
            WindowEvent::MouseInput { state, button, .. } => self
                .world
                .run_with_data(core_systems::sys_manager_mouse_key_input, (state, button)),

            WindowEvent::CursorMoved { position, .. } => self
                .world
                .run_with_data(core_systems::sys_manager_mouse_position, position),

            WindowEvent::MouseWheel { delta, .. } => match delta {
                brackens_tools::winit::event::MouseScrollDelta::LineDelta(_, _) => todo!(),
                brackens_tools::winit::event::MouseScrollDelta::PixelDelta(_) => todo!(),
            },
            // WindowEvent::CursorEntered { .. } => {}
            // WindowEvent::CursorLeft { .. } => {}

            //--------------------------------------------------
            _ => {}
        }
    }

    fn device_input(&mut self, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .world
                .run_with_data(core_systems::sys_manage_mouse_movement, delta),
            _ => {}
        }
    }

    fn main_events_cleared(&mut self) {
        self.world
            .run(|window: UniqueView<core_components::Window>| {
                window.0.request_redraw();
            });
    }

    fn tick(&mut self) {
        self.pre_update();
        self.game_state.update(&mut self.world);
        self.post_update();

        if let Err(e) = renderer::systems::start_render_pass(&mut self.world) {
            match e {
                // brackens_tools::wgpu::SurfaceError::Timeout => todo!(),
                // brackens_tools::wgpu::SurfaceError::Outdated => todo!(),
                brackens_renderer::wgpu::SurfaceError::Lost => {
                    warn!("Warning: Surface has been lost. Attempting to resize:{}", e);
                    self.force_resize();
                }
                brackens_renderer::wgpu::SurfaceError::OutOfMemory => {
                    error!(
                        "Error: Surface has no available memory to create new frame: {}",
                        e
                    );
                    self.proxy.send_event(RunnerLoopEvent::Exit).unwrap();
                }
                _ => {}
            }
        } else {
            self.pre_render();
            self.game_state.render(&mut self.world);
            self.post_render();
        }

        self.end();
    }
}

impl<T> ShipyardCore<T>
where
    T: ShipyardGameState,
{
    fn resize(&mut self, new_size: Size<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.world.run(
                |mut size: UniqueViewMut<WindowSize>,
                 mut config: UniqueViewMut<SurfaceConfig>,
                 surface: UniqueView<Surface>,
                 device: UniqueView<Device>| {
                    size.0 = new_size;
                    config.0.width = new_size.width;
                    config.0.height = new_size.height;
                    surface.0.configure(&device.0, &config.0);
                },
            );

            // Resize everything here
            renderer::run_resize_systems(&mut self.world);
        }
    }
    fn force_resize(&mut self) {
        let size = self.world.run(|size: UniqueView<WindowSize>| size.0);
        self.resize(size);
    }

    fn pre_update(&mut self) {
        self.world.run(core_systems::sys_update_tracker);
        self.world.run(tool_systems::sys_tick_timers);
    }

    fn post_update(&mut self) {
        self.world
            .run_workload(spatial_systems::workload_update_tranforms)
            .unwrap();
        self.world.run(core_systems::sys_reset_input);

        self.world
            .run_workload(renderer::workload_post_update_systems)
            .unwrap();
    }

    fn pre_render(&mut self) {
        self.world.run(renderer::systems::sys_clear_background);
    }

    fn post_render(&mut self) {
        // Process Pipelines
        // Render pipelines

        renderer::run_post_render_systems(&mut self.world);
    }

    fn end(&mut self) {
        self.world
            .run_workload(tool_systems::wl_reset_asset_storage)
            .unwrap();
    }
}

impl<GS> Drop for ShipyardCore<GS>
where
    GS: ShipyardGameState,
{
    fn drop(&mut self) {
        self.world.run(|mut all_storages: AllStoragesViewMut| {
            // Clean up by deleting all components.
            // In some cases, some components dropped after uniques are dropped can
            // cause some errors e.g. handles
            all_storages.clear();
        });
    }
}

//===============================================================
//--------------------------------------------------
//________________________________________
