#![allow(dead_code)] //TODO(EX#1): Remove this line

use std::time::{Instant, Duration};

use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

use crate::direction::Direction;

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
    /// The amount of time elapsed since the animation frame changed
    frame_timer: Instant,
}

impl Player {
    /// Creates a new player
    pub fn new(position: Point, texture: usize) -> Self {
        Self {
            position,
            texture,
            speed: 0,
            direction: Direction::Down,
            frame: 0,
            frame_timer: Instant::now(),
        }
    }

    /// Returns a rectangle that tightly encompasses the player in the world coordinate system
    pub fn bounding_box(&self) -> Rect {
        // This is different from the size of the sprite because we only want the visible region,
        // not any surrounding transparent pixels
        Rect::from_center(self.position, 32, 58)
    }

    /// Returns true if the given rectangle is touching the player's bounding box
    pub fn collides_with(&self, rect: Rect) -> bool {
        self.bounding_box().has_intersection(rect)
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
    pub fn update(&mut self, time_elapsed: Duration, world_bounds: Rect) {
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
        let last_position = self.position;
        self.position += self.direction.into_point() * distance;
        // Disallow the player from leaving the window
        if !world_bounds.contains_rect(self.bounding_box()) {
            // Reset to the last position because that is definitely inside the world boundary
            self.position = last_position;
        }

        // Advance the walking animation (only want to do this when speed != 0)
        if self.frame_timer.elapsed() >= Duration::from_millis(150) {
            // Note that this code assumes that ALL walking animations have 3 frames.
            self.frame = (self.frame + 1) % 3;
            // Reset the frame timer
            self.frame_timer = Instant::now();
        }
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
        let sprite_rect = Rect::new(sprite_x, sprite_y, sprite_width as u32, sprite_height as u32);

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let (width, height) = canvas.output_size()?;
        let screen_pos = self.position + Point::new((width/2) as i32, (height/2) as i32);
        let screen_rect = Rect::from_center(screen_pos, sprite_width as u32, sprite_height as u32);

        // Copy the current frame onto the canvas
        canvas.copy(&textures[self.texture], sprite_rect, screen_rect)?;

        Ok(())
    }
}
