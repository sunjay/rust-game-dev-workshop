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

pub struct Player<'r> {
    /// The position of the player in world coordinates
    position: Point,
    /// The texture containing the player spritesheet
    texture: Texture<'r>,
    /// The speed of the player's movement in pixels/second (0 = stopped)
    speed: i32,
    /// The direction of the player's movement
    direction: Direction,
}

impl<'r> Player<'r> {
    /// Creates a new player
    pub fn new(texture: Texture<'r>) -> Self {
        Self {
            position: Point::new(0, 0),
            texture,
            speed: 0,
            direction: Direction::Down,
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

        // There are 1000000 microseconds in 1 second.
        // Note that we do the division at the end so rounding due to integer division occurs last.
        // The conversion to i32 is only safe because we assume that the number of microseconds in
        // time_elapsed fits within the range of i32.
        let direction = self.direction.into_point();
        self.position += direction * self.speed * time_elapsed.as_micros() as i32 / 1_000_000;
    }

    /// Draw the player onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let screen_pos = self.position + Point::new((width/2) as i32, (height/2) as i32);

        // Copy the current frame onto the canvas
        canvas.copy(
            &self.texture,
            Rect::new(0, 0, 52, 72),
            Rect::from_center(screen_pos, 52, 72)
        )?;

        Ok(())
    }
}
