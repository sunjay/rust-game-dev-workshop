//! The renderer cannot be a normal system because it holds values that must be used on the main
//! thread. It cannot be executed in parallel like other systems. Another complication is that it
//! returns a `Result` whereas normal systems do not return anything.

use specs::{SystemData, ReadStorage, Join, World, prelude::ResourceId};
use sdl2::{
    rect::{Point, Rect},
    render::{WindowCanvas, Texture},
};

use crate::components::{BoundingBox, Sprite};

/// Data from the world required by the renderer
#[derive(SystemData)]
pub struct RendererData<'a> {
    bounding_boxes: ReadStorage<'a, BoundingBox>,
    sprites: ReadStorage<'a, Sprite>,
}

impl<'a> RendererData<'a> {
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        let RendererData {bounding_boxes, sprites} = self;

        let (width, height) = canvas.output_size()?;
        let world_to_screen_offset = Point::new(width as i32 / 2, height as i32 / 2);

        for (&BoundingBox(bounds), &Sprite {texture_id, region: sprite_rect}) in (bounding_boxes, sprites).join() {
            let screen_pos = bounds.center() + world_to_screen_offset;
            let screen_rect = Rect::from_center(screen_pos, sprite_rect.width() as u32, sprite_rect.height() as u32);

            canvas.copy(&textures[texture_id], sprite_rect, screen_rect)?;
        }

        Ok(())
    }
}
