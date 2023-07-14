//===============================================================

use std::hash::Hash;

use brackens_tools::{
    runner::RunnerDataCore, EventLoopProxy, Runner, RunnerCore, RunnerLoopEvent, WindowBuilder,
};
use shipyard::{
    AllStoragesView, Label, SystemModificator, UniqueView, UniqueViewMut, Workload, World,
};

use crate::{
    assets::AssetsWorkload,
    hierarchies::HierarchyWorkload,
    renderer::{Renderer2dWorkload, RendererWorkload},
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
pub mod tools;
pub mod uniques;

//===============================================================

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SetupStages {
    Start,
    Main,
    End,
}
impl Label for SetupStages {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dyn_eq(&self, other: &dyn Label) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self == other;
        }
        return false;
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

//--------------------------------------------------

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
            return self == other;
        }
        return false;
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
    fn pre_setup(&self) -> Workload {
        Workload::new("")
    }
    fn setup(&self) -> Workload;
    fn post_setup(&self) -> Workload {
        Workload::new("")
    }

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

        add_setup_workloads(&world, &workloads);
        add_workloads(&world, &workloads);

        // println!("================================\n");

        // world.workloads_type_usage().0.iter().for_each(|(k, v)| {
        //     println!("Name: {}", k);
        //     v.iter().for_each(|(name, type_info)| {
        //         println!(" -- {}: ", name);
        //         type_info.iter().for_each(|val| {
        //             println!("     -- {:?}", val);
        //         });
        //     });
        //     println!("\n");
        // });

        // println!("\n================================\n");

        run_setup_workloads(&world);

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

fn add_setup_workloads(world: &World, workloads: &WorkloadGroup) {
    add_setup_workload_group(&world, &workloads, SetupStages::Start);
    add_setup_workload_group(&world, &workloads, SetupStages::Main);
    add_setup_workload_group(&world, &workloads, SetupStages::End);
}

fn add_setup_workload_group(world: &World, workloads: &WorkloadGroup, stage: SetupStages) {
    workloads
        .0
        .iter()
        .fold(Workload::new(stage), |master_workload, workload| {
            master_workload.merge(&mut match stage {
                SetupStages::Start => workload.pre_setup(),
                SetupStages::Main => workload.setup(),
                SetupStages::End => workload.post_setup(),
            })
        })
        .add_to_world(world)
        .unwrap();
}

fn run_setup_workloads(world: &World) {
    world.run_workload(SetupStages::Start).unwrap();
    world.run_workload(SetupStages::Main).unwrap();
    world.run_workload(SetupStages::End).unwrap();
}

//--------------------------------------------------

fn add_workloads(world: &World, workloads: &WorkloadGroup) {
    add_workload_group(&world, &workloads, Stages::Start);

    add_workload_group(&world, &workloads, Stages::PreUpdate);
    add_workload_group(&world, &workloads, Stages::Update);
    add_workload_group(&world, &workloads, Stages::PostUpdate);

    add_workload_group(&world, &workloads, Stages::PreRender);
    add_workload_group(&world, &workloads, Stages::Render);
    add_workload_group(&world, &workloads, Stages::PostRender);

    add_workload_group(&world, &workloads, Stages::End);
}

fn add_workload_group(world: &World, workloads: &WorkloadGroup, stage: Stages) {
    workloads
        .0
        .iter()
        .fold(Workload::new(stage), |master_workload, workload| {
            master_workload.merge(&mut match stage {
                Stages::Start => workload.start(),
                Stages::PreUpdate => workload.pre_update(),
                Stages::Update => workload.update(),
                Stages::PostUpdate => workload.post_update(),
                Stages::PreRender => workload.pre_render(),
                Stages::Render => workload.render(),
                Stages::PostRender => workload.post_render(),
                Stages::End => workload.end(),
            })
        })
        .add_to_world(world)
        .unwrap();
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

//===============================================================

pub struct WorkloadGroup(pub(crate) Vec<Box<dyn RunnerWorkloads>>);
impl WorkloadGroup {
    pub fn default_workloads() -> Self {
        Self(vec![
            Box::new(ToolsWorkload),
            Box::new(AssetsWorkload),
            Box::new(RendererWorkload),
            Box::new(HierarchyWorkload),
            Box::new(Renderer2dWorkload),
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
    fn setup(&self) -> Workload {
        Workload::new("").with_system(|storages: AllStoragesView| {
            storages.add_unique(RunnerErrorManager::default());
            storages.add_unique(MiscEventManager::default());
            storages.add_unique(InputEventManager::default());
        })
    }

    fn end(&self) -> Workload {
        Workload::new("")
            .with_system(sys_remove_resize.skip_if_missing_unique::<ResizeEvent>())
            .with_system(sys_clear_input_events)
    }
}

//===============================================================
