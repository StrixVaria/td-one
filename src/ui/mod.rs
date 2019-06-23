use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

pub mod text;
pub use text::*;

use std::marker::PhantomData;
pub struct GUI<C: CharacterCache> {
    cache_type: PhantomData<*const C>,
    mouse_coords: TextBox<C>,
    hovered_actor: TextBox<C>,
    selected_actor: TextBox<C>,
    // screen_width: f64,
    // screen_height: f64,
}

impl<C: CharacterCache> GUI<C> {
    pub const FONT_SIZE: FontSize = 12;

    pub fn new(width: f64, height: f64, glyph_cache: &mut C) -> Self {
        let mouse_box = TextBox::new("", 1000.0, Self::FONT_SIZE, width, 0.0, AnchorPoint::TopRight, glyph_cache);
        let hovered_box = TextBox::new("", width / 2.0, Self::FONT_SIZE, width, height, AnchorPoint::BottomRight, glyph_cache);
        let selected_box = TextBox::new("", width / 2.0, Self::FONT_SIZE, 0.0, height, AnchorPoint::BottomLeft, glyph_cache);
        Self {
            cache_type: PhantomData,
            mouse_coords: mouse_box,
            hovered_actor: hovered_box,
            selected_actor: selected_box,
            // screen_width: width,
            // screen_height: height,
        }
    }

    pub fn render<G>(&self, glyph_cache: &mut C, c: Context, g: &mut G)
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        self.mouse_coords.render(glyph_cache, c, g);
        self.hovered_actor.render(glyph_cache, c, g);
        self.selected_actor.render(glyph_cache, c, g);
    }

    pub fn mouse_pos(&mut self, x: f64, y: f64, glyph_cache: &mut C) {
        self.mouse_coords.update_text_one_line(
            format!("({}, {})", x.floor(), y.floor()).as_str(),
            glyph_cache,
        );
    }

    pub fn hovered_desc(&mut self, desc: &str, glyph_cache: &mut C) {
        self.hovered_actor.update_text(desc, glyph_cache);
        self.hovered_actor.realign();
    }

    pub fn selected_desc(&mut self, desc: &str, glyph_cache: &mut C) {
        self.selected_actor.update_text(desc, glyph_cache);
        self.selected_actor.realign();
    }
}
