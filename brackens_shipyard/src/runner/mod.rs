//===============================================================

use std::hash::Hash;

use brackens_tools::{
    runner::RunnerDataCore, EventLoopProxy, Runner, RunnerCore, RunnerLoopEvent, WindowBuilder,
};
use shipyard::{
    Label, SystemModificator, UniqueView, UniqueViewMut, Workload, WorkloadModificator, World,
};

use crate::{
    assets::AssetsWorkload,
    renderer::RendererWorkload,
    tools::{ToolsWorkload, Window},
};

use self::{
    systems::{sys_clear_input_events, sys_remove_resize},
    uniques::{
        generate_device_event, generate_window_event, InputEventManager, MiscEventManager,
        ResizeEvent, RunnerErrorManager,
    },
};

pub mod systems;
pub mod uniques;

//===============================================================

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Stages {
    Start,
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
    Render,
    PostRender,
    End,
}

impl Label for Stages {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dyn_eq(&self, other: &dyn Label) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn dyn_hash(&self, mut state: &mut dyn std::hash::Hasher) {
        Self::hash(self, &mut state);
    }

    fn dyn_clone(&self) -> Box<dyn Label> {
        Box::new(self.clone())
    }

    fn dyn_debug(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

//===============================================================

// shipyard game state
pub trait RunnerWorkloads {
    #[allow(unused_variables)]
    fn pre_setup(&self, world: &World) {}
    fn setup(&self, world: &World);
    #[allow(unused_variables)]
    fn post_setup(&self, world: &World) {}

    fn start(&self) -> Workload {
        Workload::new("")
    }
    fn pre_update(&self) -> Workload {
        Workload::new("")
    }
    fn update(&self) -> Workload {
        Workload::new("")
    }
    fn post_update(&self) -> Workload {
        Workload::new("")
    }
    fn pre_render(&self) -> Workload {
        Workload::new("")
    }
    fn render(&self) -> Workload {
        Workload::new("")
    }
    fn post_render(&self) -> Workload {
        Workload::new("")
    }
    fn end(&self) -> Workload {
        Workload::new("")
    }
}

//===============================================================

#[derive(Default)]
pub struct ShipyardRunner {
    pub window_builder: WindowBuilder,
}

impl ShipyardRunner {
    pub fn run(self, core: Vec<Box<dyn RunnerWorkloads>>) {
        Runner::run_with_data::<Vec<Box<dyn RunnerWorkloads>>, ShipyardRunnerInner>(
            self.window_builder,
            core,
        );
    }

    pub fn run_all_plugins(self, mut core: Vec<Box<dyn RunnerWorkloads>>) {
        core.push(Box::new(ToolsWorkload));
        core.push(Box::new(AssetsWorkload));
        core.push(Box::new(RendererWorkload));

        Runner::run_with_data::<Vec<Box<dyn RunnerWorkloads>>, ShipyardRunnerInner>(
            self.window_builder,
            core,
        );
    }
}

//===============================================================

// shipyard core
struct ShipyardRunnerInner {
    world: World,
    proxy: EventLoopProxy<RunnerLoopEvent>,
}

impl RunnerDataCore<Vec<Box<dyn RunnerWorkloads>>> for ShipyardRunnerInner {
    fn new_data(
        window: brackens_tools::Window,
        event_loop: &brackens_tools::EventLoop<RunnerLoopEvent>,
        mut workloads: Vec<Box<dyn RunnerWorkloads>>,
    ) -> Self {
        //--------------------------------------------------

        let world = World::new();

        world.add_unique(Window::new(window));

        let proxy = event_loop.create_proxy();

        //--------------------------------------------------

        workloads.push(Box::new(ShipyardRunnerWorkloads));

        workloads
            .iter()
            .for_each(|workloads| workloads.pre_setup(&world));
        workloads
            .iter()
            .for_each(|workloads| workloads.setup(&world));
        workloads
            .iter()
            .for_each(|workloads| workloads.post_setup(&world));

        //--------------------------------------------------

        generate_workload(workloads).add_to_world(&world).unwrap();
        world.set_default_workload("MainWorkload").unwrap();

        //--------------------------------------------------

        Self { world, proxy }

        //--------------------------------------------------
    }
}

impl RunnerCore for ShipyardRunnerInner {
    fn new(
        window: brackens_tools::Window,
        event_loop: &brackens_tools::EventLoop<brackens_tools::RunnerLoopEvent>,
    ) -> Self {
        let workloads: Vec<Box<dyn RunnerWorkloads>> = vec![
            Box::new(ToolsWorkload),
            Box::new(AssetsWorkload),
            Box::new(RendererWorkload),
        ];

        Self::new_data(window, event_loop, workloads)
    }

    fn input(&mut self, event: brackens_tools::WindowEvent) {
        let event = generate_window_event(event);
        match event {
            uniques::WindowEventTypes::Resize(event) => {
                self.world.remove_unique::<ResizeEvent>().ok();
                self.world.add_unique(event);
            }
            uniques::WindowEventTypes::Misc(event) => self
                .world
                .run(|mut misc_events: UniqueViewMut<MiscEventManager>| misc_events.0.push(event)),
            uniques::WindowEventTypes::Input(event) => {
                self.world
                    .run(|mut input_events: UniqueViewMut<InputEventManager>| {
                        input_events.0.push(event)
                    })
            }
            uniques::WindowEventTypes::None => {}
        }
    }

    fn device_input(
        &mut self,
        device_id: brackens_tools::DeviceId,
        event: brackens_tools::DeviceEvent,
    ) {
        match generate_device_event(event, device_id) {
            uniques::WindowEventTypes::Resize(event) => {
                self.world.remove_unique::<ResizeEvent>().ok();
                self.world.add_unique(event);
            }
            uniques::WindowEventTypes::Misc(event) => self
                .world
                .run(|mut misc_events: UniqueViewMut<MiscEventManager>| misc_events.0.push(event)),
            uniques::WindowEventTypes::Input(event) => {
                self.world
                    .run(|mut input_events: UniqueViewMut<InputEventManager>| {
                        input_events.0.push(event)
                    })
            }
            uniques::WindowEventTypes::None => {}
        }
    }

    fn main_events_cleared(&mut self) {
        self.world
            .run(|window: UniqueView<Window>| window.request_redraw());
    }

    fn tick(&mut self) {
        self.world.run_default().unwrap();

        let mut error_manager = self
            .world
            .borrow::<UniqueViewMut<RunnerErrorManager>>()
            .unwrap();

        error_manager.drain().for_each(|e| match e {
            uniques::RunnerError::ForceResize => {
                let size = self.world.run(|window: UniqueView<Window>| window.size());

                if size.width > 0 && size.height > 0 {
                    self.world.remove_unique::<ResizeEvent>().ok();
                    self.world.add_unique(ResizeEvent::new(size));
                }
            }
        });

        let mut misc_events = self
            .world
            .borrow::<UniqueViewMut<MiscEventManager>>()
            .unwrap();
        misc_events.drain().for_each(|e| match e {
            uniques::MiscEvent::CloseRequested | uniques::MiscEvent::Destroyed => {
                self.proxy.send_event(RunnerLoopEvent::Exit).unwrap()
            }
            _ => {}
        })
    }
}

//===============================================================

pub fn generate_workload(workloads: Vec<Box<dyn RunnerWorkloads>>) -> Workload {
    let mut start = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.start())
        })
        .tag(Stages::Start);

    let mut pre_update = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.pre_update())
        })
        .tag(Stages::PreUpdate)
        .after_all(Stages::Start);

    let mut update = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.update())
        })
        .tag(Stages::Update)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate);

    let mut post_update = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.post_update())
        })
        .tag(Stages::PostUpdate)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update);

    let mut pre_render = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.pre_render())
        })
        .tag(Stages::PreRender)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate);

    let mut render = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.render())
        })
        .tag(Stages::Render)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender);

    let mut post_render = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.post_render())
        })
        .tag(Stages::PostRender)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender)
        .after_all(Stages::Render);

    let mut end = workloads
        .iter()
        .fold(Workload::new(""), |workload, stage| {
            workload.merge(&mut stage.end())
        })
        .tag(Stages::End)
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender)
        .after_all(Stages::Render)
        .after_all(Stages::PostRender);

    Workload::new("MainWorkload")
        .merge(&mut start)
        .merge(&mut pre_update)
        .merge(&mut update)
        .merge(&mut post_update)
        .merge(&mut pre_render)
        .merge(&mut render)
        .merge(&mut post_render)
        .merge(&mut end)
}

//===============================================================

struct ShipyardRunnerWorkloads;
impl RunnerWorkloads for ShipyardRunnerWorkloads {
    fn setup(&self, world: &World) {
        world.add_unique(RunnerErrorManager::default());
        world.add_unique(MiscEventManager::default());
        world.add_unique(InputEventManager::default());
    }

    fn end(&self) -> Workload {
        Workload::new("")
            .with_system(sys_remove_resize.skip_if_missing_unique::<ResizeEvent>())
            .with_system(sys_clear_input_events)
    }
}

//===============================================================
