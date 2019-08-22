#![allow(dead_code, unused_variables, unused_imports, unused_mut)] //TODO(EX#3): remove this line

use std::time::Instant;

use rand::{Rng, thread_rng};
use specs::{System, SystemData, WriteStorage, Join, World, prelude::ResourceId};

use crate::direction::Direction;
use crate::components::{Enemy, Velocity};

pub struct AI;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct AIData<'a> {
    enemies: WriteStorage<'a, Enemy>,
    velocities: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for AI {
    type SystemData = AIData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let AIData {mut enemies, mut velocities} = data;

        //TODO(EX#3): Fill in this code based on enemy.rs.
        // HINT: You will need to use the Join trait: https://slide-rs.github.io/specs/08_join.html
        let mut rng = thread_rng();
        for (enemy, velocity) in (&mut enemies, &mut velocities).join() {
            // Avoid changing the direction too rapidly by only doing it every so often
            if enemy.direction_timer.elapsed() >= enemy.direction_change_delay {
                velocity.direction = match rng.gen_range(1, 101) {
                    // 60% probability of staying in the same direction
                    1..=60 => velocity.direction,
                    // 10% chance of changing to some random direction (one of which could be the same)
                    61..=70 => Direction::Up,
                    71..=80 => Direction::Down,
                    81..=90 => Direction::Left,
                    91..=100 => Direction::Right,
                    _ => unreachable!(),
                };

            // Reset the direction timer
            enemy.direction_timer = Instant::now();
            }
        }
    }
}
