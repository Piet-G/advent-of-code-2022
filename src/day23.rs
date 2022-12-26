use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use Direction::N;
use crate::day23::Direction::{E, NE, NW, S, SE, SW, W};
use crate::vector2::Vector2i;
use multimap::MultiMap;

enum Direction {
    N, NE, E, SE, S, SW, W, NW
}

impl Direction {
    fn to_vector(&self) -> Vector2i {
        match self {
            N => Vector2i{x: 0, y: -1},
            NE => Vector2i{x: 1, y: -1},
            E => Vector2i{x: 1, y: 0},
            SE => Vector2i{x: 1, y: 1},
            S => Vector2i{x: 0, y: 1},
            SW => Vector2i{x: -1, y: 1},
            W => Vector2i{x: -1, y: 0},
            NW => Vector2i{x: -1, y: -1}
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            N => NE,
            NE => E,
            E => SE,
            SE => S,
            S => SW,
            SW => W,
            W => NW,
            NW => N,
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            N => NW,
            NE => N,
            E => NE,
            SE => E,
            S => SE,
            SW => S,
            W => SW,
            NW => W,
        }
    }
}

struct World {
    grid: HashSet<Vector2i>,
    //min: Vector2i,
    //max: Vector2i,
}

impl World {
    // fn add_elf(&mut self, pos: Vector2i) {
    //     self.min.x = min(self.min.x, pos.x);
    //     self.max.x = max(self.max.x, pos.x);
    //     self.min.y = min(self.min.y, pos.y);
    //     self.max.y = max(self.max.y, pos.y);
    //
    //     self.grid.insert(pos);
    // }

    fn get_aabb(&self) -> (Vector2i, Vector2i) {
        let mut min_pos = Vector2i{x: i32::MAX, y: i32::MAX};
        let mut max_pos = Vector2i{x: i32::MIN, y: i32::MIN};

        for pos in self.grid.iter() {
            min_pos.x = min(pos.x, min_pos.x);
            min_pos.y = min(pos.y, min_pos.y);
            max_pos.x = max(pos.x, max_pos.x);
            max_pos.y = max(pos.y, max_pos.y);
        }

        return (min_pos, max_pos)
    }

    fn get_empties(&self) -> i32 {
        let (min_pos, max_pos) = self.get_aabb();
        let size = max_pos - min_pos + Vector2i{x: 1, y: 1};

        return size.x * size.y - (self.grid.len() as i32);
    }

    fn parse_from(string: &str) -> World {
        let mut grid = HashSet::new();

        for (y, line) in string.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    grid.insert(Vector2i{x: x as i32, y: y as i32});
                }
            }
        }

        return World { grid };
    }

    fn step_1(&self, iteration: usize) -> MultiMap<Vector2i, Vector2i> {
        let mut proposed_moves = MultiMap::new();
        for pos in self.grid.iter() {
            if [N, NE, E, SE, S, SW, W, NW].iter().any(|dir| self.grid.contains(&(*pos + dir.to_vector()))) {
                let mut order = [N, S, W, E];
                order.rotate_left(iteration % 4);
                for dir in order{
                    if !self.grid.contains(&(*pos + dir.to_vector())) &&
                        !self.grid.contains(&(*pos + dir.rotate_left().to_vector())) &&
                        !self.grid.contains(&(*pos + dir.rotate_right().to_vector())) {
                        proposed_moves.insert((*pos + dir.to_vector()), *pos);
                        break;
                    }
                }
            }
        }

        return proposed_moves;
    }

    fn step(&mut self, iteration: usize) -> bool{
        let proposed = self.step_1(iteration);
        let has_no_proposed = proposed.len() == 0;

        for (to_pos, from_positions) in proposed {
            if from_positions.len() == 1 {
                self.grid.remove(from_positions.first().unwrap());
                self.grid.insert(to_pos);
            }
        }

        let (min_pos, max_pos) = self.get_aabb();

        //println!("{}",(min_pos.y..=max_pos.y).map(|y| (min_pos.x..=max_pos.x).map(|x| if self.grid.contains(&Vector2i{x,y}) {"#"} else {"."}).join("")).join("\n"));

        if iteration % 10000 == 0 {
            println!("Not found after {} steps", iteration)
        }

        return has_no_proposed
    }

    fn do_x_steps(&mut self, amount: usize) {
        for i in 0..amount {
            self.step(i);
        }
    }

    fn do_steps_until(&mut self) -> usize {
        for i in 0.. {
            if(self.step(i)){
                return i + 1;
            }
        }

        panic!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut world = World::parse_from(include_str!("day23/test_simple.txt"));

        world.do_x_steps(10);

        assert_eq!(world.get_empties(), 110)
    }

    #[test]
    fn simple_large() {
        let mut world = World::parse_from(include_str!("day23/test_large.txt"));

        world.do_x_steps(10);

        assert_eq!(world.get_empties(), 110)
    }

    #[test]
    fn simple_test_2() {
        let mut world = World::parse_from(include_str!("day23/test_simple.txt"));

        assert_eq!(world.do_steps_until(), 20)
    }

    #[test]
    fn test_large_2() {
        let mut world = World::parse_from(include_str!("day23/test_large.txt"));

        assert_eq!(world.do_steps_until(), 20)
    }
}