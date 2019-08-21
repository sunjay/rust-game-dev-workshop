#![allow(unused_imports, unused_variables)] //TODO(EX#5): remove this line

use specs::{System, SystemData, Entities, ReadStorage, WriteStorage, Join, World, prelude::ResourceId};

//TODO(EX#5): You will need to import some things.

pub struct Animator;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct AnimatorData<'a> {
    //TODO(EX#5): You will need this in order to add or remove components from a component storage.
    // HINT: Need to use `&*entities` - https://slide-rs.github.io/specs/08_join.html#basic-joining
    entities: Entities<'a>,
    //TODO(EX#5): Which components do you think you need to implement this system?
    // HINT: You can always come back and add more fields here later. Try implementing the rest of
    // the system and you'll figure out what you need naturally.
}

impl<'a> System<'a> for Animator {
    type SystemData = AnimatorData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let AnimatorData {entities} = data;

        // Update the Animation component of every entity with Velocity and MovementAnimations
        // This loop can be made into a separate System for increased parallelism as the game grows
        //TODO(EX#5): Implement this part of the animation engine.
        // HINT: https://slide-rs.github.io/specs/06_system_data.html#adding-and-removing-components
        // HINT: Don't forget that movement animations should only apply when an entity is moving.
        // HINT: Only update the animation if it is not already playing or else your animation will
        //  restart over and over again and never advance.

        // Advance each animation and update the current sprite to be rendered when necessary
        //TODO(EX#5): Implement this part of the animation engine.
        // HINT: Each frame has a duration and each animation has a timer. Can you use that
        //  information to determine when to go to the next frame? What state needs to be updated
        //  when the frame changes? Do any other components need to change?
    }
}
