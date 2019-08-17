use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
};

#[derive(Debug)]
pub struct Player {
    /// The position of the player in world coordinates
    position: Point,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
        }
    }
}

impl Player {
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;

        // The screen coordinate system has (0, 0) in its top-left corner whereas the
        // world coordinate system has (0, 0) in the center of the screen.
        let screen_pos = self.position + Point::new((width/2) as i32, (height/2) as i32);

        //TODO: This is just a placeholder until sprites are added
        canvas.set_draw_color((130, 179, 207));
        canvas.fill_rect(Rect::from_center(screen_pos, 30, 30))?;

        Ok(())
    }
}
