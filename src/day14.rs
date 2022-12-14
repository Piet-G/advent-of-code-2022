use std::collections::{HashMap, HashSet, TryReserveError};
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, fmt, ops};
use std::cmp::{max, min, Ordering, Reverse};
use std::fmt::Display;
use std::iter::Map;
use std::ptr::replace;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::day14::GridObject::{Rock, Sand};
use crate::vector2::Vector2;

#[derive(Eq, PartialEq)]
enum GridObject {
    Sand,
    Rock,
}

type Grid = HashMap<Vector2, GridObject>;

fn string_to_vector2(string: &str) -> Vector2 {
    let mut values = string.split(",");

    Vector2 {
        x: values.next().unwrap().parse().unwrap(),
        y: values.next().unwrap().parse().unwrap()
    }
}

fn drop_sand(pos: Vector2, grid: &Grid, deepest_y: usize, with_floor: bool) -> Option<Vector2> {
    let down_position = pos + Vector2{x: 0, y: 1};
    let down_left_position = down_position - Vector2{x: 1, y: 0};
    let down_right_position = down_position + Vector2{x: 1, y: 0};

    if pos.y >= deepest_y && !with_floor{
        return None
    }

    if with_floor && down_position.y >= deepest_y + 2 {
        return Some(pos)
    }

    match (grid.get(&down_position), grid.get(&down_left_position), grid.get(&down_right_position)) {
        (None, _, _) => return drop_sand(down_position, grid, deepest_y, with_floor),
        (Some(_), None, _) => return drop_sand(down_left_position, grid, deepest_y, with_floor),
        (Some(_), Some(_), None) => return drop_sand(down_right_position, grid, deepest_y, with_floor),
        (Some(_), Some(_), Some(_)) => Some(pos)
    }
}

fn parse_rocks(rocks_string: &str) -> (Grid, usize){
    let mut grid = HashMap::new();

    let mut deepest_y = 0;

    for line in rocks_string.lines() {
        for window in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            let pos1 = string_to_vector2(window[0]);
            let pos2 = string_to_vector2(window[1]);

            for x in min(pos1.x, pos2.x)..=max(pos1.x, pos2.x) {
                for y in min(pos1.y, pos2.y)..=max(pos1.y, pos2.y)  {
                    grid.insert(Vector2{x,y},Rock);
                    deepest_y = max(y, deepest_y);
                }
            }
        }
    }

    (grid, deepest_y)
}

fn count_drops_until((mut grid, deepest_y): (Grid, usize), pos: Vector2) -> usize {
    for i in 1.. {
        match drop_sand(pos, &grid, deepest_y, false) {
            None => return i - 1,
            Some(position) => {
                grid.insert(position, Sand);
            }
        }
    }

    panic!()
}

fn count_drops_until_blocked((mut grid, deepest_y): (Grid, usize), pos: Vector2) -> usize {
    for i in 1.. {
        match drop_sand(pos, &grid, deepest_y, true) {
            None => panic!(),
            Some(position) => {
                grid.insert(position, Sand);

                if position == pos {
                    return i;
                }
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_drops_until(parse_rocks(fs::read_to_string("src/day14/test_simple.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 24);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_drops_until(parse_rocks(fs::read_to_string("src/day14/test_large.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 644);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(count_drops_until_blocked(parse_rocks(fs::read_to_string("src/day14/test_simple.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 93);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(count_drops_until_blocked(parse_rocks(fs::read_to_string("src/day14/test_large.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 27324);
    }
}