use std::collections::HashSet;
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, ops};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn get_manhattan_distance(self, other: Vector2) -> i32 {
        let absolute = (self - other).abs();

        let mut distance = absolute.x + absolute.y;


        if(absolute.x != 0 && absolute.y != 0){
            distance -= 1;
        }

        return distance;
    }

    fn abs(&self) -> Vector2 {
        return Vector2 {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Vector2 {
        return Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn move_to_vec(direction: &str, amount: i32) -> Vec<Vector2>{
    (0..amount).map(|_| match direction {
        "R" => Vector2{x: 1, y: 0},
        "L" => Vector2{x: -1, y: 0},
        "U" => Vector2{x: 0, y: 1},
        "D" => Vector2{x: 0, y: -1},
        &_ => panic!()
    }).collect()
}

fn count_visited_positions(moves_string: &str, initial_rope: &mut Vec<Vector2>) -> usize {
    let moves = moves_string
        .lines()
        .map(|line| {
            let mut split_line = line.split(" ");
            move_to_vec(split_line.next().unwrap(), split_line.next().unwrap().parse::<i32>().unwrap())
        }).flatten();

    let mut visited_positions = HashSet::new();

    visited_positions.insert(initial_rope.last().unwrap().clone());

    for rope_move in moves{
        let old_rope = initial_rope.clone();

        initial_rope[0] = initial_rope[0] + rope_move;

        for i in 1..initial_rope.len() {
            if initial_rope[i].get_manhattan_distance(initial_rope[i - 1]) > 1 {
                let mut equal_position = initial_rope[i - 1] - initial_rope[i];

                if equal_position.x.abs() == equal_position.y.abs() && equal_position.x.abs() > 1 {
                    equal_position = equal_position - (old_rope[i - 1] - initial_rope[i])
                }
                else if equal_position.x.abs() > equal_position.y.abs(){
                     equal_position.x -= equal_position.x / equal_position.x.abs();
                }
                else {
                    equal_position.y -= equal_position.y / equal_position.y.abs();
                }

                initial_rope[i] = initial_rope[i] + equal_position;
            }

            visited_positions.insert(initial_rope.last().unwrap().clone());
        }
    }

    return visited_positions.len()
}
#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_visited_positions(fs::read_to_string("src/day9/test_simple.txt").unwrap().as_str(), &mut vec![Vector2{x: 0, y: 0}; 2]), 13);
    }

    #[test]
    fn medium_test() {
        assert_eq!(count_visited_positions(fs::read_to_string("src/day9/test_medium.txt").unwrap().as_str(), &mut vec![Vector2{x: 0, y: 0}; 10]), 36);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_visited_positions(fs::read_to_string("src/day9/test_large.txt").unwrap().as_str(), &mut vec![Vector2{x: 0, y: 0}; 10]), 2536);
    }
}