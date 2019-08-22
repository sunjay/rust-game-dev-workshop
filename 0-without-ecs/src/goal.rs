#![allow(dead_code, unused_variables)] //TODO(EX#1): remove this line

use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

pub struct Goal {
    /// The position of the goal in world coordinates
    position: Point,
    /// The texture containing the goal spritesheet
    texture: usize,
}

impl Goal {
    /// Creates a new goal
    pub fn new(position: Point, texture: usize) -> Self {
        Self {
            position,
            texture,
        }
    }

    /// Returns a rectangle that tightly encompasses the goal in the world coordinate system
    pub fn bounding_box(&self) -> Rect {
        //TODO(EX#1): Return a rectangle that tightly surrounds `self.position` and only contains
        // the visible portion of the sprite. You will need to measure the sprite dimensions.
        Rect::from_center(self.position, 92, 116)
    }

    /// Draw the goal onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        //TODO(EX#1): Copy and paste the render method body from either player.rs or enemy.rs and
        // make the necessary adjustments to render the goal texture.
        // HINT: If there is no animation, do you still need all the same calculations?
        let (sprite_width, sprite_height) = (128, 128);
        let (sprite_x, sprite_y) = (0, 0);
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
