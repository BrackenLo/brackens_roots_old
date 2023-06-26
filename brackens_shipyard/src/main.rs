//===============================================================

use shipyard::{Workload, WorkloadModificator, World};

//===============================================================

pub fn main() {
    let world = World::new();

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
                .after_all("Mid")
                .after_all("Start"),
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
    Workload::new("").with_system(sys_3)
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
fn sys_3() {
    println!("3")
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
