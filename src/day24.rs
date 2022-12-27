use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Position;
use multimap::MultiMap;
use priority_queue::PriorityQueue;
use crate::direction::Direction;
use crate::direction::Direction::{Down, Left, Right, Up};
use crate::vector2::Vector2i;

#[derive(PartialEq, Clone, Copy)]
struct Blizzard {
    pos: Vector2i,
    dir: Direction,
}

impl Blizzard {
    fn get_position_at(&self, i: usize, world_dimensions: Vector2i) -> Vector2i {
        let mut new_position = self.pos + self.dir.to_vector() * i as i32;

        new_position.x = ((new_position.x % world_dimensions.x) + world_dimensions.x) % world_dimensions.x;
        new_position.y = ((new_position.y % world_dimensions.y) + world_dimensions.y) % world_dimensions.y;

        return new_position;
    }
}

#[derive(PartialEq, Clone)]
struct World {
    dimensions: Vector2i,
    start: Vector2i,
    end: Vector2i,
    blizzards: Vec<Blizzard>,
}

impl World {


    fn parse_from(string: &str) -> World {
        let mut blizzards = vec![];
        for (y, line) in string.lines()
            .skip(1)
            .enumerate() {
            for (x, char) in line.chars()
                .skip(1)
                .enumerate() {

                let direction = match char {
                    '^' => Some(Up),
                    '>' => Some(Right),
                    '<' => Some(Left),
                    'v' => Some(Down),
                    _ => None
                };

                if let Some(dir) = direction {
                    blizzards.push(Blizzard {
                        pos: Vector2i{x: x as i32,y: y as i32},
                        dir
                    })
                }
            }
        }

        let dimensions = Vector2i{x: (string.lines().next().unwrap().chars().count() - 2) as i32, y: (string.lines().count() - 2) as i32 };
        return World {
            dimensions,
            start: Vector2i {x: 1, y: -1},
            end: Vector2i {x: dimensions.x - 1, y: dimensions.y},
            blizzards
        }
    }
}

struct BlizzardIterator {
    blizzards: Vec<Blizzard>,
    next_index: usize,
    world_dimensions: Vector2i,
}

