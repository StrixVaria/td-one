use graphics::{math::Matrix2d, rectangle, types::Color};
use opengl_graphics::GlGraphics;

use crate::{systems::render::anchor::Anchor};

const DEFAULT_BG_COLOR: Color = [0.0, 0.3, 0.3, 1.0];
const DEFAULT_BORDER_COLOR: Color = [1.0; 4];
const DEFAULT_BORDER_WIDTH: f64 = 2.0;

pub struct Panel {
    bg_color: Color,
    border_color: Color,
    border_width: f64,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    anchor: Anchor,
}

impl Panel {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            bg_color: DEFAULT_BG_COLOR,
            border_color: DEFAULT_BORDER_COLOR,
            border_width: DEFAULT_BORDER_WIDTH,
            x,
            y,
            w,
            h,
            anchor: Anchor::TopLeft,
        }
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn with_border_width(mut self, width: f64) -> Self {
        self.border_width = width;
        self
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn render(&self, transform: Matrix2d, g: &mut GlGraphics) {
        let (x, y) = self.anchor.absolute(self.x, self.y, self.w, self.h);
        rectangle(self.border_color, [x, y, self.w, self.h], transform, g);
        let (x, y) = (x + self.border_width, y + self.border_width);
        let (w, h) = (self.w - 2.0 * self.border_width, self.h - 2.0 * self.border_width);
        rectangle(self.bg_color, [x, y, w, h], transform, g);
    }
}