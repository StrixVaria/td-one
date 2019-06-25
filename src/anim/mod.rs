use graphics::math::Matrix2d;
use graphics::*;

pub enum AnimationType {
    Explosion,
}

pub struct Animation {
    anim_type: AnimationType,
    elapsed: f64,
    x: f64,
    y: f64,
    size: f64,
}

impl Animation {
    pub fn new(anim_type: AnimationType, x: f64, y: f64, size: f64) -> Self {
        Self {
            anim_type,
            x,
            y,
            size,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.elapsed += dt;
    }

    pub fn render<G: Graphics>(&self, c: Matrix2d, g: &mut G) -> bool {
        use AnimationType::*;
        match self.anim_type {
            Explosion => explosion_animation(self.x, self.y, self.size, self.elapsed, c, g)
        }
    }
}

fn explosion_animation<G: Graphics>(x: f64, y: f64, size: f64, elapsed: f64, c: Matrix2d, g: &mut G) -> bool {
    const ANIM_LENGTH: f64 = 3.0;
    let color = [0.9, 0.3, 0.0, 1.0 - (elapsed as f32 / ANIM_LENGTH as f32)];
    ellipse(color, rectangle::centered_square(x, y, size), c, g);
    elapsed >= ANIM_LENGTH
}