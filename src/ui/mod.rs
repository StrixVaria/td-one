use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

pub mod text;
pub use text::*;

use std::marker::PhantomData;
pub struct GUI<C: CharacterCache> {
    cache_type: PhantomData<*const C>,
    mouse_coords: TextBox<C>,
    screen_width: f64,
    screen_height: f64,
}

impl<C: CharacterCache> GUI<C> {
    pub const FONT_SIZE: FontSize = 12;

    pub fn new(width: f64, height: f64, glyph_cache: &mut C) -> Self {
        let mut mouse_box = TextBox::new("", 1000.0, Self::FONT_SIZE, glyph_cache);
        mouse_box.auto_width(glyph_cache);
        Self {
            cache_type: PhantomData,
            mouse_coords: mouse_box,
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn render<G>(&self, glyph_cache: &mut C, c: Context, g: &mut G)
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        let mouse_box_y = self.screen_height - Self::FONT_SIZE as f64 - 5.0 * TextBox::<C>::MARGIN;
        let mouse_box_x = self.screen_width - self.mouse_coords.width();
        self.mouse_coords.render(mouse_box_x, mouse_box_y, glyph_cache, c, g);
    }

    pub fn mouse_pos(&mut self, x: f64, y: f64, glyph_cache: &mut C) {
        self.mouse_coords.update_text_one_line(format!("({}, {})", x.floor(), y.floor()).as_str(), glyph_cache);
    }
}
