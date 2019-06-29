const MIN_SCALE: f64 = 0.25;
const MAX_SCALE: f64 = 2.50;

#[derive(Default)]
pub struct WorldOffset {
    pub v: f64,
    pub h: f64,
    pub scale: f64,
}

impl WorldOffset {
    pub fn new() -> Self {
        WorldOffset {
            v: 0.0,
            h: 0.0,
            scale: 1.0,
        }
    }

    pub fn slide(&mut self, dx: f64, dy: f64) {
        self.v += dy;
        self.h += dx;
    }

    pub fn zoom(&mut self, zoom_in: bool, x_center: f64, y_center: f64) {
        let mut zoom_ratio = match zoom_in {
            true => 1.0 * 1.1,
            false => 1.0 / 1.1,
        };
        // Cap how far you can zoom in/out.
        let old_scale = self.scale;
        self.scale *= zoom_ratio;
        if self.scale < MIN_SCALE {
            self.scale = MIN_SCALE;
            zoom_ratio = self.scale / old_scale;
        } else if self.scale > MAX_SCALE {
            self.scale = MAX_SCALE;
            zoom_ratio = self.scale / old_scale;
        }

        // Center the zoom wherever the mouse cursor is.
        // x_center - self.h is the position of the mouse relative to the position of the grid
        // scale that relative value up/down
        // then move it back to an absolute position on the window rather than a relative position in the grid
        // then subtract the original absolute position to find the dx
        let dx = (x_center - self.h) * zoom_ratio + self.h - x_center;
        // same but in the other axis
        let dy = (y_center - self.v) * zoom_ratio + self.v - y_center;

        self.slide(-dx, -dy);
    }

    // pub fn to_global_pixel(&self, x: f64, y: f64) -> (f64, f64) {
    //     (
    //         (x * self.scale) + self.h,
    //         (y * self.scale) + self.v,
    //     )
    // }

    pub fn to_local_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.h) / self.scale, (y - self.v) / self.scale)
    }
}
