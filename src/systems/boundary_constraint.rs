use specs::{Join, ReadExpect, System, WriteStorage};

use crate::{components::Position, map::Map};

pub struct BoundaryConstraintSystem;

impl<'a> System<'a> for BoundaryConstraintSystem {
    type SystemData = (ReadExpect<'a, Map>, WriteStorage<'a, Position>);

    fn run(&mut self, (map, mut position): Self::SystemData) {
        let bounds = map.get_bounds();
        for pos in (&mut position).join() {
            *pos = bounds.constrain(*pos);
        }
    }
}
