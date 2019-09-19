mod direction;
mod player;
mod enemy;
mod goal;

use std::thread;
use std::error::Error;
use std::time::Duration;

use rand::{Rng, thread_rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    image::{self, LoadTexture, InitFlag},
};

use crate::direction::Direction;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::goal::Goal;

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

    // Game state
    let mut rng = thread_rng();
    let goal = Goal::new(Point::new(rng.gen_range(-300, 301), -200), pink_trees_texture);
    let mut player = Player::new(Point::new(rng.gen_range(-320, 321), 250), bardo_texture);

    // Generate enemies in random positions. To avoid overlap with anything else, an area of the
    // world coordinate system is divided up into a 2D grid. Each enemy gets a random position
    // within one of the cells of that grid.
    let mut enemies = Vec::new();
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
            enemies.push(Enemy::new(enemy_pos, enemy_dir, reaper_texture));
        }
    }

    // Begin game loop
    let frame_duration = Duration::from_nanos(1_000_000_000 / 60);
    // The boundary of the window in world coordinates
    let world_bounds = {
        let (width, height) = canvas.output_size()?;
        Rect::from_center((0, 0), width, height)
    };
    let mut event_pump = sdl_context.event_pump()?;
    // A labelled loop can be used with `break` even from inside another loop
    'running: loop {
        // HANDLE EVENTS

        // Handle all of the events available right now
        for event in event_pump.poll_iter() {
            match event {
                // Quit the game if the window is closed or if the escape key is pressed
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                // Set the player direction and speed based on the arrow key that is pressed
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.walk_in_direction(Direction::Up);
                },

                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    //TODO(EX#1): Fill in this line
                    // HINT: This should be very similar to what was done in the Keycode::Up case
                    //  above.
                },

                //TODO(EX#1): Add two more cases for the left arrow and the right arrow

                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    //TODO(EX#1): Fill in this line.
                    // HINT: Look at the methods on the player struct. What should happen when the
                    //  player releases the arrow key they had previously pressed down?
                },

                _ => {}
            }
        }

        // UPDATE

        // Update game state
        player.update(frame_duration, world_bounds);
        for enemy in &mut enemies {
            enemy.update(frame_duration, world_bounds);
        }
        // If the player collides with any enemies, quit the game immediately
        if enemies.iter().any(|enemy| player.collides_with(enemy.bounding_box())) {
            println!("You lose!");
            break;
        }
        // If the player reaches the goal, they win
        if player.collides_with(goal.bounding_box()) {
            println!("You win!");
            break;
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