fn lcm(first: i32, second: i32) -> i32 {
    first * second / gcd(first, second)
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl Iterator for BlizzardIterator {
    type Item = HashSet<Vector2i>;

    fn next(&mut self) -> Option<Self::Item> {
        let positions = Some(self.blizzards
            .iter()
            .map(|blizzard| blizzard.get_position_at(self.next_index, self.world_dimensions))
            .collect());

        self.next_index += 1;

        return positions;
    }
}

fn find_shortest_path(world: &World, blizzards: &Vec<HashSet<Vector2i>>) -> usize{
    //let mut explored_paths
    let mut paths_to_expore: PriorityQueue<Vec<Vector2i>, i32> = PriorityQueue::new();
    let mut explored_paths: HashSet<(usize, Vector2i)> = HashSet::new();

    paths_to_expore.push(vec![world.start], world.start.get_manhattan_distance(world.end));

    let mut i: usize = 0;
    while let Some((path, prior)) = paths_to_expore.pop() {
        let current_blizzards = &blizzards[path.len() % blizzards.len()];

        let mut new =
            [
                Vector2i { x: 0, y: 0 },
                Right.to_vector(),
                Left.to_vector(),
                Up.to_vector(),
                Down.to_vector(),
            ]
                .map(|dir| {
                    let mut new_path = path.clone();
                    new_path.push(*path.last().unwrap() + dir);

                    return new_path;
                })
                .into_iter();

        let result =  new.clone().find(|path| world.end == *path.last().unwrap());
        if let Some(path) = result {
            return path.len() - 1;
        }

        if i % 10000 == 0 {
            //println!("Priority: {}, Length: {}", i32::MAX - prior, path.len());
        }



        let mut nexts: Vec<Vec<Vector2i>> = new
            .filter(|path| {
                let pos = path.last().unwrap();

                let a = (pos.x >= 0 && pos.x < world.dimensions.x &&
                    pos.y >= 0 && pos.y < world.dimensions.y);
                let b = *pos == world.start;
                let c = current_blizzards.contains(pos);
                let d = explored_paths.contains(&(path.len(), *pos));

                return ((pos.x >= 0 && pos.x < world.dimensions.x &&
                    pos.y >= 0 && pos.y < world.dimensions.y) || *pos == world.start) &&
                    !current_blizzards.contains(pos) &&
                    !explored_paths.contains(&(path.len(), *pos));
            }).collect();

        for next in nexts {
            let true_priority = (next.len() as i32 + next.last().unwrap().get_manhattan_distance(world.end));
            let priority = i32::MAX - true_priority;
            explored_paths.insert((next.len(), *next.last().unwrap()));
            paths_to_expore.push(next, priority);
        }

        i += 1;
    }

    panic!()
}

fn find_shortest_back_forth_distance(world: &World) -> usize {
    let max_cycle = lcm(world.dimensions.x, world.dimensions.y);
    let mut blizzards: Vec<HashSet<Vector2i>> = (0..max_cycle).map(|i| world.blizzards.iter().map(|blizzard| blizzard.get_position_at(i as usize, world.dimensions)).collect()).collect();

    println!("Cycle of length {}", max_cycle);

    let mut backward_world = world.clone();
    backward_world.end = world.start;
    backward_world.start = world.end;

    let forward_world = world.clone();

    let forward_paths: Vec<_> = (0..max_cycle).map(|i| {
        let mut rotated = blizzards.clone();
        rotated.rotate_left(i as usize);

        println!("Calculating forward path {}/{}", i, max_cycle);

        return find_shortest_path(&forward_world, &rotated);
    }).collect();



    let backward_paths: Vec<_> = (0..max_cycle).map(|i| {
        let mut rotated = blizzards.clone();
        rotated.rotate_left(i as usize);

        println!("Calculating backward path {}/{}", i, max_cycle);
        return find_shortest_path(&backward_world, &rotated);
    }).collect();

    let mut paths_to_explore = PriorityQueue::<(usize, usize), Reverse<usize>>::new();

    for (i, path) in forward_paths.iter().enumerate() {
        paths_to_explore.push((0, *path + i), Reverse(*path + i));

        println!("Added {} ", *path)
    }

    while let Some(((i, _), path_len)) = paths_to_explore.pop() {
        println!("Read {}, {} ", i, path_len.0);

        if i == 2 {
            let a = "e";
            return path_len.0;
        }

        if i == 1 && path_len.0 == 41 {
            let a = 0;
        }

        for (extra, cycle) in (path_len.0..(path_len.0 + max_cycle as usize)).enumerate()  {
            if i == 0 {
                let val = path_len.0 + backward_paths[cycle % max_cycle as usize] + extra;

                println!("{}, {}", i + 1, val);

                paths_to_explore.push((i + 1, val), Reverse(path_len.0 + backward_paths[cycle % max_cycle as usize] + extra));
            }
            else {
                let new_val = Reverse(path_len.0 + forward_paths[cycle % max_cycle as usize] + extra);
                println!("{}, {}", i + 1, new_val.0);

                paths_to_explore.push((i + 1, path_len.0 + forward_paths[cycle % max_cycle as usize] + extra), new_val);
            }
        }
    }

    panic!()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut world = World::parse_from(include_str!("day24/test_simple.txt"));
        let max_cycle = lcm(world.dimensions.x, world.dimensions.y);
        let mut blizzards: Vec<HashSet<Vector2i>> = (0..max_cycle).map(|i| world.blizzards.iter().map(|blizzard| blizzard.get_position_at(i as usize, world.dimensions)).collect()).collect();

        assert_eq!(find_shortest_path(&world, &blizzards), 18)
    }

    #[test]
    fn large_test() {
        let mut world = World::parse_from(include_str!("day24/test_large.txt"));
        let max_cycle = lcm(world.dimensions.x, world.dimensions.y);
        let mut blizzards: Vec<HashSet<Vector2i>> = (0..max_cycle).map(|i| world.blizzards.iter().map(|blizzard| blizzard.get_position_at(i as usize, world.dimensions)).collect()).collect();

        assert_eq!(find_shortest_path(&world, &blizzards), 264)
    }

    #[test]
    fn simple_test_2() {
        let mut world = World::parse_from(include_str!("day24/test_simple.txt"));

        assert_eq!(find_shortest_back_forth_distance(&world), 54)
    }

    #[test]
    fn large_test_2() {
        let mut world = World::parse_from(include_str!("day24/test_large.txt"));

        assert_eq!(find_shortest_back_forth_distance(&world), 789)
    }
}