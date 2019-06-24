extern crate graphics;
extern crate names;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use graphics::character::CharacterCache;
use graphics::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;

extern crate float_cmp;

use float_cmp::ApproxEq;
use names::Generator;
use rand::Rng;

mod world;
use world::map::Map;
use world::*;

mod ui;
use ui::*;

mod qt;
use qt::*;

const WINDOW_DEFAULT_WIDTH: f64 = 1024.0;
const WINDOW_DEFAULT_HEIGHT: f64 = 768.0;

const MIN_SCALE: f64 = 0.4;
const MAX_SCALE: f64 = 5.0;

struct Game<'a, 'b, C: CharacterCache> {
    map: Map,
    actors: Vec<Actor>,
    hovered_actor: Option<usize>,
    selected_actor: Option<usize>,
    mouse: MouseDetails,
    offset: WorldOffset,
    ui: GUI<'b, C>,
    name_generator: Generator<'a>,
}

impl<'a, 'b, C: CharacterCache> Game<'a, 'b, C> {
    pub fn new(font: &'b mut C) -> Self {
        let width = 80;
        let height = 100;
        let mut game = Self {
            map: Map::new(width, height),
            actors: vec![],
            hovered_actor: None,
            selected_actor: None,
            mouse: MouseDetails::new(),
            offset: WorldOffset::new(),
            ui: GUI::new(WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT, font),
            name_generator: Generator::default(),
        };
        game.center_on(
            width / 2,
            height / 2,
            WINDOW_DEFAULT_WIDTH,
            WINDOW_DEFAULT_HEIGHT,
        );
        // TODO: Currently just adds 10 generic actors. Long-term remove this.
        let bounds = game.map.get_bounds();
        let (x_dim, y_dim) = (bounds.w, bounds.h);
        for _ in 0..10 {
            let x: f64 = rand::thread_rng().gen();
            let y: f64 = rand::thread_rng().gen();
            game.add_actor(x * x_dim, y * y_dim, ActorBody::Worker, ActorAi::Wanderer);
        }
        game.add_actor(50.0, 800.0, ActorBody::Worker, ActorAi::Kamikaze);
        game.add_actor(50.0, 50.0, ActorBody::Worker, ActorAi::Kamikaze);
        game.add_actor(800.0, 50.0, ActorBody::Worker, ActorAi::Kamikaze);
        game.add_actor(800.0, 800.0, ActorBody::Worker, ActorAi::Kamikaze);

        game.add_actor(
            300.0,
            300.0,
            ActorBody::Building,
            ActorAi::Spawner { rate: 5.0 },
        );

        game
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let qt = self.build_quadtree();
        let mut results = Actor::update_all(args.dt, &mut self.actors, &qt, &self.map.get_bounds());
        if !results.dead_actors.is_empty() {
            results.dead_actors.sort();
            for dead_actor_index in results.dead_actors.into_iter().rev() {
                if let Some(selected_actor_index) = self.selected_actor {
                    if selected_actor_index == dead_actor_index {
                        // If the selected actor is dead, just remove the
                        // reference.
                        self.selected_actor = None;
                    } else if selected_actor_index > dead_actor_index {
                        // Every time we delete something earlier in the array,
                        // move our reference back one.
                        self.selected_actor = Some(selected_actor_index - 1);
                    }
                }
                self.actors.remove(dead_actor_index);
            }
        }
        for mut actor in results.new_actors.drain(..) {
            actor.name = self.get_name();
            self.actors.push(actor);
        }
        self.find_hovered_actor(&qt);
        self.update_ui();
    }

