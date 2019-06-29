use graphics::{math::Matrix2d, types::{FontSize, Color}, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};

use crate::systems::render::{panel::UIElem, anchor::Anchor};

const DEFAULT_TEXT_COLOR: Color = [1.0; 4];
const DEFAULT_FONT_SIZE: FontSize = 12;

pub struct Text {
    id: String,
    color: Color,
    lines: Vec<String>,
    x: f64,
    y: f64,
    w: f64,
    anchor: Anchor,
    font_size: FontSize,
}

impl Text {
    pub fn new(id: &str, text: &str, width: f64) -> Self {
        Self {
            id: id.into(),
            color: DEFAULT_TEXT_COLOR,
            lines: text.split('\n').map(|s| s.into()).collect(),
            x: 0.0,
            y: 0.0,
            w: width,
            anchor: Anchor::TopLeft,
            font_size: DEFAULT_FONT_SIZE,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_size(mut self, size: FontSize) -> Self {
        self.font_size = size;
        self
    }

    fn height(&self) -> f64 {
        self.font_size as f64 * self.lines.len() as f64
    }
}

impl UIElem for Text {
    fn render(&self, gc: &mut GlyphCache, transform: Matrix2d, g: &mut GlGraphics) {
        let (x, mut y) = self.anchor.absolute(self.x, self.y, self.w, self.height());
        // Text is always rendered from the bottom left of the first character,
        // so we need to offset by an additional font size compared to the
        // anchoring.
        y += self.font_size as f64;
        for line in self.lines.iter() {
            graphics::text(self.color, self.font_size, line.as_str(), gc, transform.trans(x, y), g).unwrap();
            y += self.font_size as f64;
        }
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}