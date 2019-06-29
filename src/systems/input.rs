use specs::{System, Write};

use crate::{
    input::{Input, MousePressedState, MouseScrollDir},
    offset::WorldOffset,
};

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (Write<'a, Input>, Write<'a, WorldOffset>);

    fn run(&mut self, (mut input, mut offset): Self::SystemData) {
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
    }
}
