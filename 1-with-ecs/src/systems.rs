//TODO(BONUS#2): Remove this line or else your solution will never run!
#[path = "../../1-with-ecs-solutions/src/systems/keyboard.rs"]
mod keyboard;
mod movement;
//TODO(BONUS#3): Remove this line or else your solution will never run!
#[path = "../../1-with-ecs-solutions/src/systems/animator.rs"]
mod animator;
mod ai;
mod win_lose_checker;

pub use keyboard::*;
pub use movement::*;
pub use animator::*;
pub use ai::*;
pub use win_lose_checker::*;
