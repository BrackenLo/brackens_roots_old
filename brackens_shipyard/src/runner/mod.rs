//===============================================================

use std::{hash::Hash, marker::PhantomData};

use brackens_renderer::Size;
use brackens_tools::{EventLoopProxy, RunnerCore, RunnerLoopEvent, WindowEvent};
use shipyard::{
    Label, SystemModificator, UniqueView, UniqueViewMut, Workload, WorkloadModificator, World,
};

use crate::{
    assets::{setup_assets, sys_reset_asset_storage},
    renderer::{setup_renderer, sys_clear_background, sys_start_render_pass, RenderPassTools},
    tools::{
        input_manage_device_event, input_manage_window_event, setup_tools, sys_reset_input_manager,
        sys_tick_timers, sys_update_upkeep, Window,
    },
};

use self::{
    systems::sys_remove_resize,
    uniques::{ResizeEvent, RunnerErrorManager},
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
pub trait ShipyardCore {
    fn new(world: &World);

    fn start() -> Workload;
    fn pre_update() -> Workload;
    fn update() -> Workload;
    fn post_update() -> Workload;
    fn pre_render() -> Workload;
    fn render() -> Workload;
    fn post_render() -> Workload;
    fn end() -> Workload;
}

// shipyard core
pub struct ShipyardRunner<Core: ShipyardCore> {
    phantom: PhantomData<Core>,
    world: World,
    _proxy: EventLoopProxy<RunnerLoopEvent>,
}

impl<Core: ShipyardCore> RunnerCore for ShipyardRunner<Core> {
    fn new(
        window: brackens_tools::Window,
        event_loop: &brackens_tools::EventLoop<brackens_tools::RunnerLoopEvent>,
    ) -> Self {
        //--------------------------------------------------

        let world = World::new();

        world.add_unique(Window::new(window));

        let proxy = event_loop.create_proxy();

        //--------------------------------------------------

        ShipyardRunnerInner::new(&world);
        Core::new(&world);

        world.add_workload(|| Self::workload_main());

        //--------------------------------------------------

        Self {
            world,
            _proxy: proxy,
            phantom: PhantomData,
        }

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
                // self.core.input(&self.world, event);
                // todo!()
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
        self.world.run_workload(|| Self::workload_main()).unwrap();

        let mut manager = self
            .world
            .borrow::<UniqueViewMut<RunnerErrorManager>>()
            .unwrap();

        manager.drain().for_each(|e| match e {
            uniques::RunnerError::ForceResize => {
                let size = self.world.run(|window: UniqueView<Window>| window.size());
                self.world.run_with_data(systems::resize, size);
            }
        });
    }
}

impl<Core: ShipyardCore> ShipyardRunner<Core> {
    pub fn resize(&mut self, new_size: Size<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.world.run_with_data(systems::resize, new_size);
    }

    fn workload_main() -> Workload {
        Workload::new("")
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::start())
                    .merge(&mut Core::start())
                    .tag(Stages::Start),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::pre_update())
                    .merge(&mut Core::pre_update())
                    .tag(Stages::PreUpdate)
                    .after_all(Stages::Start),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::update())
                    .merge(&mut Core::update())
                    .tag(Stages::Update)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::post_update())
                    .merge(&mut Core::post_update())
                    .tag(Stages::PostUpdate)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate)
                    .after_all(Stages::Update),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::pre_render())
                    .merge(&mut Core::pre_render())
                    .tag(Stages::PreRender)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate)
                    .after_all(Stages::Update)
                    .after_all(Stages::PostUpdate),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::render())
                    .merge(&mut Core::render())
                    .tag(Stages::Render)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate)
                    .after_all(Stages::Update)
                    .after_all(Stages::PostUpdate)
                    .after_all(Stages::PreRender),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::post_render())
                    .merge(&mut Core::post_render())
                    .tag(Stages::PostRender)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate)
                    .after_all(Stages::Update)
                    .after_all(Stages::PostUpdate)
                    .after_all(Stages::PreRender)
                    .after_all(Stages::Render),
            )
            .merge(
                &mut Workload::new("")
                    .merge(&mut ShipyardRunnerInner::end())
                    .merge(&mut Core::end())
                    .tag(Stages::End)
                    .after_all(Stages::Start)
                    .after_all(Stages::PreUpdate)
                    .after_all(Stages::Update)
                    .after_all(Stages::PostUpdate)
                    .after_all(Stages::PreRender)
                    .after_all(Stages::Render)
                    .after_all(Stages::PostRender),
            )
    }
}

//===============================================================

//===============================================================

pub struct ShipyardRunnerInner;
impl ShipyardCore for ShipyardRunnerInner {
    fn new(world: &World) {
        world.run(setup_assets);
        world.run(setup_renderer);
        world.run(setup_tools);
        world.add_unique(RunnerErrorManager::default());
    }

    fn start() -> Workload {
        Workload::new("").with_system(sys_update_upkeep)
    }

    fn pre_update() -> Workload {
        Workload::new("").with_system(sys_tick_timers)
    }

    fn update() -> Workload {
        Workload::new("")
    }

    fn post_update() -> Workload {
        Workload::new("").with_system(sys_reset_input_manager)
    }

    fn pre_render() -> Workload {
        Workload::new("").with_system(sys_start_render_pass).merge(
            &mut Workload::new("")
                .skip_if_missing_unique::<RenderPassTools>()
                .after_all(sys_start_render_pass)
                .with_system(sys_clear_background),
        )
    }

    fn render() -> Workload {
        Workload::new("").skip_if_missing_unique::<RenderPassTools>()
    }

    fn post_render() -> Workload {
        Workload::new("")
            .with_system(sys_start_render_pass.skip_if_missing_unique::<RenderPassTools>())
    }

    fn end() -> Workload {
        Workload::new("")
            .with_system(sys_remove_resize.skip_if_missing_unique::<ResizeEvent>())
            .with_system(sys_reset_asset_storage)
    }
}

//===============================================================
