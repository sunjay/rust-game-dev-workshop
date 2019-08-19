use sdl2::rect::Rect;
use specs::{System, SystemData, ReadExpect, ReadStorage, WriteStorage, Join, World, prelude::ResourceId};

use crate::resources::TimeDelta;
use crate::components::{BoundingBox, Velocity};

pub struct Movement {
    world_bounds: Rect,
}

/// Data from the world required by the system
#[derive(SystemData)]
pub struct MovementData<'a> {
    velocities: ReadStorage<'a, Velocity>,
    bounding_boxes: WriteStorage<'a, BoundingBox>,
    time_delta: ReadExpect<'a, TimeDelta>,
}

impl<'a> System<'a> for Movement {
    type SystemData = MovementData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let MovementData {velocities, bounding_boxes, time_delta} = data;
        let TimeDelta(time_elapsed) = *time_delta;

        for (&Velocity {speed, direction}, BoundingBox(bounds)) in (&velocities, &mut bounding_boxes).join() {
            // No need to update position if not moving
            if speed == 0 {
                continue;
            }

            // Compute the distance (in pixels) traversed during the time elapsed.
            // Notes:
            // * There are 1000000 microseconds in 1 second.
            // * We do the division at the end so rounding due to integer division occurs last.
            // * The conversion to i32 is only safe because we assume that the number of
            //   microseconds in time_elapsed fits within the range of i32.
            let distance = speed * time_elapsed.as_micros() as i32 / 1_000_000;

            // Move in the current direction
            let new_pos = bounds.center() + direction.into_point() * distance;
            let new_bounds = Rect::from_center(new_pos, bounds.width(), bounds.height());

            // Disallow entities from leaving the window
            if self.world_bounds.contains_rect(new_bounds) {
                *bounds = new_bounds;
            }
        }
    }
}
