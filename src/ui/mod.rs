use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

pub mod text;
pub use text::*;

pub struct GUI<'a, C: CharacterCache> {
    mouse_coords: TextBox<C>,
    hovered_actor: TextBox<C>,
    selected_actor: TextBox<C>,
    // screen_width: f64,
    // screen_height: f64,
    glyph_cache: &'a mut C,
}

impl<'a, C: CharacterCache> GUI<'a, C> {
    pub const FONT_SIZE: FontSize = 12;

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
        Self {
            mouse_coords: mouse_box,
            hovered_actor: hovered_box,
            selected_actor: selected_box,
            // screen_width: width,
            // screen_height: height,
            glyph_cache,
        }
    }

    pub fn render<G>(&mut self, c: Context, g: &mut G) -> Result<(), C::Error>
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        self.mouse_coords.render(&mut self.glyph_cache, c, g)?;
        self.hovered_actor.render(&mut self.glyph_cache, c, g)?;
        self.selected_actor.render(&mut self.glyph_cache, c, g)?;
        Ok(())
    }

    pub fn resize(&mut self, w: f64, h: f64) {
        self.mouse_coords.reposition(w, 0.0, AnchorPoint::TopRight);
        self.hovered_actor.set_width(w / 2.0, self.glyph_cache);
        self.hovered_actor
            .reposition(w, h, AnchorPoint::BottomRight);
        self.selected_actor.set_width(w / 2.0, self.glyph_cache);
        self.selected_actor
            .reposition(0.0, h, AnchorPoint::BottomLeft);
    }

    pub fn mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_coords.update_text_one_line(
            format!("({}, {})", x.floor(), y.floor()).as_str(),
            &mut self.glyph_cache,
        );
    }

    pub fn hovered_desc(&mut self, desc: &str) {
        if desc == "" {
            self.hovered_actor.update_text("Hover over a unit for details.", &mut self.glyph_cache);
        } else {
            self.hovered_actor.update_text(desc, &mut self.glyph_cache);
        }
    }

    pub fn selected_desc(&mut self, desc: &str) {
        if desc == "" {
            self.selected_actor.update_text("Click on a unit to pin details here.", &mut self.glyph_cache);
        } else {
            self.selected_actor.update_text(desc, &mut self.glyph_cache);
        }
    }
}
