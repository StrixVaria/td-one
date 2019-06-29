use graphics::{clear, color, ellipse, rectangle, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use specs::{Join, Read, ReadExpect, ReadStorage, System};
use viewport::Viewport;

use crate::{components::*, input::Input, map::Map, offset::WorldOffset};

pub struct RenderSystem<'m> {
    pub gl: GlGraphics,
    pub gc: GlyphCache<'m>,
}

impl<'a, 'm> System<'a> for RenderSystem<'m> {
    type SystemData = (
        ReadExpect<'a, Viewport>,
        ReadExpect<'a, Map>,
        Read<'a, WorldOffset>,
        Read<'a, Input>,
        ReadStorage<'a, Body>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TargetLocation>,
    );

    fn run(&mut self, (viewport, map, offset, input, body, position, target): Self::SystemData) {
        let ref mut gc = self.gc;
        self.gl.draw(*viewport, |c, g| {
            let transform = c
                .transform
                .trans(offset.h, offset.v)
                .scale(offset.scale, offset.scale);
            let (mouse_x, mouse_y) = offset.to_local_pixel(input.mouse_x, input.mouse_y);
            clear(color::hex("000000"), g);
            map.render(transform, g, mouse_x, mouse_y);
            for (body, position) in (&body, &position).join() {
                match body.body_shape {
                    BodyShape::Circle => {
                        ellipse(
                            body.color,
                            rectangle::centered_square(position.x, position.y, body.size),
                            transform,
                            g,
                        );
                    }
                    BodyShape::Square => {
                        rectangle(
                            body.color,
                            rectangle::centered_square(position.x, position.y, body.size),
                            transform,
                            g,
                        );
                    }
                }
            }
            for (position, target) in (&position, &target).join() {
                graphics::line(
                    [1.0, 0.0, 0.0, 1.0],
                    1.0,
                    [position.x, position.y, target.position.x, target.position.y],
                    transform,
                    g,
                );
            }
            // TODO: Replace with rendering code for a real UI.
            graphics::text(
                color::hex("ffffff"),
                20,
                "test text",
                gc,
                c.transform.trans(100.0, 100.0),
                g,
            )
            .unwrap();
        })
    }
}
