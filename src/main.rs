mod direction;
mod player;
mod enemy;
mod goal;
mod components;
mod resources;
mod systems;

use std::thread;
use std::error::Error;
use std::time::{Instant, Duration};

use rand::{Rng, thread_rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    image::{self, LoadTexture, InitFlag},
};
use specs::{World, WorldExt, Builder, DispatcherBuilder};

use crate::direction::Direction;
use crate::resources::{TimeDelta, KeyboardEvent, GameStatus};
use crate::components::{BoundingBox, Velocity, Sprite, Player, Enemy, Goal};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the SDL2 library
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. We have to
    // have this variable because if we just called the function as is then the return value would
    // be treated as a temporary value and then dropped right away.
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Create a window with the given title and dimensions
    let window = video_subsystem.window("Minimal Game", 800, 600)
        .position_centered()
        .build()?;

    // Create a canvas that draws on the window
    let mut canvas = window.into_canvas().build()?;
    // The boundary of the window in world coordinates
    let world_bounds = {
        let (width, height) = canvas.output_size()?;
        Rect::from_center((0, 0), width, height)
    };

    // Load assets
    let texture_creator = canvas.texture_creator();
    // Store the textures in an array so that they can be referenced by index. This allows textures
    // to be shared between entities without having to copy the texture all over the place.
    let textures = [
        texture_creator.load_texture("assets/bardo_2x.png")?,
        texture_creator.load_texture("assets/reaper_blade_2x.png")?,
        texture_creator.load_texture("assets/pinktrees_2x.png")?,
    ];
    let bardo_texture = 0;
    let reaper_texture = 1;
    let pink_trees_texture = 2;

    // Declare the hierarchy of systems that will process entities and components
    let dispatcher = DispatcherBuilder::new()
        .with(systems::Keyboard, "Keyboard", &[])
        .with(systems::AI, "AI", &[])
        .with(systems::Movement {world_bounds}, "Movement", &["Keyboard", "AI"])
        .with(systems::WinLoseChecker, "WinLoseChecker", &["Movement"])
        .with(systems::Animator, "Animator", &["Keyboard", "AI"])
        .build();

    // Game state
    let mut world = World::new();
    // Setup the component storages based on the data used by the systems
    dispatcher.setup(&mut world);

    let mut rng = thread_rng();

    world.create_entity()
        .with(Goal)
        .with(BoundingBox(Rect::from_center((rng.gen_range(-300, 301), -200), 92, 116)))
        .with(Sprite {
            texture_id: pink_trees_texture,
            region: Rect::new(0, 0, 128, 128),
        })
        .build();

    world.create_entity()
        .with(Player {movement_speed: 200})
        .with(BoundingBox(Rect::from_center((rng.gen_range(-320, 321), 250), 32, 58)))
        .with(Velocity {speed: 0, direction: Direction::Down})
        .with(Sprite {
            texture_id: bardo_texture,
            region: Rect::new(0, 0, 52, 72),
        })
        .build();

    // Generate enemies in random positions. To avoid overlap with anything else, an area of the
    // world coordinate system is divided up into a 2D grid. Each enemy gets a random position
    // within one of the cells of that grid.
    for i in -1..2 {
        for j in -2..0 {
            let enemy_pos = Point::new(
                i * 200 + rng.gen_range(-80, 80),
                j * 140 + 200 + rng.gen_range(-40, 40),
            );
            let enemy_dir = match rng.gen_range(0, 4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => unreachable!(),
            };

            world.create_entity()
                .with(Enemy {
                    direction_timer: Instant::now(),
                    direction_change_delay: Duration::from_millis(200),
                })
                .with(BoundingBox(Rect::from_center(enemy_pos, 50, 58)))
                .with(Velocity {speed: 200, direction: enemy_dir})
                .with(Sprite {
                    texture_id: reaper_texture,
                    region: Rect::new(0, 0, 52, 72),
                })
                .build();
        }
    }

    // Add resources (resources used with ReadExpect/WriteExpect must be added before use)
    world.insert(TimeDelta::default());
    world.insert(GameStatus::Running);

    // Begin game loop
    let frame_duration = Duration::from_nanos(1_000_000_000 / 60);
    let mut event_pump = sdl_context.event_pump()?;
    // A labelled loop can be used with `break` even from inside another loop
    'running: loop {
        // HANDLE EVENTS

        // Handle all of the events available right now
        let mut keyboard_event = None;
        for event in event_pump.poll_iter() {
            match event {
                // Quit the game if the window is closed or if the escape key is pressed
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Set the player direction and speed based on the arrow key that is pressed
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    keyboard_event = Some(KeyboardEvent::MoveInDirection(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard_event = Some(KeyboardEvent::MoveInDirection(Direction::Down));
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    keyboard_event = Some(KeyboardEvent::MoveInDirection(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    keyboard_event = Some(KeyboardEvent::MoveInDirection(Direction::Right));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard_event = Some(KeyboardEvent::Stop);
                },
                _ => {}
            }
        }
        // Inform the systems of the keyboard event
        world.insert(keyboard_event);

        // UPDATE

        // Store the time elapsed since the last frame in a resource so that all systems may have
        // access to it. This is mostly just done for illustration purposes since we technically
        // do not need to repeatedly set the value over and over again if it is constant.
        *world.write_resource() = TimeDelta(frame_duration);

        // Update game state
        dispatcher.dispatch(&world);
        // Apply any lazy updates that occurred during dispatch
        world.maintain();

        // Check if we need to quit the game
        match *world.read_resource() {
            GameStatus::Running => {}, // Keep going
            GameStatus::Win => {
                println!("You win!");
                break;
            },
            GameStatus::Lose => {
                println!("You lose!");
                break;
            },
        }

        // RENDER

        // Draw the game onto the screen
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.clear();

        player.render(&mut canvas, &textures)?;
        for enemy in &enemies {
            enemy.render(&mut canvas, &textures)?;
        }
        goal.render(&mut canvas, &textures)?;

        canvas.present();

        // LIMIT FRAMERATE

        // Manage the timing of the game so that the loop doesn't go too quickly or too slowly.
        //
        // Time stepping is a complex topic. We're simplifying things by just always assuming that
        // 1/60 seconds has passed in each iteration of the loop. 1/60th of a second is 60 FPS.
        // There are *many* downsides to the code as it is below, but it's good enough as a
        // starting point.
        //
        // For more information and some more robust approaches:
        // * http://web.archive.org/web/20190506122532/http://gafferongames.com/post/fix_your_timestep/
        // * https://www.gamasutra.com/blogs/BramStolk/20160408/269988/Fixing_your_time_step_the_easy_way_with_the_golden_48537_ms.php
        thread::sleep(frame_duration);
    }

    Ok(())
}
