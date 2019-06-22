use graphics::math::Matrix2d;
use graphics::*;
use std::time::Instant;

use crate::world::*;

#[derive(Debug)]
pub struct Actor {
    pub name: Option<String>,
    pub x: f64,
    pub y: f64,
    pub speed: Option<f64>,
    sight: Option<f64>,
    body: ActorBody,
    pub ai: ActorAi,
    pub task: Option<Task>,
    pub id: Instant,
}

impl Actor {
    pub fn new(x: f64, y: f64, body: ActorBody, ai: ActorAi) -> Self {
        Self {
            name: None,
            x,
            y,
            speed: Some(20.0),
            sight: Some(25.0),
            body,
            ai,
            task: None,
            id: Instant::now(),
        }
    }

    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn render<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.body.render(self.x, self.y, t, g);
    }
    
    pub fn render_extras<G: Graphics>(&self, actors: &Vec<Actor>, t: Matrix2d, g: &mut G) {
        if let Some(sight) = self.sight {
            ellipse([1.0, 1.0, 1.0, 0.3], rectangle::centered_square(self.x, self.y, sight), t, g);
        }
        if let Some(ref task) = self.task {
            if let Some(target) = task.get_target_index() {
                line([0.8, 0.2, 0.2, 1.0], 1.0, [self.x, self.y, actors[target].x, actors[target].y], t, g);
            }
        }
    }

    pub fn update_all(dt: f64, actors: &mut Vec<Actor>, qt: &QuadTree<ActorRef>) -> UpdateResults {
        let mut new_actors = vec![];
        let mut dead_actors = vec![];
        for i in 0..actors.len() {
            let task_completion = Task::execute(i, dt, actors, qt);
            if let Some(actor) = task_completion.new_actor {
                new_actors.push(actor);
            }
            if let Some(killed) = task_completion.dead_actors {
                for actor_id in killed {
                    if !dead_actors.contains(&actor_id) {
                        dead_actors.push(actor_id);
                    }
                }
            }
            match task_completion.next_action {
                NextAction::AiChoice => {
                    actors[i].task = Some(ActorAi::get_task(
                        i,
                        actors,
                        task_completion.prev_target,
                        qt,
                    ))
                }
                NextAction::ChangeTo(next) => actors[i].task = Some(next),
                NextAction::Continue => {}
            }
        }
        UpdateResults {
            new_actors,
            dead_actors,
        }
    }

    pub fn get_ref(&self, id: usize) -> ActorRef {
        ActorRef {
            id,
            region: self.get_region(),
        }
    }

    pub fn get_region(&self) -> Region {
        self.body.get_region(self.x, self.y)
    }

    // TODO
    // pub fn get_sight_range(&self) -> Option<Region> {
    //     self.sight
    //         .map(|sight| Region::new_circle(self.x, self.y, sight))
    // }

    pub fn can_see(&self, x: f64, y: f64) -> bool {
        match self.sight {
            Some(sight) => vector::distance_cmp(self.x, self.y, x, y, sight),
            None => false,
        }
    }

    /// Returns whether or not it arrived. Always returns false if this actor
    /// can't move.
    pub fn step_towards(&mut self, x: f64, y: f64, dt: f64) -> bool {
        match self.speed {
            Some(speed) => {
                let speed = speed * dt;
                if vector::distance_cmp(self.x, self.y, x, y, speed) {
                    self.x = x;
                    self.y = y;
                    true
                } else {
                    self.step_in_dir(self.x, self.y, x, y, dt);
                    false
                }
            }
            None => false,
        }
    }

    /// Take a step in the direction directly away from (x,y)
    pub fn step_from(&mut self, x: f64, y: f64, dt: f64) {
        self.step_in_dir(x, y, self.x, self.y, dt);
    }

    pub fn description(i: usize, actors: &Vec<Actor>) -> String {
        let mut desc = String::new();
        let ref actor = actors[i];
        if let Some(name) = &actor.name {
            desc += format!("Name: {}\n", name).as_str();
        }
        desc += format!("Body: {}", actor.body).as_str();
        desc += format!("\nAI: {}", actor.ai).as_str();
        if actor.task.is_some() {
            desc += format!("\n\n{}", Task::description(i, actors)).as_str();
        }

        desc
    }

    /// Step this actor's speed in direction from (x1,y1) to (x2,y2). Does
    /// nothing if actor can't move.
    fn step_in_dir(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, dt: f64) {
        if let Some(speed) = self.speed {
            let (dx, dy) = vector::direction(x1, y1, x2, y2);
            let (dx, dy) = vector::scale(dx, dy, speed * dt);
            self.x += dx;
            self.y += dy;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ActorBody {
    Worker,
    Building,
}

impl ActorBody {
    pub fn render<G: Graphics>(&self, x: f64, y: f64, t: Matrix2d, g: &mut G) {
        let render_callback = match self {
            ActorBody::Worker => ActorBody::worker_render,
            ActorBody::Building => ActorBody::building_render,
        };
        render_callback(self, x, y, t, g);
    }

    pub fn get_region(&self, x: f64, y: f64) -> Region {
        let size = self.size();
        match self {
            ActorBody::Worker => Region::new_circle(x, y, size / 2.0),
            ActorBody::Building => Region::new_rect(x, y, size, size),
        }
    }

    fn worker_render<G: Graphics>(&self, x: f64, y: f64, t: Matrix2d, g: &mut G) {
        let color = color::hex("ffffff");
        let (x1, y1, x2, y2) = ActorBody::circle_position(x, y, self.size());
        ellipse(color, rectangle::rectangle_by_corners(x1, y1, x2, y2), t, g);
    }

    fn building_render<G: Graphics>(&self, x: f64, y: f64, t: Matrix2d, g: &mut G) {
        let color = color::hex("888888");
        rectangle(color, rectangle::square(x, y, self.size()), t, g);
    }

    fn size(&self) -> f64 {
        match self {
            ActorBody::Worker => ACTOR_REF_SIZE,
            ActorBody::Building => ACTOR_REF_SIZE * 2.0,
        }
    }

    /// Get a bounding rectangle for the circle with given (x,y) center and diameter
    pub fn circle_position(x: f64, y: f64, diameter: f64) -> (f64, f64, f64, f64) {
        let radius = diameter / 2.0;
        (x - radius, y - radius, x + radius, y + radius)
    }
}

impl fmt::Display for ActorBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActorBody::Worker => "Worker",
                ActorBody::Building => "Building",
            }
        )
    }
}
