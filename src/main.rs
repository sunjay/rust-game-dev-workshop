mod direction;
mod world;
mod player;
mod enemy;
mod goal;

use std::thread;
use std::error::Error;
use std::time::Duration;

use sdl2::{
    pixels::Color,
    rect::Rect,
    image::{self, LoadTexture, InitFlag},
};

use crate::world::{World, GameOver};

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
    // Store the textures in a Vec so that they can be referenced by index. This allows textures
    // to be shared between entities without having to copy the texture all over the place.
    let mut textures = Vec::new();
    let load_texture = |path| {
        texture_creator.load_texture(path)?;
        Ok(textures.len() - 1)
    };

    // Game state
    let world = World::new(load_texture);

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
        // Handle all of the events available right now
        for event in event_pump.poll_iter() {
            world.handle_event(&mut event_pump, &mut world.player);
        }

        // Update game state
        player.update(frame_duration, world_bounds);
        for enemy in &mut enemies {
            enemy.update(frame_duration, world_bounds);
        }
        if enemies.iter().any(|enemy| player.collides_with(enemy.bounding_box())) {
            println!("You lose!");
            break;
        }
        if player.collides_with(goal.bounding_box()) {
            println!("You win!");
            break;
        }

        // Draw the game onto the screen
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.clear();

        player.render(&mut canvas, &textures)?;
        for enemy in &enemies {
            enemy.render(&mut canvas, &textures)?;
        }
        goal.render(&mut canvas, &textures)?;

        canvas.present();

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
