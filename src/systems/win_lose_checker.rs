use specs::{System, SystemData, ReadStorage, WriteExpect, Join, World, prelude::ResourceId};

use crate::resources::GameStatus;
use crate::components::{Player, Enemy, Goal, BoundingBox};

pub struct WinLoseChecker;

/// Data from the world required by the system
#[derive(SystemData)]
pub struct WinLoseCheckerData<'a> {
    players: ReadStorage<'a, Player>,
    enemies: ReadStorage<'a, Enemy>,
    goals: ReadStorage<'a, Goal>,
    bounding_boxes: ReadStorage<'a, BoundingBox>,
    game_status: WriteExpect<'a, GameStatus>,
}

impl<'a> System<'a> for WinLoseChecker {
    type SystemData = WinLoseCheckerData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let WinLoseCheckerData {players, enemies, goals, bounding_boxes, mut game_status} = data;

        for (_, BoundingBox(player_bounds)) in (&players, &bounding_boxes).join() {
            for (_, &BoundingBox(enemy_bounds)) in (&enemies, &bounding_boxes).join() {
                // If the player collides with any enemies, they lose
                if player_bounds.has_intersection(enemy_bounds) {
                    *game_status = GameStatus::Lose;
                    return;
                }
            }

            for (_, &BoundingBox(goal_bounds)) in (&goals, &bounding_boxes).join() {
                // If the player reaches the goal, they win
                if player_bounds.has_intersection(goal_bounds) {
                    *game_status = GameStatus::Win;
                    return;
                }
            }
        }
    }
}
