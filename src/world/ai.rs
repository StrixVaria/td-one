use rand::Rng;

use crate::world::*;

#[derive(Debug, Copy, Clone)]
pub enum ActorAi {
    Wanderer,
    Kamikaze,
    Spawner { rate: f64 },
}

impl ActorAi {
    pub fn get_task(
        i: usize,
        actors: &mut Vec<Actor>,
        prev_target: Option<Target>,
        _qt: &QuadTree<ActorRef>,
    ) -> Task {
        use ActorAi::*;
        match actors[i].ai {
            Wanderer => wanderer_callback(i, actors),
            Kamikaze => kamikaze_callback(i, actors, prev_target),
            Spawner { rate } => spawn_callback(i, actors, rate),
        }
    }
}

fn wanderer_callback(i: usize, actors: &mut Vec<Actor>) -> Task {
    let (x, y) = actors[i].get_pos();
    Task::move_to(
        x + (rand::thread_rng().gen::<f64>() - 0.5) * 25.0,
        y + (rand::thread_rng().gen::<f64>() - 0.5) * 25.0,
    )
}

fn kamikaze_callback(i: usize, actors: &mut Vec<Actor>, prev_target: Option<Target>) -> Task {
    let t = match prev_target {
        Some(target) => target,
        None => {
            let new_target = rand::thread_rng().gen_range(0, actors.len());
            Target::new(new_target, actors[new_target].id)
        }
    };
    if t.id == actors[i].id || t.index.is_none() {
        return Task::idle();
    }
    let (x, y) = actors[i].get_pos();
    let (tx, ty) = actors[t.index.unwrap()].get_pos();
    if vector::distance_cmp(x, y, tx, ty, 25.0) {
        Task::explode()
    } else {
        Task::move_to_actor(t, 20.0)
    }
}

fn spawn_callback(_i: usize, _actors: &mut Vec<Actor>, rate: f64) -> Task {
    Task::spawn(rate, 10.0, 10.0, ActorAi::Wanderer, ActorBody::Worker)
}

impl fmt::Display for ActorAi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActorAi::Wanderer => "Wanderer",
                ActorAi::Kamikaze => "Bomber",
                ActorAi::Spawner { .. } => "Spawner",
            }
        )
    }
}
