use std::fmt;

const ACTOR_REF_SIZE: f64 = 10.0;

pub mod actor;
pub mod ai;
pub mod map;
pub mod task;

mod vector;

use crate::qt::*;
use crate::anim::Animation;
pub use actor::*;
pub use ai::*;
pub use task::*;

pub struct UpdateResults {
    pub new_actors: Vec<Actor>,
    pub dead_actors: Vec<usize>,
    pub new_animations: Vec<Animation>,
}

#[derive(Clone, Copy, Debug)]
pub struct ActorRef {
    pub id: usize,
    region: Region,
}

impl HasRegion for ActorRef {
    fn get_region(&self) -> Region {
        self.region.clone()
    }
}
