//===============================================================

use bracken_engine::shipyard_core::{ShipyardGameState, ShipyardRunner};

//===============================================================

fn main() {
    ShipyardRunner::default().run::<Game>();
}

//===============================================================

struct Game;
impl ShipyardGameState for Game {
    fn new(_world: &mut shipyard::World) -> Self {
        Self
    }

    fn update(&mut self, _world: &mut shipyard::World) {}
}

//===============================================================
