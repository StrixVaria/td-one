use crate::math::Vector;
use specs::{Component, Entity, VecStorage};
use std::ops::AddAssign;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone, Copy)]
pub enum Mobility {
    Omnidirectional { speed: f64 },
}

impl Component for Mobility {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct TargetLocation {
    pub position: Position,
}

impl Component for TargetLocation {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct TargetEntity {
    pub entity: Entity,
}

impl Component for TargetEntity {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub enum BodyShape {
    Circle,
    Square,
}

#[derive(Clone, Copy)]
pub struct Body {
    pub body_shape: BodyShape,
    pub size: f64,
    pub color: [f32; 4],
}

impl Component for Body {
    type Storage = VecStorage<Self>;
}