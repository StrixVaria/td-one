use crate::components::Position;
use std::ops::{Div, Mul, MulAssign};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Does not cache magnitude. Avoid reusing often.
    pub fn unit(&self) -> Vector {
        *self / self.mag()
    }

    /// Does not cache results. Avoid reusing often.
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<&f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: &f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub fn get_unit_dir(p1: Position, p2: Position) -> Vector {
    Vector::new(p2.x - p1.x, p2.y - p1.y).unit()
}
