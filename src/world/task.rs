use std::time::Instant;

use crate::world::*;

#[derive(Debug, Clone)]
pub struct Target {
    pub index: Option<usize>,
    pub id: Instant,
}

impl Target {
    pub fn new(index: usize, id: Instant) -> Self {
        Self {
            index: Some(index),
            id,
        }
    }
}

#[derive(Debug)]
pub struct TaskParams {
    target: Option<Target>,
    x: Option<f64>,
    y: Option<f64>,
    custom: Option<f64>,
    ai: Option<ActorAi>,
    body: Option<ActorBody>,
}

impl TaskParams {
    fn empty() -> Self {
        Self {
            target: None,
            x: None,
            y: None,
            custom: None,
            ai: None,
            body: None,
        }
    }
    pub fn move_to(x: f64, y: f64) -> Self {
        let mut ret = TaskParams::empty();
        ret.x = Some(x);
        ret.y = Some(y);
        ret
    }

    pub fn move_to_actor(target: Target, max_distance: f64) -> Self {
        let mut ret = TaskParams::empty();
        ret.target = Some(target);
        ret.custom = Some(max_distance);
        ret
    }

    pub fn run_from_actor(index: usize, id: Instant) -> Self {
        let mut ret = TaskParams::empty();
        ret.target = Some(Target::new(index, id));
        ret
    }

    pub fn spawn(x_offset: f64, y_offset: f64, delay: f64, ai: ActorAi, body: ActorBody) -> Self {
        let mut ret = TaskParams::empty();
        ret.x = Some(x_offset);
        ret.y = Some(y_offset);
        ret.custom = Some(delay);
        ret.ai = Some(ai);
        ret.body = Some(body);
        ret
    }

    pub fn xy_params(&self) -> Option<(f64, f64)> {
        if self.x.is_none() || self.y.is_none() {
            None
        } else {
            Some((self.x.unwrap(), self.y.unwrap()))
        }
    }

