use std::time::Duration;

use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

/// The direction of motion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns a point that represents this direction in world coordinates
    pub fn into_point(self) -> Point {
        use Direction::*;
        match self {
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
            Left => Point::new(-1, 0),
            Right => Point::new(1, 0),
        }
    }
}

pub struct Player {
    /// The position of the player in world coordinates
    position: Point,
    /// The texture containing the player spritesheet
    texture: usize,
    /// The speed of the player's movement in pixels/second (0 = stopped)
    speed: i32,
    /// The direction of the player's movement
    direction: Direction,
    /// The current animation frame for the player's walking animation
    frame: i32,
}

impl Player {
    /// Creates a new player
    pub fn new(texture: usize) -> Self {
        Self {
            position: Point::new(0, 0),
            texture,
            speed: 0,
            direction: Direction::Down,
            frame: 0,
        }
    }

    /// Set the player in motion in the given direction
    pub fn walk_in_direction(&mut self, direction: Direction) {
        self.speed = 200;
        self.direction = direction;
    }

    /// Stop the player's movement but preserve their direction
    pub fn stop(&mut self) {
        self.speed = 0;
    }

    /// Update the player's state
    pub fn update(&mut self, time_elapsed: Duration) {
        if self.speed == 0 {
            return;
        }

        // Compute the distance (in pixels) traversed during the time elapsed.
        // Notes:
        // * There are 1000000 microseconds in 1 second.
        // * We do the division at the end so rounding due to integer division occurs last.
        // * The conversion to i32 is only safe because we assume that the number of microseconds
        //   in time_elapsed fits within the range of i32.
        let distance = self.speed * time_elapsed.as_micros() as i32 / 1_000_000;

        // Move in the current direction
        self.position += self.direction.into_point() * distance;

        // Advance the walking animation (only want to do this when speed != 0)
        // Note that this code assumes that ALL walking animations have 3 frames.
        self.frame = (self.frame + 1) % 3;
    }

    /// Draw the player onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        let (sprite_width, sprite_height) = (52, 72);
        let sprite_x = self.frame * sprite_width;
        let spritesheet_row = match self.direction {
            Direction::Up => 3,
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 2,
        };
        let sprite_y = spritesheet_row * sprite_height;

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let (width, height) = canvas.output_size()?;
        let screen_pos = self.position + Point::new((width/2) as i32, (height/2) as i32);

        // Copy the current frame onto the canvas
        canvas.copy(
            &textures[self.texture],
            Rect::new(sprite_x, sprite_y, sprite_width as u32, sprite_height as u32),
            Rect::from_center(screen_pos, sprite_width as u32, sprite_height as u32),
        )?;

        Ok(())
    }
}
