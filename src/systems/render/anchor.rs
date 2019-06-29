
pub enum Anchor {
    TopLeft,
    // TopRight,
    BottomLeft,
    BottomRight,
    // Center,
}

impl Anchor {
    pub fn absolute(&self, x: f64, y: f64, w: f64, h: f64) -> (f64, f64) {
        use Anchor::*;
        match self {
            TopLeft => (x, y),
            // TopRight => (x - w, y),
            BottomLeft => (x, y - h),
            BottomRight => (x - w, y - h),
            // Center => (x - w / 2.0, y - h / 2.0),
        }
    }
}
