use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

use std::marker::PhantomData;

pub struct TextBox<C: CharacterCache> {
    lines: Vec<String>,
    width: f64,
    size: FontSize,
    cache_type: PhantomData<*const C>,
}

impl<C: CharacterCache> TextBox<C> {
    const MARGIN: f64 = 2.0;

    pub fn new(text: &str, width: f64, size: FontSize, glyph_cache: &mut C) -> Self {
        TextBox {
            lines: Self::get_lines(text, width, size, glyph_cache),
            width,
            size,
            cache_type: PhantomData,
        }
    }

    pub fn render<G>(&self, x: f64, y: f64, glyph_cache: &mut C, c: Context, g: &mut G)
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        let h = self.height();
        rectangle(color::hex("ffffff"), [x, y, self.width, h], c.transform, g);
        rectangle(
            color::hex("003333"),
            [
                x + Self::MARGIN,
                y + Self::MARGIN,
                self.width - 2.0 * Self::MARGIN,
                h - 2.0 * Self::MARGIN,
            ],
            c.transform,
            g,
        );
        for (i, line) in self.lines.iter().enumerate() {
            let line_h = self.size as f64 * (i + 1) as f64;
            let t = c
                .transform
                .trans(x + Self::MARGIN * 2.0, y + line_h + Self::MARGIN * 1.5);
            // unwrap isn't implemented for this! yay!
            match text(color::hex("ffffff"), self.size, line, glyph_cache, t, g) {
                _ => {}
            }
        }
    }

    pub fn height(&self) -> f64 {
        self.lines.len() as f64 * self.size as f64 + 5.0 * Self::MARGIN
    }

    fn get_lines(text: &str, width: f64, size: FontSize, glyph_cache: &mut C) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        for line in text.lines() {
            if let Ok(wrapped_lines) =
                Self::wrap(line, glyph_cache, size, width - 4.0 * Self::MARGIN)
            {
                for wrapped_line in wrapped_lines {
                    lines.push(wrapped_line);
                }
            }
        }
        lines
    }

    fn wrap(
        text: &str,
        glyph_cache: &mut C,
        size: FontSize,
        width: f64,
    ) -> Result<Vec<String>, C::Error> {
        let mut lines = vec![];
        let words: Vec<&str> = text.split(" ").collect();
        let mut cur_line = String::new();
        let mut cur_length = 0.0;
        let space_width = glyph_cache.width(size, " ")?;
        for word in words {
            let word_length = glyph_cache.width(size, word)?;
            if cur_length + word_length + space_width > width {
                lines.push(cur_line);
                cur_line = String::new();
                cur_length = 0.0;
            }
            let add_space = cur_length > 0.0;
            cur_length += word_length + if add_space { space_width } else { 0.0 };
            if add_space {
                cur_line.push_str(" ");
            }
            cur_line.push_str(word);
        }
        lines.push(cur_line);

        Ok(lines)
    }
}
