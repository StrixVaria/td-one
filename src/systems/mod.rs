mod motion;
pub use motion::*;

mod targeting;
pub use targeting::*;

mod render;
pub use render::*;

mod input;
pub use input::*;

#[derive(Default)]
pub struct DeltaTime(f64);

impl From<f64> for DeltaTime {
    fn from(dt: f64) -> Self {
        Self(dt)
    }
}
