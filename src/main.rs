use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::{
    Button, EventLoop, EventSettings, Events, MouseButton, MouseCursorEvent, MouseScrollEvent,
    PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, WindowSettings,
};
use sdl2_window::Sdl2Window;
use specs::{Builder, DispatcherBuilder, World};
use viewport::Viewport;

const WINDOW_DEFAULT_WIDTH: f64 = 1024.0;
const WINDOW_DEFAULT_HEIGHT: f64 = 768.0;

mod components;
use components::*;
mod systems;
use systems::*;

mod math;

mod map;
use map::Map;
mod offset;
use offset::WorldOffset;
mod input;
use input::Input;

fn main() {
    // PISTON SETUP
    let ref mut window: Sdl2Window = WindowSettings::new(
        "Simulation Thing",
        (WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT),
    )
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();

    let gc = GlyphCache::new("assets/OpenSans-Regular.ttf", (), TextureSettings::new()).unwrap();

    let gl = GlGraphics::new(OpenGL::V4_5);
    let mut events = Events::new(EventSettings::new().ups(30));

    // SPECS SETUP
    let mut world = World::new();
    world.add_resource(DeltaTime::default());
    world.add_resource(Viewport {
        rect: Default::default(),
        draw_size: Default::default(),
        window_size: Default::default(),
    });
    world.add_resource(Map::new(60, 60));
    world.add_resource(WorldOffset::new());
    world.add_resource(Input::default());

    let mut update_dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(TargetingSystem, "targeting", &["input"])
        .with(MotionSystem, "motion", &["targeting"])
        .build();
    update_dispatcher.setup(&mut world.res);
    let mut render_dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem { gl, gc })
        .build();
    render_dispatcher.setup(&mut world.res);

    create_entities(&mut world);

    // GAME LOOP
    while let Some(event) = events.next(window) {
        event.update(|args| {
            {
                let mut dt = world.write_resource::<DeltaTime>();
                *dt = args.dt.into();
            }
            update_dispatcher.dispatch(&mut world.res);
            world.maintain();
        });

        event.render(|args| {
            {
                let mut viewport = world.write_resource::<Viewport>();
                *viewport = args.viewport();
            }
            render_dispatcher.dispatch(&mut world.res);
        });

        event.mouse_cursor(|args| {
            let mut input = world.write_resource::<Input>();
            input.mouse_pos(args[0], args[1]);
        });

        event.mouse_scroll(|args| {
            let mut input = world.write_resource::<Input>();
            input.mouse_scroll(args[1] > 0.0);
        });

        event.press(|args| {
            let mut input = world.write_resource::<Input>();
            match args {
                Button::Mouse(MouseButton::Left) => input.mouse_down(),
                _ => (),
            }
        });

        event.release(|args| {
            let mut input = world.write_resource::<Input>();
            match args {
                Button::Mouse(MouseButton::Left) => input.mouse_up(),
                _ => (),
            }
        });
    }
}

fn create_entities(world: &mut World) {
    let e1 = world
        .create_entity()
        .with(Position { x: 50.0, y: 50.0 })
        .with(Mobility::Omnidirectional { speed: 10.0 })
        .with(TargetLocation {
            position: Position {
                x: 1000.0,
                y: 700.0,
            },
        })
        .with(Body {
            body_shape: BodyShape::Circle,
            size: 5.0,
            color: [1.0, 1.0, 1.0, 1.0],
        })
        .build();
    world
        .create_entity()
        .with(Position { x: 50.0, y: 600.0 })
        .with(Mobility::Omnidirectional { speed: 7.0 })
        .with(TargetEntity { entity: e1 })
        .with(Body {
            body_shape: BodyShape::Circle,
            size: 7.5,
            color: [0.8, 0.8, 1.0, 1.0],
        })
        .build();
}
