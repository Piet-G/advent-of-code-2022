use std::collections::{HashSet, TryReserveError};
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, ops};
use std::cmp::Reverse;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::vector2::Vector2;

type Path = Vec<Vector2>;

struct Square {
    path: Option<Path>,
    height: usize,
}

struct Grid {
    squares: Vec<Vec<Square>>,
    starting_positions: Vec<Vector2>,
    stop: Vector2
}

fn parse_grid(grid_string: &str, count_a: bool) -> Grid{
    let mut start = Vec::new();
    let mut stop = Vector2{x: 0, y: 0};

    for (x, line) in grid_string.lines().enumerate() {
        if let Some(y) = line.chars().position(|c| c == 'S') {
            start.push(Vector2{x ,y});
        }

        if count_a {
            start.append(&mut line.chars().enumerate().filter(|(y, el)| *el == 'a').map(|(y, el)| Vector2{x,y}).collect())
        }
        if let Some(y) = line.chars().position( |c| c == 'E'){
            stop = Vector2{x,y};
        }
    }

    let squares = grid_string
        .lines()
        .map(|line| line
            .chars()
            .map(|char| Square {
                height: char_to_height(char),
                path: None
            })
            .collect()
        )
        .collect();

    Grid {
        starting_positions: start,
        stop,
        squares
    }
}

fn char_to_height(char: char) -> usize {
    let mut char_to_find = char;
    if char.is_uppercase(){
        char_to_find = 'z';
    }

    return ('a'..='z').position(|c| c == char_to_find).unwrap();
}

fn get_shortest_path(mut grid: Grid) -> usize {

    let mut priority_queue: PriorityQueue<Vector2, Reverse<usize>> = PriorityQueue::new();

    for start in grid.starting_positions{
        grid.squares[start.x][start.y].path = Some(vec![]);
        priority_queue.push(start, Reverse(1));
    }

    while let Some((position, length)) = priority_queue.pop() {
        if position == grid.stop {
            return length.0
        }

        let dimensions = Vector2{x: grid.squares.len(), y: grid.squares[0].len()};

        let neighbour_positions: Vec<_> = get_neighbouring_positions(position, dimensions)
            .into_iter()
            .filter(|pos| grid.squares[pos.x][pos.y].path.is_none() || grid.squares[pos.x][pos.y].path.as_ref().unwrap().len() > length.0)
            .filter(|pos| grid.squares[pos.x][pos.y].height <= grid.squares[position.x][position.y].height + 1)
            .collect();

        for neighbour_position in neighbour_positions {
            let mut path =  grid.squares[position.x][position.y].path.as_ref().unwrap().clone();
            path.push(neighbour_position);
            let len = path.len();
            grid.squares[neighbour_position.x][neighbour_position.y].path = Some(path);

            priority_queue.push(neighbour_position, Reverse(len));
        }
    }

    return 0;

}

fn get_neighbouring_positions(pos: Vector2, grid_size: Vector2) -> Vec<Vector2>{
    let mut positions = Vec::new();
    if pos.x > 0 {
        positions.push(pos - Vector2{x: 1,y: 0})
    }

    if pos.y > 0 {
        positions.push(pos - Vector2{x: 0,y: 1})
    }

    if pos.x + 1 < grid_size.x {
        positions.push(pos + Vector2{x: 1,y: 0})
    }

    if pos.y + 1 < grid_size.y {
        positions.push(pos + Vector2{x: 0,y: 1})
    }

    positions
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(get_shortest_path(parse_grid(fs::read_to_string("src/day12/test_simple.txt").unwrap().as_str(), false)), 31);
    }

    #[test]
    fn large_test() {
        assert_eq!(get_shortest_path(parse_grid(fs::read_to_string("src/day12/test_large.txt").unwrap().as_str(), false)), 504);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(get_shortest_path(parse_grid(fs::read_to_string("src/day12/test_simple.txt").unwrap().as_str(), true)), 29);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(get_shortest_path(parse_grid(fs::read_to_string("src/day12/test_large.txt").unwrap().as_str(), true)), 500);
    }
}