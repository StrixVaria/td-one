use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

pub mod text;
pub use text::*;

pub struct GUI<'a, C: CharacterCache> {
    text_boxes: Vec<TextBox<C>>,
    // mouse_coords: TextBox<C>,
    // hovered_actor: TextBox<C>,
    // selected_actor: TextBox<C>,
    // screen_width: f64,
    // screen_height: f64,
    glyph_cache: &'a mut C,
}

impl<'a, C: CharacterCache> GUI<'a, C> {
    pub const FONT_SIZE: FontSize = 12;
    const MOUSE_BOX: usize = 0;
    const HOVERED_BOX: usize = 1;
    const SELECTED_BOX: usize = 2;

    pub fn new(width: f64, height: f64, glyph_cache: &'a mut C) -> Self {
        let mouse_box = TextBox::new(
            "",
            1000.0,
            Self::FONT_SIZE,
            width,
            0.0,
            AnchorPoint::TopRight,
            glyph_cache,
        );
        let mut hovered_box = TextBox::new(
            "",
            width / 2.0,
            Self::FONT_SIZE,
            width,
            height,
            AnchorPoint::BottomRight,
            glyph_cache,
        );
        hovered_box.set_height(7);
        hovered_box.realign();
        let mut selected_box = TextBox::new(
            "",
            width / 2.0,
            Self::FONT_SIZE,
            0.0,
            height,
            AnchorPoint::BottomLeft,
            glyph_cache,
        );
        selected_box.set_height(7);
        selected_box.realign();
        let boxes = vec![mouse_box, hovered_box, selected_box];
        Self {
            text_boxes: boxes,
            // screen_width: width,
            // screen_height: height,
            glyph_cache,
        }
    }

    pub fn render<G>(&mut self, c: Context, g: &mut G) -> Result<(), C::Error>
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        for text_box in self.text_boxes.iter() {
            text_box.render(&mut self.glyph_cache, c, g)?;
        }
        Ok(())
    }

    pub fn resize(&mut self, w: f64, h: f64) {
        self.text_boxes[Self::MOUSE_BOX].reposition(w, 0.0, AnchorPoint::TopRight);
        self.text_boxes[Self::HOVERED_BOX].set_width(w / 2.0, self.glyph_cache);
        self.text_boxes[Self::HOVERED_BOX]
            .reposition(w, h, AnchorPoint::BottomRight);
        self.text_boxes[Self::SELECTED_BOX].set_width(w / 2.0, self.glyph_cache);
        self.text_boxes[Self::SELECTED_BOX]
            .reposition(0.0, h, AnchorPoint::BottomLeft);
    }

    pub fn mouse_pos(&mut self, x: f64, y: f64) {
        self.text_boxes[Self::MOUSE_BOX].update_text_one_line(
            format!("({}, {})", x.floor(), y.floor()).as_str(),
            &mut self.glyph_cache,
        );
    }

    pub fn hovered_desc(&mut self, desc: &str) {
        if desc == "" {
            self.text_boxes[Self::HOVERED_BOX].update_text("Hover over a unit for details.", &mut self.glyph_cache);
        } else {
            self.text_boxes[Self::HOVERED_BOX].update_text(desc, &mut self.glyph_cache);
        }
    }

    pub fn selected_desc(&mut self, desc: &str) {
        if desc == "" {
            self.text_boxes[Self::SELECTED_BOX].update_text("Click on a unit to pin details here.", &mut self.glyph_cache);
        } else {
            self.text_boxes[Self::SELECTED_BOX].update_text(desc, &mut self.glyph_cache);
        }
    }

    /// Returns whether the click was on a UI element. If true, the click was
    /// already handled.
    pub fn handle_click(&mut self, x: f64, y: f64) -> bool {
        if let Some(_text_box) = self.in_bounds(x, y) {
            // TODO: Actually handle the click here.
            true
        } else {
            false
        }
    }

    fn in_bounds(&mut self, x: f64, y: f64) -> Option<&mut TextBox<C>> {
        for text_box in self.text_boxes.iter_mut() {
            if text_box.in_bounds(x, y) {
                return Some(text_box);
            }
        }
        None
    }

    pub fn handle_scroll(&mut self, x: f64, y: f64, up: bool) -> bool {
        if let Some(text_box) = self.in_bounds(x, y) {
            text_box.scroll(up);
            true
        } else {
            false
        }
    }
}
