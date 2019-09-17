use std::time::Duration;

use crate::direction::Direction;

/// The amount of time elapsed since the last frame
#[derive(Debug, Default)]
pub struct TimeDelta(pub Duration);

#[derive(Debug)]
pub enum KeyboardEvent {
    /// Begin to move in the given direction
    MoveInDirection(Direction),
    /// Stop moving in the current direction
    Stop,
}

/// The current status of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running,
    Win,
    Lose,
}
