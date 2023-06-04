//===============================================================

use brackens_engine::{prelude::*, renderer::tools_2d::load_blank_texture};

//===============================================================

fn main() {
    ShipyardRunner::default().run::<Game>();
}

//===============================================================

struct Game;
impl ShipyardGameState for Game {
    fn new(world: &mut shipyard::World) -> Self {
        let texture1 = load_texture(world, "res/bossFace.png", "BossFace", None);
        let texture2 = load_blank_texture(world, "BlankTexture", [1., 1., 1.], None);

        let parent = world.add_entity((
            GlobalTransform::default(),
            Transform::default(),
            Visible::default(),
            Texture {
                size: Vec2::new(80., 80.),
                handle: texture1.clone(),
                color: [1., 1., 1., 1.],
            },
            Center,
        ));

        let child = world.add_entity((
            GlobalTransform::default(),
            Transform::default(),
            Visible::default(),
            Texture {
                size: Vec2::new(30., 30.),
                handle: texture2.clone(),
                color: [1., 0., 0., 1.],
            },
            Spin(103.),
            Progress(0., 1.),
            UseParentTransform,
        ));

        let child2 = world.add_entity((
            GlobalTransform::default(),
            Transform::default(),
            Visible::default(),
            Texture {
                size: Vec2::new(28., 28.),
                handle: texture2.clone(),
                color: [1., 1., 1., 1.],
            },
            Spin(150.),
            Progress(0., 1.2),
            UseParentTransform,
        ));

        let child3 = world.add_entity((
            GlobalTransform::default(),
            Transform::default(),
            Visible::default(),
            Texture {
                size: Vec2::new(50., 50.),
                handle: texture2.clone(),
                color: [1., 1., 1., 1.],
            },
            Spin(250.),
            Progress(0., 0.8),
            UseParentTransform,
        ));

        let child4 = world.add_entity((
            GlobalTransform::default(),
            Transform::default(),
            Visible::default(),
            Texture {
                size: Vec2::new(30., 30.),
                handle: texture2.clone(),
                color: [1., 1., 1., 1.],
            },
            Spin(80.),
            Progress(0., 2.3),
            UseParentTransform,
        ));

        {
            let mut hierarchy = world.borrow::<HierarchyBundle>().unwrap();
            hierarchy.attach(parent, child);
            hierarchy.attach(parent, child2);
            hierarchy.attach(parent, child3);
            hierarchy.attach(child3, child4);
        }

        Self
    }

    fn update(&mut self, world: &mut shipyard::World) {
        world.run(sys_center);
        world.run(sys_progress);
        world.run(sys_move);
        world.run(sys_spin);
    }
}

//===============================================================

struct Center;
impl Component for Center {
    type Tracking = shipyard::track::Untracked;
}

struct Progress(f32, f32);
impl Component for Progress {
    type Tracking = shipyard::track::Untracked;
}

struct Move;
impl Component for Move {
    type Tracking = shipyard::track::Untracked;
}

struct Spin(f32);
impl Component for Spin {
    type Tracking = shipyard::track::Untracked;
}

//===============================================================

fn sys_center(
    screen: UniqueView<WindowSize>,
    centers: View<Center>,
    mut transforms: ViewMut<Transform>,
) {
    for (_, mut transform) in (&centers, &mut transforms).iter() {
        *transform.translation() =
            Vec3::new(screen.width() as f32 / 2., screen.height() as f32 / 2., 0.);
    }
}

fn sys_progress(tracker: UniqueView<UpkeepTracker>, mut progresses: ViewMut<Progress>) {
    let delta = tracker.delta();
    for mut progress in (&mut progresses).iter() {
        progress.0 += delta * progress.1;
    }
}

fn sys_move(
    screen: UniqueView<WindowSize>,
    moves: View<Move>,
    progresses: View<Progress>,
    mut transforms: ViewMut<Transform>,
) {
    for (_, progress, mut transform) in (&moves, &progresses, &mut transforms).iter() {
        let half_size = Vec2::new(screen.width() as f32 / 2., screen.height() as f32 / 2.);

        *transform.translation() = Vec3::new(
            half_size.x * progress.0.sin() * 0.7 + half_size.x,
            half_size.y * progress.0.cos() * 0.7 + half_size.y,
            0.,
        );
    }
}

fn sys_spin(spins: View<Spin>, progresses: View<Progress>, mut transforms: ViewMut<Transform>) {
    for (spin, progress, mut transform) in (&spins, &progresses, &mut transforms).iter() {
        *transform.translation() =
            Vec3::new(progress.0.sin() * spin.0, progress.0.cos() * spin.0, 0.);
    }
}

//===============================================================
