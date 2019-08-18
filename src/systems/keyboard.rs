use specs::{System, SystemData, Read, ReadStorage, WriteStorage, Join, World, prelude::ResourceId};

use crate::resources::KeyboardEvent;
use crate::components::{Player, Velocity};

pub struct Keyboard;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct KeyboardData<'a> {
    players: ReadStorage<'a, Player>,
    velocities: WriteStorage<'a, Velocity>,
    keyboard_event: Read<'a, Option<KeyboardEvent>>,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = KeyboardData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let KeyboardData {players, velocities, keyboard_event} = data;

        use KeyboardEvent::*;
        match *keyboard_event {
            // Instruct player to move in the given direction
            Some(MoveInDirection(direction)) => {
                for (&Player {movement_speed}, velocity) in (&players, &velocities).join() {
                    velocity.speed = movement_speed;
                    velocity.direction = direction;
                }
            },
            // Instruct player to stop (but preserve the direction)
            Some(Stop) => {
                for (_, velocity) in (&players, &velocities).join() {
                    velocity.speed = 0;
                }
            },
            // Do nothing if there is no event to process
            None => {},
        }
    }
}
