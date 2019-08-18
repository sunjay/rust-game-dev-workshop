use std::time::{Instant, Duration};

use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

use crate::direction::Direction;

pub struct Enemy {
    /// The position of the enemy in world coordinates
    position: Point,
    /// The texture containing the enemy spritesheet
    texture: usize,
    /// The speed of the enemy's movement in pixels/second (0 = stopped)
    speed: i32,
    /// The direction of the enemy's movement
    direction: Direction,
    /// The current animation frame for the enemy's walking animation
    frame: i32,
    /// The amount of time elapsed since the animation frame changed
    frame_timer: Instant,
}

impl Enemy {
    /// Creates a new enemy
    pub fn new(position: Point, direction: Direction, texture: usize) -> Self {
        Self {
            position,
            texture,
            speed: 0,
            direction,
            frame: 0,
            frame_timer: Instant::now(),
        }
    }

    /// Returns a rectangle that tightly encompasses the enemy in the world coordinate system
    pub fn bounding_box(&self) -> Rect {
        // This is different from the size of the sprite because we only want the visible region,
        // not any surrounding transparent pixels
        Rect::from_center(self.position, 50, 58)
    }

    /// Update the enemy's state
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
        if self.frame_timer.elapsed() >= Duration::from_millis(150) {
            // Note that this code assumes that ALL walking animations have 3 frames.
            self.frame = (self.frame + 1) % 3;
            // Reset the frame timer
            self.frame_timer = Instant::now();
        }
    }

    /// Draw the enemy onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        let (sprite_width, sprite_height) = (64, 72);
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