    pub fn render<G>(&mut self, c: Context, g: &mut G)
    where
        G: Graphics<Texture = <C as character::CharacterCache>::Texture>,
    {
        clear(color::hex("003333"), g);
        let world_transform = c
            .transform
            .trans(self.offset.h, self.offset.v)
            .scale(self.offset.scaling_factor, self.offset.scaling_factor);
        let (mouse_x, mouse_y) = self.offset.to_local_pixel(self.mouse.x, self.mouse.y);
        self.map.render(world_transform, g, mouse_x, mouse_y);
        for actor in self.actors.iter() {
            actor.render(world_transform, g);
        }
        if let Some(actor_index) = self.hovered_actor {
            self.actors[actor_index].render_extras(&self.actors, world_transform, g);
        }
        if let Some(actor_index) = self.selected_actor {
            self.actors[actor_index].render_extras(&self.actors, world_transform, g);
        }
        self.ui.render(c, g);
    }

    pub fn mouse_at(&mut self, x: f64, y: f64) {
        // Piston gives us a single frame of 0.0, 0.0 when re-entering the
        // window that we have to ignore.
        if x.approx_eq(0.0, (0.0, 2)) && y.approx_eq(0.0, (0.0, 2)) {
            return;
        }
        self.mouse.set_pos(x, y);
        if self.mouse.pressed {
            let (dx, dy) = self.mouse.pos_diff();
            self.offset.slide(dx, dy);
        }
    }

    pub fn mouse_down(&mut self) {
        self.mouse.save_pos();
        self.mouse.pressed = true;
    }

    pub fn mouse_up(&mut self) {
        if self.mouse.barely_moved() {
            // TODO: Handle click event for real.
            let qt = self.build_quadtree();
            self.set_selected_actor(&qt);
        }
        self.mouse.pressed = false;
    }

    pub fn mouse_scroll(&mut self, up: bool) {
        self.offset.zoom(up, self.mouse.x, self.mouse.y);
    }

    pub fn resize(&mut self, w: f64, h: f64) {
        self.ui.resize(w, h);
    }

    fn find_hovered_actor(&mut self, qt: &QuadTree<ActorRef>) {
        let (mouse_x, mouse_y) = self.offset.to_local_pixel(self.mouse.x, self.mouse.y);
        let region = Region::new_point(mouse_x, mouse_y);
        let results = qt.query(&region);
        if results.is_empty() {
            // If we're not intersescting anything.
            self.hovered_actor = None;
        } else {
            if let Some(new_hovered) = results.first().map(|actor_ref| actor_ref.id) {
                self.hovered_actor = Some(new_hovered);
            }
        }
    }

    fn set_selected_actor(&mut self, qt: &QuadTree<ActorRef>) {
        let (mouse_x, mouse_y) = self.offset.to_local_pixel(self.mouse.x, self.mouse.y);
        let region = Region::new_point(mouse_x, mouse_y);
        let results = qt.query(&region);
        if results.is_empty() {
            self.selected_actor = None;
        } else {
            if let Some(new_selected) = results.first().map(|actor_ref| actor_ref.id) {
                self.selected_actor = Some(new_selected);
            }
        }
    }

    fn build_quadtree(&self) -> QuadTree<ActorRef> {
        let bounds = self.map.get_bounds();
        let mut qt: QuadTree<ActorRef> =
            QuadTree::new(RectangleData::new(bounds.x, bounds.y, bounds.w, bounds.h));
        for (i, actor) in self.actors.iter().enumerate() {
            qt.insert(actor.get_ref(i));
        }
        qt
    }

    fn center_on(&mut self, x: usize, y: usize, screen_width: f64, screen_height: f64) {
        let (local_x, local_y) = self.map.get_cell_loc(x, y);
        let (global_x, global_y) = self.offset.to_global_pixel(local_x, local_y);

        let target_x = screen_width / 2.0;
        let target_y = screen_height / 2.0;

        self.offset.slide(target_x - global_x, target_y - global_y);
    }

    fn add_actor(&mut self, x: f64, y: f64, body: ActorBody, ai: ActorAi) {
        let mut new_actor = Actor::new(x, y, body, ai);
        new_actor.name = self.get_name();
        self.actors.push(new_actor);
    }

