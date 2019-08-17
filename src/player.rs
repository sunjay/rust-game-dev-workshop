use sdl2::{
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

pub struct Player<'r> {
    /// The position of the player in world coordinates
    position: Point,
    /// The texture containing the player spritesheet
    texture: Texture<'r>,
}

impl<'r> Player<'r> {
    /// Creates a new player
    pub fn new(texture: Texture<'r>) -> Self {
        Self {
            position: Point::new(0, 0),
            texture,
        }
    }

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
