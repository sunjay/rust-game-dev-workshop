use std::time::Instant;

use specs::{System, SystemData, Entities, ReadStorage, WriteStorage, Join, World, prelude::ResourceId};

use crate::direction::Direction;
use crate::components::{Velocity, Animation, Sprite, MovementAnimations};

pub struct Animator;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct AnimatorData<'a> {
    entities: Entities<'a>,
    velocities: ReadStorage<'a, Velocity>,
    movement_animations: ReadStorage<'a, MovementAnimations>,
    animations: WriteStorage<'a, Animation>,
    sprites: WriteStorage<'a, Sprite>,
}

impl<'a> System<'a> for Animator {
    type SystemData = AnimatorData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let AnimatorData {
            entities,
            velocities,
            movement_animations,
            animations,
            sprites,
        } = data;

        // Update the Animation component of every entity with Velocity and MovementAnimations
        // This loop can be made into a separate System for increased parallelism as the game grows
        for (entity, &Velocity {speed, direction}, move_animations) in (&*entities, &velocities, &movement_animations).join() {
            // Clone the frames (cheaply thanks to Arc) so we can use them without keeping a
            // reference to the animation around. This helps us mutate `animations` without keeping
            // an immutable reference to it around.
            let anim_frames = animations.get(entity).map(|anim| anim.frames.clone());
            // Stop animating movement if the entity has stopped
            if speed == 0 && anim_frames.is_some() {
                animations.remove(entity);
                continue;
            }

            let dir_anim = match direction {
                Direction::Up => &move_animations.walking_up,
                Direction::Down => &move_animations.walking_down,
                Direction::Left => &move_animations.walking_left,
                Direction::Right => &move_animations.walking_right,
            };

            // Testing for equality of two Vecs would normally be quite expensive, but luckily
            // since we are using Arc<Vec<_>>, this will check if the pointers are equal first
            // (thus making the comparision very cheap in most cases)
            let needs_update = match anim_frames {
                // Only update if a different animation is currently playing
                Some(anim_frames) => anim_frames != dir_anim.frames,
                // No animation currently, so we can update it unconditionally
                None => true,
            };

            if needs_update {
                animations.insert(entity, dir_anim.clone());
            }
        }

        // Advance each animation and update the current sprite to be rendered when necessary
        for (anim, sprite) in (&mut animations, &mut sprites).join() {
            // Advance the animation frame if enough time has elapsed
            if anim.frame_timer.elapsed() >= anim.frames[anim.current_frame].duration {
                // Loop back to the first frame if we've advanced past the end
                anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
                // Reset the frame timer
                anim.frame_timer = Instant::now();

                // Current frame has changed, so we need to update the sprite
                *sprite = anim.frames[anim.current_frame].sprite.clone();
            }
        }
    }
}
