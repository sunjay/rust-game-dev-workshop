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
        Rect::from_center(self.position, 92, 116)
    }

    /// Returns true if the given rectangle is touching the goal
    pub fn touching(&self, rect: Rect) -> bool {
        self.bounding_box().has_intersection(rect)
    }

    /// Draw the goal onto the given canvas
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        let (sprite_x, sprite_y) = (0, 0);
        let (sprite_width, sprite_height) = (128, 128);
        let sprite_rect = Rect::new(sprite_x, sprite_y, sprite_width, sprite_height);

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let (width, height) = canvas.output_size()?;
        let screen_pos = self.position + Point::new((width/2) as i32, (height/2) as i32);
        let screen_rect = Rect::from_center(screen_pos, sprite_width, sprite_height);

        // Copy the sprite onto the canvas
        canvas.copy(&textures[self.texture], sprite_rect, screen_rect)?;

        Ok(())
    }
}
