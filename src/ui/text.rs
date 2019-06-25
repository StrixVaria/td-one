use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::*;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub enum AnchorPoint {
    // TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    // Center,
}

pub struct TextBox<C: CharacterCache> {
    lines: Vec<String>,
    width: f64,
    display_lines: Option<usize>,
    line_offset: usize,
    size: FontSize,
    cache_type: PhantomData<*const C>,
    abs_x: f64,
    abs_y: f64,
    x: f64,
    y: f64,
    anchor: AnchorPoint,
}

impl<C: CharacterCache> TextBox<C> {
    pub const MARGIN: f64 = 2.0;

    pub fn new(
        text: &str,
        width: f64,
        size: FontSize,
        x: f64,
        y: f64,
        anchor: AnchorPoint,
        glyph_cache: &mut C,
    ) -> Self {
        let mut text_box = TextBox {
            lines: get_lines(text, width, size, glyph_cache, Self::MARGIN),
            width,
            display_lines: None,
            line_offset: 0,
            size,
            cache_type: PhantomData,
            x,
            y,
            anchor,
            abs_x: 0.0,
            abs_y: 0.0,
        };
        text_box.realign();
        text_box
    }

    pub fn render<G>(&self, glyph_cache: &mut C, c: Context, g: &mut G) -> Result<(), C::Error>
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        let h = self.height();
        rectangle(
            color::hex("ffffff"),
            [self.abs_x, self.abs_y, self.width, h],
            c.transform,
            g,
        );
        rectangle(
            color::hex("003333"),
            [
                self.abs_x + Self::MARGIN,
                self.abs_y + Self::MARGIN,
                self.width - 2.0 * Self::MARGIN,
                h - 2.0 * Self::MARGIN,
            ],
            c.transform,
            g,
        );
        if let Some(num_lines) = self.display_lines {
            let mut lines_rendered = 0;
            let mut line_index = self.line_offset;
            while lines_rendered < num_lines {
                if line_index >= self.lines.len() {
                    break;
                }
                let line_h = self.size as f64 * (lines_rendered + 1) as f64;
                let t = c.transform.trans(
                    self.abs_x + Self::MARGIN * 2.0,
                    self.abs_y + line_h + Self::MARGIN * 1.5,
                );
                text(color::hex("ffffff"), self.size, self.lines[line_index].as_str(), glyph_cache, t, g)?;
                lines_rendered += 1;
                line_index += 1;
            }
        } else {
            for (i, line) in self.lines.iter().enumerate() {
                let line_h = self.size as f64 * (i + 1) as f64;
                let t = c.transform.trans(
                    self.abs_x + Self::MARGIN * 2.0,
                    self.abs_y + line_h + Self::MARGIN * 1.5,
                );
                text(color::hex("ffffff"), self.size, line, glyph_cache, t, g)?;
            }
        }
        Ok(())
    }

    pub fn height(&self) -> f64 {
        if let Some(height) = self.display_lines {
            // Only display the number of lines indicated.
            height as f64 * self.size as f64 + 5.0 * Self::MARGIN
        } else {
            // Show all lines, and let the box scale up/down to whatever size necessary.
            self.lines.len() as f64 * self.size as f64 + 5.0 * Self::MARGIN
        }
    }

    // pub fn width(&self) -> f64 {
    //     self.width
    // }

    pub fn auto_width(&mut self, glyph_cache: &mut C) {
        let mut final_width = 0.0;
        for line in self.lines.iter() {
            if let Ok(line_width) = glyph_cache.width(self.size, line) {
                if line_width > final_width {
                    final_width = line_width;
                }
            }
        }
        self.width = final_width + 4.0 * Self::MARGIN;
        self.realign();
    }

    pub fn set_width(&mut self, width: f64, glyph_cache: &mut C) {
        self.lines = get_lines(
            self.lines.join("\n").as_str(),
            width,
            self.size,
            glyph_cache,
            Self::MARGIN,
        );
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.display_lines = Some(height);
    }

    pub fn update_text(&mut self, new_text: &str, glyph_cache: &mut C) {
        self.lines = get_lines(new_text, self.width, self.size, glyph_cache, Self::MARGIN);
    }

    pub fn update_text_one_line(&mut self, new_text: &str, glyph_cache: &mut C) {
        self.lines = vec![new_text.into()];
        self.auto_width(glyph_cache);
    }

    pub fn reposition(&mut self, x: f64, y: f64, anchor: AnchorPoint) {
        self.x = x;
        self.y = y;
        self.anchor = anchor;
        self.realign();
    }

    pub fn realign(&mut self) {
        use AnchorPoint::*;
        let (x, y) = match self.anchor {
            // TopLeft => (self.x, self.y),
            TopRight => (self.x - self.width, self.y),
            BottomLeft => (self.x, self.y - self.height()),
            BottomRight => (self.x - self.width, self.y - self.height()),
        };
        self.abs_x = x;
        self.abs_y = y;
    }

    pub fn in_bounds(&self, x: f64, y: f64) -> bool {
        x >= self.abs_x && x <= self.abs_x + self.width && y >= self.abs_y && y <= self.abs_y + self.height()
    }

    pub fn reset_scroll(&mut self) {
        self.line_offset = 0;
    }

    pub fn scroll(&mut self, up: bool) {
        if up {
            if self.line_offset > 0 {
                self.line_offset -= 1;
            }
        } else {
            if self.line_offset < self.lines.len() - 1 {
                self.line_offset += 1;
            }
        }
    }
}

fn get_lines<C: CharacterCache>(
    text: &str,
    width: f64,
    size: FontSize,
    glyph_cache: &mut C,
    margin: f64,
) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in text.lines() {
        if let Ok(wrapped_lines) = wrap(line, glyph_cache, size, width - 4.0 * margin) {
            for wrapped_line in wrapped_lines {
                lines.push(wrapped_line);
            }
        }
    }
    lines
}

fn wrap<C: CharacterCache>(
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
