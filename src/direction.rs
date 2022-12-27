use Direction::{Down, Left, Right, Up};
use crate::vector2::Vector2i;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}


impl Direction {
    pub(crate) fn to_vector(&self) -> Vector2i {
        match self {
            Up => Vector2i{x: 0, y: -1},
            Right => Vector2i{x: 1, y: 0},
            Down => Vector2i{x: 0, y: 1},
            Left => Vector2i{x: -1, y: 0}
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
}