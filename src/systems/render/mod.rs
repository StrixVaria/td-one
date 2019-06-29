use graphics::{clear, color, ellipse, line, math::Matrix2d, rectangle, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use specs::{Join, Read, ReadExpect, ReadStorage, System};
use viewport::Viewport;

use crate::{components::*, input::Input, map::Map, offset::WorldOffset, EntityTracker};

mod anchor;
use anchor::Anchor;

mod panel;
use panel::Panel;

pub struct RenderSystem<'m> {
    pub gl: GlGraphics,
    pub gc: GlyphCache<'m>,
    hovered_panel: Panel,
}

impl<'m> RenderSystem<'m> {
    /// Initial screen width/height required for UI positioning.
    pub fn new(gl: GlGraphics, gc: GlyphCache<'m>, w: f64, h: f64) -> Self {
        Self {
            gl,
            gc,
            hovered_panel: Panel::new(w, h, w / 2.0, 100.0).with_anchor(Anchor::BottomRight),
        }
    }
}

impl<'a, 'm> System<'a> for RenderSystem<'m> {
    type SystemData = (
        ReadExpect<'a, Viewport>,
        ReadExpect<'a, Map>,
        Read<'a, WorldOffset>,
        Read<'a, Input>,
        Read<'a, EntityTracker>,
        ReadStorage<'a, Body>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TargetLocation>,
    );

    fn run(
        &mut self,
        (viewport, map, offset, input, entity_tracker, body, position, target): Self::SystemData,
    ) {
        let ref mut gc = self.gc;
        let ref panels = [&self.hovered_panel];
        self.gl.draw(*viewport, |c, g| {
            let transform = get_world_transform(c.transform, &offset);
            let (mouse_x, mouse_y) = offset.to_local_pixel(input.mouse_x, input.mouse_y);

            clear(color::hex("000000"), g);
            map.render(transform, g, mouse_x, mouse_y);

            for (body, position) in (&body, &position).join() {
                draw_body(body, position, transform, g);
            }
            for (position, target) in (&position, &target).join() {
                draw_targeting_line(position, &target.position, transform, g);
            }
            for panel in panels {
                panel.render(c.transform, g);
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
            // TODO: Actually render some UI thing for hovered entity.
            if let Some(entity) = entity_tracker.hovered {
                println!("Hovering over {:?}", entity);
            }
        })
    }
}

fn get_world_transform(global_transform: Matrix2d, offset: &WorldOffset) -> Matrix2d {
    global_transform
        .trans(offset.h, offset.v)
        .scale(offset.scale, offset.scale)
}

fn draw_body(body: &Body, position: &Position, transform: Matrix2d, g: &mut GlGraphics) {
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

fn draw_targeting_line(p1: &Position, p2: &Position, transform: Matrix2d, g: &mut GlGraphics) {
    line(
        [1.0, 0.0, 0.0, 1.0],
        1.0,
        [p1.x, p1.y, p2.x, p2.y],
        transform,
        g,
    );
}
