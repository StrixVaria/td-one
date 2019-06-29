use crate::components::*;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct TargetingSystem;

impl<'a> System<'a> for TargetingSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TargetLocation>,
        ReadStorage<'a, TargetEntity>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (entities, mut target_location, target_entity, position): Self::SystemData) {
        for (entity, target_entity) in (&entities, &target_entity).join() {
            if let Some(target_position) = position.get(target_entity.entity) {
                target_location
                    .insert(
                        entity,
                        TargetLocation {
                            position: *target_position,
                        },
                    )
                    .unwrap();
            }
        }
    }
}
