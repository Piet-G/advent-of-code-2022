use std::collections::{HashMap, HashSet, TryReserveError};
use std::rc::{Rc, Weak};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, fmt, ops};
use std::cmp::{max, min, Ordering, Reverse};
use std::fmt::Display;
use std::iter::Map;
use std::ptr::replace;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use regex::SetMatches;
use crate::day15::GridObject::{Beacon, NoBeacon, Sensor};
use crate::vector2::{Vector2, Vector2i};

#[derive(Eq, PartialEq)]
enum GridObject {
    Beacon,
    NoBeacon,
    Sensor,
}

type Sensors = Vec<(Vector2i, Vector2i)>;

fn parse_grid(sensor_lines: &str) -> Sensors {
    sensor_lines.lines().map(|line| {
        let split_regex = regex::Regex::new(r"Sensor at x=|, y=|: closest beacon is at x=|, y=").unwrap();
        let mut vector_elements = split_regex.split(line).filter(|str| !str.is_empty()).map(|el| el.parse().unwrap());

        let sensor = Vector2i {
            x: vector_elements.next().unwrap(),
            y: vector_elements.next().unwrap(),
        };

        let beacon = Vector2i {
            x: vector_elements.next().unwrap(),
            y: vector_elements.next().unwrap(),
        };


        (sensor, beacon)
    }).collect()
}

fn get_position_at_distance(sensor: Vector2i, y_line: i32, distance: i32) -> Vec<i32> {
    (0..=distance).map(|x| [sensor.x - x, sensor.x + x]).flatten().collect()
}


fn count_invalids(beacons_and_sensors: Sensors, y: i32) -> usize {
    let beacons: Vec<_> = beacons_and_sensors.iter().map(|(_, beacon)| *beacon).collect();
    let sensors: Vec<_> = beacons_and_sensors.iter().map(|(sensor, _)| *sensor).collect();

    beacons_and_sensors.iter().map(|(sensor, beacon)| {
        let distance = sensor.get_manhattan_distance(*beacon);

        get_position_at_distance(*sensor, y, distance).into_iter().filter(|x| !sensors.contains(&Vector2i { x: *x, y }) && !beacons.contains(&Vector2i { x: *x, y }))
    }).flatten().unique().count()
}


fn get_outer_edge(sensor: Vector2i, bounds: Vector2i, distance: i32) -> HashSet<Vector2i> {
    println!("Getting outer edge for {}, {}", sensor.x, sensor.y);

    (max(0, sensor.x - distance)..=max(bounds.x, sensor.x + distance)).map(|x| {
        let y_abs = distance - (sensor.x - x).abs();

        [Vector2i { x, y: sensor.y - y_abs }, Vector2i { x, y: sensor.y + y_abs }]
    }).flatten().collect()

}

fn get_only_valid(beacons_and_sensors: Sensors, bounds: Vector2i) -> i64 {
    let edge_positions = beacons_and_sensors
        .iter()
        .map(|(sensor, beacon)| {
            let distance = sensor.get_manhattan_distance(*beacon) + 1;

            get_outer_edge(*sensor, bounds, distance)
        })
        .flatten()
        .unique();

    let position = edge_positions
        .filter(|pos| pos.x >= 0 && pos.y >= 0 && pos.x <= bounds.x && pos.y <= bounds.y)
        .filter(|pos| beacons_and_sensors.iter().all(|(sensor, beacon)| sensor.get_manhattan_distance(*pos) > sensor.get_manhattan_distance(*beacon)))
        .next()
        .unwrap();

    println!("Found {} {}", position.x, position.y);
    position.x as i64 * 4000000 + position.y as i64
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_invalids(parse_grid(include_str!("day15/test_simple.txt")), 10), 26);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_invalids(parse_grid(include_str!("day15/test_large.txt")), 2000000), 26);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(get_only_valid(parse_grid(include_str!("day15/test_simple.txt")), Vector2i{x: 20, y: 20}), 56000011);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(get_only_valid(parse_grid(include_str!("day15/test_large.txt")), Vector2i{x: 4000000, y: 4000000}), 11374534948438);
    }

    // #[test]
    // fn large_test() {
    //     assert_eq!(count_drops_until(parse_rocks(fs::read_to_string("src/day14/test_large.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 644);
    // }
    //
    // #[test]
    // fn simple_test_2() {
    //     assert_eq!(count_drops_until_blocked(parse_rocks(fs::read_to_string("src/day14/test_simple.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 93);
    // }
    //
    // #[test]
    // fn large_test_2() {
    //     assert_eq!(count_drops_until_blocked(parse_rocks(fs::read_to_string("src/day14/test_large.txt").unwrap().as_str()), Vector2{x: 500, y: 0}), 27324);
    // }
}