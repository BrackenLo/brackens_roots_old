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
    pub fn run(self, core: WorkloadGroup) {
        Runner::run_with_data::<WorkloadGroup, ShipyardRunnerInner>(self.window_builder, core);
    }

    pub fn run_all_plugins(self, plugins: WorkloadGroup) {
        let mut core = WorkloadGroup::default_workloads();
        core.add_workload_group(plugins);

        Runner::run_with_data::<WorkloadGroup, ShipyardRunnerInner>(self.window_builder, core);
    }
}

//===============================================================

// shipyard core
struct ShipyardRunnerInner {
    world: World,
    proxy: EventLoopProxy<RunnerLoopEvent>,
}

impl RunnerDataCore<WorkloadGroup> for ShipyardRunnerInner {
    fn new_data(
        window: brackens_tools::Window,
        event_loop: &brackens_tools::EventLoop<RunnerLoopEvent>,
        mut workloads: WorkloadGroup,
    ) -> Self {
        //--------------------------------------------------

        let world = World::new();

        world.add_unique(Window::new(window));

        let proxy = event_loop.create_proxy();

        //--------------------------------------------------

        workloads.add_workload(Box::new(ShipyardRunnerWorkloads));

        workloads
            .0
            .iter()
            .for_each(|workloads| workloads.pre_setup(&world));
        workloads
            .0
            .iter()
            .for_each(|workloads| workloads.setup(&world));
        workloads
            .0
            .iter()
            .for_each(|workloads| workloads.post_setup(&world));

        //--------------------------------------------------

        // generate_workload(workloads).add_to_world(&world).unwrap();
        // let workload = generate_workload(workloads);
        // world.add_workload(|| workload);

        // world.add_workload(|| generate_workload(workloads));

        add_workloads(&world, workloads);

        println!("================================\n");

        // println!("workloads: {:?}", world.workloads_type_usage());

        world.workloads_type_usage().0.iter().for_each(|(k, v)| {
            println!("Name: {}", k);
            v.iter().for_each(|(name, type_info)| {
                println!(" -- {}: ", name);
                type_info.iter().for_each(|val| {
                    println!("     -- {:?}", val);
                });
            });
            println!("\n");
        });

        println!("\n================================\n");

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
        let workloads = WorkloadGroup::default_workloads();
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
        // self.world.run_default().unwrap();
        run_workloads(&self.world);

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

fn add_workloads(world: &World, workloads: WorkloadGroup) {
    start_workloads(&workloads).add_to_world(world).unwrap();

    pre_update_workloads(&workloads)
        .add_to_world(world)
        .unwrap();
    update_workloads(&workloads).add_to_world(world).unwrap();
    post_update_workloads(&workloads)
        .add_to_world(world)
        .unwrap();

    pre_render_workloads(&workloads)
        .add_to_world(world)
        .unwrap();
    render_workloads(&workloads).add_to_world(world).unwrap();
    post_render_workloads(&workloads)
        .add_to_world(world)
        .unwrap();

    end_workloads(&workloads).add_to_world(world).unwrap();
}

fn run_workloads(world: &World) {
    world.run_workload(Stages::Start).unwrap();

    world.run_workload(Stages::PreUpdate).unwrap();
    world.run_workload(Stages::Update).unwrap();
    world.run_workload(Stages::PostUpdate).unwrap();

    world.run_workload(Stages::PreRender).unwrap();
    world.run_workload(Stages::Render).unwrap();
    world.run_workload(Stages::PostRender).unwrap();

    world.run_workload(Stages::End).unwrap();
}

fn start_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::Start), |workload, stage| {
            workload.merge(&mut stage.start())
        })
}

fn pre_update_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::PreUpdate), |workload, stage| {
            workload.merge(&mut stage.pre_update())
        })
        .after_all(Stages::Start)
}

fn update_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::Update), |workload, stage| {
            workload.merge(&mut stage.update())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
}

fn post_update_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::PostUpdate), |workload, stage| {
            workload.merge(&mut stage.post_update())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
}

fn pre_render_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::PreRender), |workload, stage| {
            workload.merge(&mut stage.pre_render())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
}

fn render_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::Render), |workload, stage| {
            workload.merge(&mut stage.render())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender)
}

fn post_render_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::PostRender), |workload, stage| {
            workload.merge(&mut stage.post_render())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender)
        .after_all(Stages::Render)
}

fn end_workloads(workloads: &WorkloadGroup) -> Workload {
    workloads
        .0
        .iter()
        .fold(Workload::new(Stages::End), |workload, stage| {
            workload.merge(&mut stage.end())
        })
        .after_all(Stages::Start)
        .after_all(Stages::PreUpdate)
        .after_all(Stages::Update)
        .after_all(Stages::PostUpdate)
        .after_all(Stages::PreRender)
        .after_all(Stages::Render)
        .after_all(Stages::PostRender)
}

//===============================================================

pub struct WorkloadGroup(pub(crate) Vec<Box<dyn RunnerWorkloads>>);
impl WorkloadGroup {
    pub fn default_workloads() -> Self {
        Self(vec![
            Box::new(ToolsWorkload),
            Box::new(AssetsWorkload),
            Box::new(RendererWorkload),
        ])
    }

    pub fn with_workload(workload: Box<dyn RunnerWorkloads>) -> Self {
        Self(vec![workload])
    }
    pub fn with_workloads(workloads: Vec<Box<dyn RunnerWorkloads>>) -> Self {
        Self(workloads)
    }

    pub fn add_workload(&mut self, workload: Box<dyn RunnerWorkloads>) {
        self.0.push(workload);
    }
    pub fn add_workload_group(&mut self, workload_group: WorkloadGroup) {
        workload_group
            .0
            .into_iter()
            .for_each(|val| self.0.push(val));
    }
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
