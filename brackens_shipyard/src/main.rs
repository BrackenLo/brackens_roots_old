//===============================================================

use std::{error::Error, fmt::Display};

use shipyard::{Unique, UniqueView, Workload, WorkloadModificator, World};

//===============================================================

#[derive(Unique)]
pub struct TestUnique(u32);

pub fn main() {
    let world = World::new();

    world.add_unique(TestUnique(45));
    world.remove_unique::<TestUnique>().ok();
    world.add_unique(TestUnique(22));

    let val = world.borrow::<UniqueView<TestUnique>>().unwrap();
    println!("val = {}", val.0);

    world.add_workload(master_workload);
    world.run_workload(master_workload).unwrap();
    println!("============");
    world.run_workload(master_workload).unwrap();
    println!("============");
    world.run_workload(master_workload).unwrap();
    println!("============");
}

//===============================================================

fn master_workload() -> Workload {
    Workload::new("")
        .merge(
            &mut Workload::new("")
                .merge(&mut start_phase_1())
                .merge(&mut start_phase_2())
                .tag("Start"),
        )
        .merge(
            &mut Workload::new("")
                .merge(&mut mid_phase_1())
                .merge(&mut mid_phase_2())
                .tag("Mid")
                .after_all("Start"),
        )
        .merge(
            &mut Workload::new("")
                .merge(&mut end_phase_1())
                .merge(&mut end_phase_2())
                .tag("End")
                .after_all("Start")
                .after_all("Mid"),
        )
}

//===============================================================

fn start_phase_1() -> Workload {
    Workload::new("").with_system(sys_1)
}

fn start_phase_2() -> Workload {
    Workload::new("").with_system(sys_2)
}

fn mid_phase_1() -> Workload {
    Workload::new("").with_try_system(sys_3)
}

fn mid_phase_2() -> Workload {
    Workload::new("").with_system(sys_4)
}

fn end_phase_1() -> Workload {
    Workload::new("").with_system(sys_5)
}

fn end_phase_2() -> Workload {
    Workload::new("").with_system(sys_6)
}

//===============================================================

fn sys_1() {
    println!("1")
}
fn sys_2() {
    println!("2")
}
fn sys_3() -> Result<(), MyError> {
    println!("3");
    Err(MyError)
}
fn sys_4() {
    println!("4")
}
fn sys_5() {
    println!("5")
}
fn sys_6() {
    println!("6")
}

//===============================================================

#[derive(Debug)]
struct MyError;
impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}
