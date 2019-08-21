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
        unimplemented!() //TODO(EX#1)
    }

    /// Returns a rectangle that tightly encompasses the goal in the world coordinate system
    pub fn bounding_box(&self) -> Rect {
        //TODO(EX#1): Return a rectangle that tightly surrounds `self.position` and only contains
        // the visible portion of the sprite. You will need to measure the sprite dimensions.
        unimplemented!()
    }

    /// Draw the goal onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        //TODO(EX#1): Copy and paste the render method body from either player.rs or enemy.rs and
        // make the necessary adjustments to render the goal texture.
        // HINT: If there is no animation, do you still need all the same calculations?
        unimplemented!()
    }
}
