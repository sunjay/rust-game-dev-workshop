mod player;
mod goal;

use std::thread;
use std::error::Error;
use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
    image::{self, LoadTexture, InitFlag},
};

use crate::player::{Direction, Player};
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
        texture_creator.load_texture("assets/pinktrees_2x.png")?,
    ];
    let bardo_texture = 0;
    let pink_trees_texture = 1;

    // Game state
    let mut player = Player::new(Point::new(0, 0), bardo_texture);
    let goal = Goal::new(Point::new(0, -200), pink_trees_texture);

    let frame_duration = Duration::from_nanos(1_000_000_000 / 60);
    let mut event_pump = sdl_context.event_pump()?;
    // A labelled loop can be used with `break` even from inside another loop
    'running: loop {
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
                    player.walk_in_direction(Direction::Down);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.walk_in_direction(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.walk_in_direction(Direction::Right);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.stop();
                },
                _ => {}
            }
        }

        // Update game state
        player.update(frame_duration);

        // Draw the game onto the screen
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.clear();

        player.render(&mut canvas, &textures)?;
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
