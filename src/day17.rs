use std::borrow::Borrow;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::repeat;
use std::ops::Add;
use itertools::{interleave, Itertools};
use crate::vector2::{Vector2, Vector2i, Vector2i64};

type Shape = Vec<Vector2i64>;
type Jet = Vector2i64;

type SeenShapes = HashMap<(Vec<Vector2i64>, usize, usize), (i64, usize)>;

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
        for mov in [movements.next().unwrap(), Vector2i64{x: 0, y: -1}]{
            resulting_shape = move_shape(resulting_shape.unwrap(), mov, tunnel, i)
        }
    }
}

fn drop_shapes(shapes: Vec<Shape>, jets: Vec<Jet>, dimension: i64, amount: usize) -> i64 {
    let mut seen_shapes: SeenShapes = HashMap::new();
    let len_jet = jets.len();
    let len_snake = shapes.len();
    let mut jet_iterator = LoopingIterator { vector: jets, next: 0 };
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

        let last_two_rows: Vec<Vec<_>> = (0..dimension).map(|x| (0..=1).map(|y_offset| Vector2i64{x, y: tunnel.height - y_offset}).collect()).collect();

        if(i == (2471 + 449)) {
            let a = "a";
        }

        if last_two_rows.iter().all(|positions| !tunnel.is_free(positions[0]) || !tunnel.is_free(positions[1])) {
            let to_obj: (Vec<_>, usize, usize) = (last_two_rows.into_iter().map(|positions| positions[0].add(Vector2i64{x: 0, y: -tunnel.height})).collect(), (jet_iterator.next) % len_jet, i % len_snake);

            println!("Completed {}/{}, shape is suitable.", i, amount);

            if seen_shapes.contains_key(&to_obj) {
                println!("Found");

                let segment_height = (tunnel.height - seen_shapes.get(&to_obj).unwrap().0);
                let segment_count = (amount - i) / (i - seen_shapes.get(&to_obj).unwrap().1);
                let total_height = segment_count * segment_height as usize;
                let remaining_after_segment = (amount - i) % (i - seen_shapes.get(&to_obj).unwrap().1);
                let a = "";
            }
            else {
                println!("Inserted {}", to_obj.1);
                seen_shapes.insert(to_obj, (tunnel.height, i));
            }
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

        assert_eq!(drop_shapes(get_shapes(), jets, 7, 2022), 3219);
    }

    #[test]
    fn simple_test_2() {
        let jets = parse_jets(include_str!("day17/test_simple.txt"));
        let shapes = get_shapes();
        assert_eq!(drop_shapes(shapes, jets, 7, 1000000000000), 3068);
    }

    #[test]
    fn large_test_2() {
        let jets = parse_jets(include_str!("day17/test_large.txt"));

        assert_eq!(drop_shapes(get_shapes(), jets, 7, 1000000000000), 0);
    }
}