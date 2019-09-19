//! The renderer cannot be a normal system because it holds values that must be used on the main
//! thread. It cannot be executed in parallel like other systems. Specs provides a mechanism called
//! a thread local system that helps address this. Unfortunately there is also another complication
//! which is that the renderer system returns a `Result` whereas normal systems do not return
//! anything. You would have to use a resource or a channel to completely integrate this into specs

#![allow(dead_code, unused_variables, unused_imports)] //TODO(BONUS#2): remove this line

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

        //TODO(BONUS#2): Copy the code from the render() function of goal.rs, player.rs, or enemy.rs
        // and then adapt it to work in this function
        for (&BoundingBox(bounds), &Sprite {texture_id, region: sprite_rect}) in (bounding_boxes, sprites).join() {
            //TODO(BONUS#2): Figure out how to render given the bounding box, texture_id, and
            // sprite_rect.
            // HINT: How do you determine the position based on the bounding box?
            //  Go to the sdl2 documentation and look up `Rect`.
        }

        Ok(())
    }
}