    fn get_name(&mut self) -> Option<String> {
        self.name_generator.next()
    }

    fn update_ui(&mut self) {
        if let Some(selected) = self.selected_actor {
            self.ui
                .selected_desc(Actor::description(selected, &self.actors).as_str());
        } else {
            self.ui.selected_desc("");
        }
        if let Some(hovered) = self.hovered_actor {
            self.ui
                .hovered_desc(Actor::description(hovered, &self.actors).as_str());
        } else {
            self.ui.hovered_desc("");
        }
        self.ui.mouse_pos(self.mouse.x, self.mouse.y);
    }
}

pub struct WorldOffset {
    v: f64,              // vertical offset
    h: f64,              // horizontal offset
    scaling_factor: f64, // 1.0 means render 1:1.
}

impl WorldOffset {
    pub fn new() -> Self {
        WorldOffset {
            v: 0.0,
            h: 0.0,
            scaling_factor: 1.0,
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
        let old_scaling_factor = self.scaling_factor;
        self.scaling_factor *= zoom_ratio;
        if self.scaling_factor < MIN_SCALE {
            self.scaling_factor = MIN_SCALE;
            zoom_ratio = self.scaling_factor / old_scaling_factor;
        } else if self.scaling_factor > MAX_SCALE {
            self.scaling_factor = MAX_SCALE;
            zoom_ratio = self.scaling_factor / old_scaling_factor;
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

    pub fn to_global_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (x * self.scaling_factor) + self.h,
            (y * self.scaling_factor) + self.v,
        )
    }

    pub fn to_local_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (x - self.h) / self.scaling_factor,
            (y - self.v) / self.scaling_factor,
        )
    }
}

#[derive(Debug)]
struct MouseDetails {
    x: f64,
    y: f64,
    pressed: bool,
    prev_pos: Option<(f64, f64)>,
}

impl MouseDetails {
    pub fn new() -> Self {
        MouseDetails {
            x: Default::default(),
            y: Default::default(),
            pressed: false,
            prev_pos: None,
        }
    }

    pub fn barely_moved(&self) -> bool {
        if let Some((x, y)) = self.prev_pos {
            x.approx_eq(self.x, (0.0, 2)) && y.approx_eq(self.y, (0.0, 2))
        } else {
            false
        }
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.save_pos();
        self.x = x;
        self.y = y;
    }

    pub fn save_pos(&mut self) {
        self.prev_pos = Some((self.x, self.y));
    }

    pub fn pos_diff(&self) -> (f64, f64) {
        if let Some((x, y)) = self.prev_pos {
            (self.x - x, self.y - y)
        } else {
            (0.0, 0.0)
        }
    }
}

fn main() {
    let ref mut window: Sdl2Window = WindowSettings::new(
        "Simulation Thing",
        (WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT),
    )
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();

    let mut glyph_cache =
        GlyphCache::new("assets/OpenSans-Regular.ttf", (), TextureSettings::new()).unwrap();

    let mut gl = GlGraphics::new(OpenGL::V4_5);
    let mut game = Game::new(&mut glyph_cache);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(window) {
        e.update(|args| {
            game.update(args);
        });

        e.render(|args| {
            gl.draw(args.viewport(), |c, g| {
                game.render(c, g);
            })
        });

        e.mouse_scroll(|args| {
            game.mouse_scroll(if args[1] > 0.0 { true } else { false });
        });

        e.mouse_cursor(|args| {
            game.mouse_at(args[0], args[1]);
        });

        e.press(|args| match args {
            Button::Mouse(MouseButton::Left) => {
                game.mouse_down();
            }
            _ => {}
        });

        e.release(|args| match args {
            Button::Mouse(MouseButton::Left) => {
                game.mouse_up();
            }
            _ => {}
        });

        e.resize(|args| {
            game.resize(args.window_size[0], args.window_size[1]);
        });
    }
}
