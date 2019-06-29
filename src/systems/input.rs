use specs::{System, Write, ReadExpect};

use crate::{
    input::{Input, MousePressedState, MouseScrollDir},
    offset::WorldOffset,
    EntityTracker, EntityRef,
    qt::{QuadTree, Region},
};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadExpect<'a, QuadTree<EntityRef>>, Write<'a, Input>, Write<'a, EntityTracker>, Write<'a, WorldOffset>);

    fn run(&mut self, (qt, mut input, mut entity_tracker, mut offset): Self::SystemData) {
        match &input.mouse_scroll_dir {
            MouseScrollDir::None => (),
            dir => offset.zoom(dir.into(), input.mouse_x, input.mouse_y),
        }
        input.reset_scroll();

        match &input.mouse_pressed {
            MousePressedState::JustReleased => {
                if input.barely_moved_from_down() {
                    println!("Clicked at ({}, {})", input.mouse_x, input.mouse_y);
                }
                input.mouse_up();
            }
            MousePressedState::Down => {
                let (dx, dy) = input.mouse_pos_diff();
                offset.slide(-dx, -dy);
            }
            _ => (),
        }

        let (mouse_x, mouse_y) = offset.to_local_pixel(input.mouse_x, input.mouse_y);
        let results = qt.query(&Region::new_point(mouse_x, mouse_y));
        entity_tracker.hovered = if !results.is_empty() {
            Some(results[0].entity)
        } else {
            None
        }
    }
}
