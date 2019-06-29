use crate::{components::*, math, systems::DeltaTime};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Mobility>,
        ReadStorage<'a, TargetLocation>,
    );

    fn run(&mut self, (dt, mut position, mobility, target_location): Self::SystemData) {
        use Mobility::*;
        let dt = dt.0;
        for (position, mobility, target) in (&mut position, &mobility, &target_location).join() {
            match mobility {
                Omnidirectional { speed } => {
                    *position += math::get_unit_dir(*position, target.position) * speed * dt;
                }
            }
        }
    }
}
