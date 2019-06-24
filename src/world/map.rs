use graphics::math::Matrix2d;
use graphics::types::Color;
use graphics::*;
use rand::Rng;

const GRID_TILE_SIZE: f64 = 12.0;

pub enum Tile {
    Grass,
    Forest,
    Stone,
    Ore,
    Gold,
    Road,
    Building,
}

pub struct WorldBounds {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl WorldBounds {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn in_bounds(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn constrain(&self, mut x: f64, mut y: f64) -> (f64, f64) {
        if x < self.x {
            x = self.x;
        } else if x > self.x + self.w {
            x = self.x + self.w;
        }
        if y < self.y {
            y = self.y;
        } else if y >= self.y + self.h {
            y = self.y + self.h;
        }
        (x, y)
    }
}

pub struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = (0..width * height)
            .map(|_| match rand::thread_rng().gen_range(0, 7) {
                0 => Tile::Grass,
                1 => Tile::Road,
                2 => Tile::Stone,
                3 => Tile::Ore,
                4 => Tile::Gold,
                5 => Tile::Forest,
                _ => Tile::Building,
            })
            .collect();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn render<G: Graphics>(&self, t: Matrix2d, g: &mut G, mouse_x: f64, mouse_y: f64) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.render_tile(x, y, t, g);
            }
        }

        // Highlight the current cell
        match self.get_cell_for_pixel(mouse_x, mouse_y) {
            Some((x, y)) => self.render_cell(x, y, [1.0, 1.0, 1.0, 0.3], t, g),
            _ => {}
        }
    }

    fn render_tile<G: Graphics>(&self, x: usize, y: usize, t: Matrix2d, g: &mut G) {
        let color = match self.grid[self.get_index(x, y)] {
            Tile::Grass => color::hex("005555"),
            Tile::Forest => color::hex("006666"),
            Tile::Road => color::hex("007777"),
            Tile::Building => color::hex("008888"),
            Tile::Gold => color::hex("009999"),
            Tile::Ore => color::hex("00aaaa"),
            Tile::Stone => color::hex("00bbbb"),
        };

        self.render_cell(x, y, color, t, g);
    }

    fn render_cell<G: Graphics>(
        &self,
        x_index: usize,
        y_index: usize,
        color: Color,
        t: Matrix2d,
        g: &mut G,
    ) {
        let x = x_index as f64 * GRID_TILE_SIZE;
        let y = y_index as f64 * GRID_TILE_SIZE;
        let rect = rectangle::square(x, y, GRID_TILE_SIZE);
        rectangle(color, rect, t, g);
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn get_cell_for_pixel(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let bounds = self.get_bounds();
        if !bounds.in_bounds(x, y) {
            None
        } else {
            Some((
                (x / GRID_TILE_SIZE - 0.01).floor() as usize,
                (y / GRID_TILE_SIZE - 0.01).floor() as usize,
            ))
        }
    }

    pub fn get_cell_loc(&self, x: usize, y: usize) -> (f64, f64) {
        (x as f64 * GRID_TILE_SIZE, y as f64 * GRID_TILE_SIZE)
    }

    pub fn get_bounds(&self) -> WorldBounds {
        WorldBounds::new(0.0, 0.0, self.width as f64 * GRID_TILE_SIZE, self.height as f64 * GRID_TILE_SIZE)
    }
}
