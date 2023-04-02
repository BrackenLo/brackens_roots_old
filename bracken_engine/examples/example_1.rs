//===============================================================

use bracken_engine::shipyard_core::{ShipyardGameState, ShipyardRunner};

//===============================================================

fn main() {
    ShipyardRunner::default().run::<Game>();
}

//===============================================================

struct Game;
impl ShipyardGameState for Game {
    fn new(world: &mut shipyard::World) -> Self {
        Self
    }

    fn update(&mut self, world: &mut shipyard::World) {
        todo!()
    }

    fn render(&mut self, world: &mut shipyard::World) {
        todo!()
    }
}

//===============================================================
