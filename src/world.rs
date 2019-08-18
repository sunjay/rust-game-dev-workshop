use rand::{Rng, thread_rng};
use sdl2::{
    EventPump,
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

/// Signifies whether or not the game should end immediately
#[must_use] // Do not ignore this if returned from a function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOver {
    Yes,
    No,
}

pub struct World {
    player: Player,
    goal: Goal,
    enemies: Vec<Enemy>,
}

impl World {
    /// Randomly generate a new world
    pub fn new<F>(load_texture: F) -> Result<World, String>
        where F: FnMut(&str) -> Result<usize, String>
    {
        let bardo_texture = load_texture("assets/bardo_2x.png")?;
        let reaper_texture = load_texture("assets/reaper_blade_2x.png")?;
        let pink_trees_texture = load_texture("assets/pinktrees_2x.png")?;

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

        Ok(World {
            player,
            goal,
            enemies,
        })
    }

    pub fn handle_event(event: Event, player: &mut Player) -> GameOver {
        match event {
            // Quit the game if the window is closed or if the escape key is pressed
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return GameOver::Yes;
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

        GameOver::No
    }
}
