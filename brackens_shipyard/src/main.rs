//===============================================================

use shipyard::{Component, EntitiesViewMut, IntoIter, IntoWithId, View, ViewMut, Workload, World};

//===============================================================

pub fn main() {
    let world = World::new();

    world.add_workload(workload_1);
    world.add_workload(workload_2);

    world.run_workload(workload_1).unwrap();
    world.run_workload(workload_2).unwrap();
}

//===============================================================

fn workload_1() -> Workload {
    Workload::new("").with_system(sys_1)
}

fn workload_2() -> Workload {
    Workload::new("").with_system(sys_2)
}

pub fn sys_1(mut entities: EntitiesViewMut, mut view: ViewMut<MyVal>) {
    entities.add_entity(&mut view, MyVal(5));
}

pub fn sys_2(view: View<MyVal>) {
    view.inserted_or_modified()
        .iter()
        .with_id()
        .for_each(|(id, _)| {
            println!("id: {:?}", id);
        });
}

//===============================================================

#[derive(Component)]
#[track(All)]
pub struct MyVal(pub u32);

//===============================================================
