use std::time::Duration;

//TODO(EX#4): You may need to import some things.

/// The amount of time elapsed since the last frame
#[derive(Debug, Default)]
pub struct TimeDelta(pub Duration);

#[derive(Debug)]
#[allow(dead_code)] //TODO(EX#4): remove this line
pub enum KeyboardEvent {
    //TODO(EX#4): How do you want to model the commands we can get from the keyboard?
    // Use the event handling code from 0-without-ecs/src/main.rs to plan this out.
    // HINT: Look at the `walk_in_direction` and `stop` methods in player.rs.
}

/// The current status of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running,
    Win,
    Lose,
}
