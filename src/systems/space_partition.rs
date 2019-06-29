use specs::{System, WriteExpect, ReadStorage, ReadExpect, Entities, Join};

use crate::{components::{Position, Body, BodyShape}, EntityRef, qt::{QuadTree, RectangleData, Region}, map::Map};

pub struct SpacePartitionSystem;

impl<'a> System<'a> for SpacePartitionSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, QuadTree<EntityRef>>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Body>,
    );

    fn run(&mut self, (entities, mut qt, map, positions, bodies): Self::SystemData) {
        let map_bounds = map.get_bounds();
        let map_bounds = RectangleData::new(map_bounds.x, map_bounds.y, map_bounds.w, map_bounds.h);
        let mut new_qt: QuadTree<EntityRef> = QuadTree::new(map_bounds);
        for (entity, pos, body) in (&entities, &positions, &bodies).join() {
            new_qt.insert(EntityRef { entity, region: get_region(pos, body)});
        }
        *qt = new_qt;
    }
}

fn get_region(pos: &Position, body: &Body) -> Region {
    match body.body_shape {
        BodyShape::Circle => {
            Region::new_circle(pos.x, pos.y, body.size)
        },
        BodyShape::Square => {
            let size = body.size * 2.0;
            Region::new_rect(pos.x, pos.y, size, size)
        }
    }
}