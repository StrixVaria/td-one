/// Get unit vector in direction from (x1, y1) to (x2, y2)
pub fn direction(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    unit(x2 - x1, y2 - y1)
}

// TODO
// pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
//     mag(x2 - x1, y2 - y1)
// }

/// Return whether (x1,y1) is within d distance of (x2,y2)
pub fn distance_cmp(x1: f64, y1: f64, x2: f64, y2: f64, d: f64) -> bool {
    (x2 - x1).powi(2) + (y2 - y1).powi(2) <= d.powi(2)
}

pub fn unit(x: f64, y: f64) -> (f64, f64) {
    let mag = mag(x, y);
    (x / mag, y / mag)
}

pub fn mag(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

pub fn scale(x: f64, y: f64, factor: f64) -> (f64, f64) {
    (x * factor, y * factor)
}
