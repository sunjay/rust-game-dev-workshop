#![allow(unused_imports)] //TODO(BONUS#2): remove this line

use specs::{System, SystemData, Read, ReadStorage, WriteStorage, Join, World, prelude::ResourceId};

//TODO(BONUS#2): You will need to import some things.

pub struct Keyboard;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct KeyboardData<'a> {
    //TODO(BONUS#2): What data do you need to implement this system?
    // HINT: https://slide-rs.github.io/specs/06_system_data.html
    // HINT: https://slide-rs.github.io/specs/04_resources.html
    #[allow(dead_code)] //TODO(BONUS#2): This field is not needed. It is just a placeholder.
    enemies: WriteStorage<'a, crate::components::Enemy>,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = KeyboardData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        //TODO(BONUS#2): Update the player's velocity based on the keyboard event.
        // HINT: Look at the `walk_in_direction` and `stop` methods in player.rs.
        let KeyboardData {..} = data;
    }
}
