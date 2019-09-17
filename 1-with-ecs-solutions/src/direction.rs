use sdl2::rect::Point;

/// Represents a direction of motion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns a point that represents this direction in world coordinates
    pub fn into_point(self) -> Point {
        use Direction::*;
        match self {
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
            Left => Point::new(-1, 0),
            Right => Point::new(1, 0),
        }
    }
}
