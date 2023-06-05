//===============================================================

use brackens_engine::{
    core_components::KeyManager,
    prelude::{Texture, Vec3},
    renderer::{
        components::Camera,
        components_2d::TextureBundleViewMut,
        tools_2d::{load_blank_texture, BlankTextureDescriptor},
    },
    spatial_components::{GlobalTransform, Transform},
    tool_components::{Activated, AutoUpdate},
    KeyCode, ShipyardGameState, ShipyardRunner,
};
use shipyard::{Component, EntitiesViewMut, IntoIter, UniqueView, View, ViewMut};

//===============================================================

fn main() {
    ShipyardRunner::default().run::<Game>();
}

//===============================================================

struct Game;
impl ShipyardGameState for Game {
    fn new(world: &mut shipyard::World) -> Self {
        let texture = world.run_with_data(load_blank_texture, BlankTextureDescriptor::default());

        world.run(
            |mut entities: EntitiesViewMut, mut vm_texture_bundle: TextureBundleViewMut| {
                vm_texture_bundle.new_texture(
                    &mut entities,
                    Transform::default(),
                    Texture::new_color(texture, 32., 32., [1., 0., 0., 1.]),
                );
            },
        );

        world.add_entity((
            Transform::default(),
            GlobalTransform::default(),
            Camera::new_orthographic(-300., 300., -200., 200., 0., 100.),
            AutoUpdate,
            Activated,
            Movable(5.),
        ));

        Self
    }

    fn update(&mut self, world: &mut shipyard::World) {
        world.run(
            |keys: UniqueView<KeyManager>,
             mut vm_transforms: ViewMut<Transform>,
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

                if dir.length() == 0. {
                    return;
                }

                for (mut transform, movable) in (&mut vm_transforms, &v_movable).iter() {
                    *transform.translation_mut() += dir * movable.0;
                }
            },
        );
    }
}

#[derive(Component)]
struct Movable(f32);

//===============================================================
