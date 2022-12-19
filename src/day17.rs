use std::borrow::Borrow;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::repeat;
use std::ops::Add;
use itertools::{interleave, Itertools};
use crate::vector2::{Vector2i, Vector2i64};

type Shape = Vec<Vector2i64>;
type Jet = Vector2i64;

struct Tunnel {
    grid: HashSet<Vector2i64>,
    width: i64,
    height: i64
}

impl Tunnel {
    fn is_free(&self, pos: Vector2i64) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && !self.grid.contains(&pos)
    }

    fn print(&self) {
        let str = (0..15).rev().map(|y| (0..7).map(|x| if(self.grid.contains(&Vector2i64{x, y})){"#"} else {"."}).join("")).join("\n");

        println!("{}", str)
    }

    fn insert(&mut self, vector: Vector2i64) {
        self.height = max(self.height, vector.y);

        self.grid.insert(vector);

    }
}

struct LoopingIterator<T> {
    vector: Vec<T>,
    next: usize,
}

impl<T: Clone> Iterator for LoopingIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.vector.get(self.next).map(|result| result.clone());
        self.next = (self.next + 1) % self.vector.len();

        return value;
    }
}


fn parse_jets(jets: &str) -> Vec<Vector2i64> {
    jets.chars()
        .map(|char| match char {
            '<' => Vector2i64 { x: -1, y: 0 },
            '>' => Vector2i64 { x: 1, y: 0 },
            _ => panic!()
        })
        .collect()
}

fn move_shape(shape: Shape, jet: Vector2i64, tunnel: &mut Tunnel, i: usize) -> Option<Shape> {
    let mut new_shape = shape
        .clone()
        .into_iter()
        .map(|pos| pos.add(jet));

    if(i < 10) {
        println!("{}, {}", jet.x, jet.y);
    }

    if new_shape.clone().any(|pos| !tunnel.is_free(pos)) {
        if jet.y < 0 {
            for pos in shape {
                tunnel.insert(pos);
            }

            return None;
        }
        else {
            return Some((&shape).clone());
        }
    }

    return Some(new_shape.collect());
}

fn drop_shape(shape: Shape, movements: &mut dyn Iterator<Item=Vector2i64>, tunnel: &mut Tunnel, i: usize) {
    let mut resulting_shape = Some(shape.into_iter().map(|pos| pos.add(Vector2i64 { x: 2, y: tunnel.height + 4 })).collect());

    while resulting_shape.is_some() {
        resulting_shape = move_shape(resulting_shape.unwrap(), movements.next().unwrap(), tunnel, i)
    }
}

fn drop_shapes(shapes: Vec<Shape>, jets: Vec<Jet>, dimension: i64, amount: usize) -> i64 {
    let mut jet_iterator = interleave(LoopingIterator { vector: jets, next: 0 }, repeat(Vector2i64 { x: 0, y: -1 }));
    let shape_iterator = LoopingIterator { vector: shapes, next: 0 };
    let mut tunnel = Tunnel {
        grid: HashSet::new(),
        width: dimension,
        height: -1,
    };

    for (i, shape) in shape_iterator.take(amount).enumerate() {
        if(i < 10) {
            println!("------ {} ------", i);
            tunnel.print();
        }

        if(i % 10000 == 0) {
            println!("Completed {}/{}", i, amount)
        }
        drop_shape(shape, &mut jet_iterator, &mut tunnel, i);

        if((0..dimension).map(|x| Vector2i64{x, y: tunnel.height}).all(|pos| !tunnel.is_free(pos))){
            println!("Exact!");
        }
    }

    tunnel.height + 1

}

fn get_shapes() -> Vec<Shape> {
    let shapes_string = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

    shapes_string.split("\n\n").map(|shape_string| shape_string
        .lines()
        .rev()
        .enumerate()
        .map(|(y, line)| line.trim().chars().enumerate().filter(|(_, char)| *char == '#').map(move |(x, char)| Vector2i64 { x: x as i64, y: y as i64 }))
        .flatten().collect()).collect()
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        let jets = parse_jets(include_str!("day17/test_simple.txt"));
        let shapes = get_shapes();
        assert_eq!(drop_shapes(shapes, jets, 7, 2022), 3068);
    }

    #[test]
    fn large_test() {
        let jets = parse_jets(include_str!("day17/test_large.txt"));

        assert_eq!(drop_shapes(get_shapes(), jets, 7, 2022), 0);
    }

    #[test]
    fn large_test_2() {
        let jets = parse_jets(include_str!("day17/test_large.txt"));

        assert_eq!(drop_shapes(get_shapes(), jets, 7, 1000000000000), 0);
    }
}