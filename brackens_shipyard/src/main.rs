//===============================================================

use shipyard::{IntoWorkload, Label, Workload, WorkloadModificator, World};
use std::hash::Hash;

//===============================================================

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
enum Workloads {
    First,
    Second,
    Third,
}

impl Label for Workloads {
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

fn main() {
    let world = World::new();

    world.add_workload(workload_main);
    world.run_workload(workload_main).unwrap();
}

fn workload_main() -> Workload {
    (
        workload1, // workload2,
        workload3,
    )
        .into_workload()
}

fn workload1() -> Workload {
    Workload::new(Workloads::First)
        .with_system(system1)
        .with_system(system2)
        .after_all(Workloads::Third)
        .before_all(Workloads::Second)
}

fn workload3() -> Workload {
    Workload::new(Workloads::Third).with_system(system3)
}

//===============================================================

fn system1() {
    println!("A");
}
fn system2() {
    println!("B");
}
fn system3() {
    println!("C");
}

//===============================================================
