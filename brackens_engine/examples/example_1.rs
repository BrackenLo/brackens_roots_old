//===============================================================

use brackens_engine::{
    core_components::KeyManager,
    load_texture,
    renderer::components::{Texture, Visible},
    spatial_components::{GlobalTransform, Transform},
    KeyCode, ShipyardGameState, ShipyardRunner,
};
use brackens_tools::glam::{Vec2, Vec3};
use shipyard::{Component, IntoIter, UniqueView, View, ViewMut};

//===============================================================

fn main() {
    ShipyardRunner::default().run::<Game>();
}

//===============================================================

struct Game;
impl ShipyardGameState for Game {
    fn new(world: &mut shipyard::World) -> Self {
        let texture = load_texture(world, "res/bossFace.png", "BossFace", None);

        world.add_entity((
            Visible { visible: true },
            Transform::default(),
            GlobalTransform::default(),
            Texture {
                size: Vec2::new(32., 32.),
                handle: texture,
                color: [1., 0., 1., 1.],
            },
            Movable(5.),
        ));

        Self
    }

    fn update(&mut self, world: &mut shipyard::World) {
        world.run(
            |keys: UniqueView<KeyManager>,
             mut transforms: ViewMut<Transform>,
             movables: View<Movable>| {
                let mut dir = Vec3::ZERO;
                if keys.pressed(KeyCode::A) {
                    dir.x -= 1.;
                }
                if keys.pressed(KeyCode::D) {
                    dir.x += 1.;
                }
                if keys.pressed(KeyCode::W) {
                    dir.y += 1.;
                }
                if keys.pressed(KeyCode::S) {
                    dir.y -= 1.;
                }

                for (mut transform, movable) in (&mut transforms, &movables).iter() {
                    *transform.translation() += dir * movable.0;
                }
            },
        );
    }
}

//===============================================================

#[derive(Component)]
struct Movable(f32);

//===============================================================
