//===============================================================

use brackens_engine::{
    core_components::KeyManager,
    renderer::{
        components::Visible,
        components_2d::Texture,
        tools_2d::{load_texture, LoadTextureDescriptor},
    },
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
        let texture = world.run_with_data(
            load_texture,
            LoadTextureDescriptor {
                label: "Boss Face",
                path: "res/bossFace.png",
                sampler: None,
            },
        );

        world.add_entity((
            Visible { visible: true },
            Transform::from_translation(Vec3::new(0., 0., 99.)),
            GlobalTransform::default(),
            Texture {
                size: Vec2::new(32., 32.),
                handle: texture.clone(),
                color: [1., 0., 1., 1.],
            },
            Movable(5.),
        ));

        world.add_entity((
            Visible { visible: true },
            Transform::from_translation(Vec3::new(100., 100., 0.)),
            GlobalTransform::default(),
            Texture {
                size: Vec2::new(64., 64.),
                handle: texture,
                color: [1., 0., 0., 1.],
            },
        ));

        Self
    }

    fn update(&mut self, world: &mut shipyard::World) {
        world.run(
            |keys: UniqueView<KeyManager>,
             mut transforms: ViewMut<Transform>,
             v_movable: View<Movable>| {
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

                for (mut transform, movable) in (&mut transforms, &v_movable).iter() {
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