    pub fn spawn_params(&self) -> Option<(f64, f64, f64, ActorAi, ActorBody)> {
        if self.x.is_none()
            || self.y.is_none()
            || self.custom.is_none()
            || self.ai.is_none()
            || self.body.is_none()
        {
            None
        } else {
            Some((
                self.x.unwrap(),
                self.y.unwrap(),
                self.custom.unwrap(),
                self.ai.unwrap(),
                self.body.unwrap(),
            ))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TaskType {
    Idle,
    MoveTo,
    MoveToActor,
    RunFromActor,
    Spawn,
    Explode,
}

#[derive(Debug)]
pub struct Task {
    tag: TaskType,
    params: TaskParams,
}

impl Task {
    fn new(tag: TaskType, params: TaskParams) -> Self {
        Self { tag, params }
    }

    pub fn idle() -> Self {
        Task::new(TaskType::Idle, TaskParams::empty())
    }

    pub fn move_to(x: f64, y: f64) -> Self {
        Task::new(TaskType::MoveTo, TaskParams::move_to(x, y))
    }

    pub fn move_to_actor(target: Target, max_distance: f64) -> Self {
        Task::new(
            TaskType::MoveToActor,
            TaskParams::move_to_actor(target, max_distance),
        )
    }

    pub fn run_from(index: usize, id: Instant) -> Self {
        Task::new(
            TaskType::RunFromActor,
            TaskParams::run_from_actor(index, id),
        )
    }

    pub fn spawn(delay: f64, x_offset: f64, y_offset: f64, ai: ActorAi, body: ActorBody) -> Self {
        Task::new(
            TaskType::Spawn,
            TaskParams::spawn(x_offset, y_offset, delay, ai, body),
        )
    }

    pub fn explode() -> Self {
        Task::new(TaskType::Explode, TaskParams::empty())
    }
}

pub struct TaskCompletion {
    pub next_action: NextAction,
    pub prev_target: Option<Target>,
    pub new_actor: Option<Actor>,
    pub dead_actors: Option<Vec<usize>>,
}

impl TaskCompletion {
    pub fn new(next_action: NextAction) -> Self {
        TaskCompletion {
            next_action,
            new_actor: None,
            dead_actors: None,
            prev_target: None,
        }
    }

    pub fn ai_choice() -> Self {
        TaskCompletion::new(NextAction::AiChoice)
    }

    pub fn spawn(mut self, actor: Actor) -> Self {
        self.new_actor = Some(actor);
        self
    }

    pub fn kill(mut self, i: usize) -> Self {
        if self.dead_actors.is_none() {
            self.dead_actors = Some(vec![]);
        }
        if let Some(ref mut vec) = self.dead_actors {
            vec.push(i);
        }
        self
    }

    pub fn targeted(mut self, target: Option<Target>) -> Self {
        self.prev_target = target;
        self
    }
}

pub enum NextAction {
    Continue,
    AiChoice,
    ChangeTo(Task),
}

impl Task {
    // Returns whether the task is done executing.
    pub fn execute(
        i: usize,
        dt: f64,
        actors: &mut Vec<Actor>,
        qt: &QuadTree<ActorRef>,
    ) -> TaskCompletion {
        if i >= actors.len() {
            println!("===================");
            println!("ACTOR OUT OF BOUNDS");
            println!("===================");
            println!("i: {}", i);
            println!("dt: {}", dt);
            println!("actors: {:?}", actors);
            println!("qt: {:?}", qt);
            println!("===================");
        }
        fix_target(i, actors);
        use TaskType::*;
        if let Some(ref task) = actors[i].task {
            match task.tag {
                Idle => TaskCompletion::ai_choice(),
                MoveTo => move_to_callback(i, dt, actors, qt),
                MoveToActor => move_to_actor_callback(i, dt, actors, qt),
                RunFromActor => run_from_actor_callback(i, dt, actors, qt),
                Spawn => spawn_callback(i, dt, actors, qt),
                Explode => explode_callback(i, dt, actors, qt),
            }
        } else {
            TaskCompletion::ai_choice()
        }
    }

    pub fn description(i: usize, actors: &Vec<Actor>) -> String {
        if let Some(ref task) = actors[i].task {
            use TaskType::*;
            match task.tag {
                Idle => "doing nothing".into(),
                MoveTo => {
                    if let Some((x, y)) = task.params.xy_params() {
                        format!("moving to ({}, {})", x.round(), y.round())
                    } else {
                        "moving".into()
                    }
                }
                MoveToActor => {
                    if let Some(ref target) = task.params.target {
                        if let Some(index) = target.index {
                            format!(
                                "chasing {}",
                                match &actors[index].name {
                                    Some(name) => name,
                                    _ => "",
                                }
                            )
                        } else {
                            "chasing".into()
                        }
                    } else {
                        "chasing".into()
                    }
                }
                RunFromActor => format!(
                    "running from {}",
                    "TODO" // match &actors[params.target.index].name {
                           //     Some(name) => name,
                           //     _ => "",
                           // }
                ),
                Spawn => {
                    if let Some((_, _, delay, ai, _)) = task.params.spawn_params() {
                        format!("spawning a {} in {} seconds", ai, delay.floor())
                    } else {
                        "spawning".into()
                    }
                }
                Explode => "exploding".into(),
            }
        } else {
            Default::default()
        }
    }

    pub fn get_target_index(&self) -> Option<usize> {
        if let Some(ref target) = self.params.target {
            if let Some(index) = target.index {
                return Some(index);
            }
        }
        None
    }
}

/// Look backwards through the actors array starting at index `t - 1` for one
/// with the given id, then return the new index.
fn fix_target(index: usize, actors: &mut Vec<Actor>) {
    let mut new_index: Option<usize> = None;
    if let Some(ref task) = actors[index].task {
        if let Some(ref target) = task.params.target {
            if let Some(mut index) = target.index {
                if index >= actors.len() {
                    index = actors.len();
                }
                loop {
                    if actors[index].id == target.id {
                        new_index = Some(index);
                        break;
                    }
                    index -= 1;
                    if index == 0 {
                        new_index = None;
                        break;
                    }
                }
            }
        }
    }
    if let Some(target_index) = new_index {
        if let Some(ref mut task) = actors[index].task {
            if let Some(ref mut target) = task.params.target {
                target.index = Some(target_index);
            }
        }
    }
}

fn move_to_callback(
    i: usize,
    dt: f64,
    actors: &mut Vec<Actor>,
    qt: &QuadTree<ActorRef>,
) -> TaskCompletion {
    let collided: Vec<ActorRef> = qt
        .query(&actors[i].get_region())
        .into_iter()
        .filter(|a| a.id != i)
        .collect();
    if let Some(ref task) = actors[i].task {
        TaskCompletion::new(if collided.len() == 0 {
            if let Some((x, y)) = task.params.xy_params() {
                if actors[i].step_towards(x, y, dt) {
                    NextAction::AiChoice
                } else {
                    NextAction::Continue
                }
            } else {
                NextAction::AiChoice
            }
        } else {
            match collided.iter().find(|a| a.id != i) {
                Some(t) => NextAction::ChangeTo(Task::run_from(t.id, actors[t.id].id)),
                None => NextAction::AiChoice,
            }
        })
    } else {
        TaskCompletion::ai_choice()
    }
}

fn move_to_actor_callback(
    i: usize,
    dt: f64,
    actors: &mut Vec<Actor>,
    _qt: &QuadTree<ActorRef>,
) -> TaskCompletion {
    let mut target_position: Option<(f64, f64)> = None;
    let mut max_distance: Option<f64> = None;
    let mut prev_target: Option<Target> = None;
    if let Some(ref task) = actors[i].task {
        if let Some(ref target) = task.params.target {
            if let Some(index) = target.index {
                target_position = Some(actors[index].get_pos());
            }
            prev_target = Some(target.clone());
        }
        max_distance = task.params.custom;
    }
    if let Some((x, y)) = target_position {
        actors[i].step_towards(x, y, dt);
        return TaskCompletion::new(
            if vector::distance_cmp(actors[i].x, actors[i].y, x, y, max_distance.unwrap_or(0.0)) {
                NextAction::AiChoice
            } else {
                NextAction::Continue
            },
        )
        .targeted(prev_target);
    }
    TaskCompletion::ai_choice()
}

fn run_from_actor_callback(
    i: usize,
    dt: f64,
    actors: &mut Vec<Actor>,
    _qt: &QuadTree<ActorRef>,
) -> TaskCompletion {
    if let Some(ref task) = actors[i].task {
        if let Some(ref target) = task.params.target {
            if let Some(index) = target.index {
                let (x, y) = actors[index].get_pos();
                actors[i].step_from(x, y, dt);
                return TaskCompletion::new(if actors[i].can_see(x, y) {
                    NextAction::Continue
                } else {
                    NextAction::AiChoice
                });
            }
        }
    }
    TaskCompletion::ai_choice()
}

/// Custom param is `delay`, i.e. how long until the spawn should execute.
fn spawn_callback(
    i: usize,
    dt: f64,
    actors: &mut Vec<Actor>,
    _qt: &QuadTree<ActorRef>,
) -> TaskCompletion {
    let mut should_spawn = false;
    if let Some(ref mut task) = actors[i].task {
        if let Some(ref mut delay) = task.params.custom {
            *delay = *delay - dt;
            if *delay <= 0.0 {
                should_spawn = true;
            }
        }
    }
    if should_spawn {
        if let Some(ref task) = actors[i].task {
            let (x, y) = actors[i].get_pos();
            if let Some((xo, yo, _, ai, body)) = task.params.spawn_params() {
                return TaskCompletion::ai_choice().spawn(Actor::new(x + xo, y + yo, body, ai));
            }
        }
    }
    TaskCompletion::new(NextAction::Continue)
}

fn explode_callback(
    i: usize,
    _dt: f64,
    actors: &mut Vec<Actor>,
    qt: &QuadTree<ActorRef>,
) -> TaskCompletion {
    let targets = qt.query(&Region::new_circle(actors[i].x, actors[i].y, 25.0));
    let mut ret = TaskCompletion::ai_choice();
    for target in targets.iter() {
        ret = ret.kill(target.id);
    }
    ret
}
