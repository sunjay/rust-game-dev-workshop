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
        // This is different from the size of the sprite because we only want the visible region,
        // not any surrounding transparent pixels

        //TODO(EX#N3): Measure the visible size of the goal sprite (using any image editor) and
        // return a `Rect` centered at `self.position`
        // HINT: Search for `center` on docs.rs/sdl2. Is there a method on `Rect` that helps here?
        Rect::new(0, 0, 128, 128)
    }

    /// Draw the goal onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        #![allow(unused_variables)] //TODO(EX#N3): Remove this line

        let (sprite_x, sprite_y) = (0, 0);
        let (sprite_width, sprite_height) = (128, 128);
        let sprite_rect = Rect::new(sprite_x, sprite_y, sprite_width, sprite_height);

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let (width, height) = canvas.output_size()?;
        //TODO(EX#N3): Change the next line so that `self.position` goes from the world coordinate
        // system to the screen coordinate system.
        // HINT: Try a few examples on a piece of paper or in a drawing app on your phone.
        //  Use concrete numbers to help you get an idea of how to do this.
        // BONUS: What would this code look like if you wanted to convert a standard cartesian
        //  coordinate system to screen coordinates? Walk one of the people leading the workshop
        //  through your solution and see if you can convince them that you're correct. It is still
        //  recommended that you stay with the simpler world coordinate system for now, but this
        //  might be a great thing to integrate into your own games that you make after today!
        let screen_pos = self.position;
        let screen_rect = Rect::from_center(screen_pos, sprite_width, sprite_height);

        // Copy the sprite onto the canvas
        //TODO(EX#N3): Pass `sprite_rect` and `screen_rect` into this function in the right order.
        // HINT: Look up the documentation for `Canvas` in the `sdl2` crate and look at the
        //  coumentation for the `copy` method.
        canvas.copy(&textures[self.texture], Rect::new(0, 0, 1, 1), Rect::new(0, 0, 1, 1))?;

        Ok(())
    }
}
