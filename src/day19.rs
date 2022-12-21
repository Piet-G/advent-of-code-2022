use std::cmp::{max, min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Lines;
use std::ops::{Add, Sub};
use std::time::Instant;
use itertools::{enumerate, Itertools};

#[derive(Clone)]
struct State {
    robot_counts: ResourceAmount,
    resource_counts: ResourceAmount,
    length: usize,
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct ResourceAmount {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Sub<&ResourceAmount> for &ResourceAmount {
    type Output = ResourceAmount;

    fn sub(self, other: &ResourceAmount) -> Self::Output {
        ResourceAmount {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Add<&ResourceAmount> for &ResourceAmount {
    type Output = ResourceAmount;

    fn add(self, other: &ResourceAmount) -> Self::Output {
        ResourceAmount {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl PartialOrd for ResourceAmount {
    fn partial_cmp(&self, other: &ResourceAmount) -> Option<Ordering> {
        let comparisons = [
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.obsidian),
            self.geode.cmp(&other.geode)
        ];

        if comparisons.iter().all(|result| *result == Equal) {
            return Some(Equal);
        }

        if comparisons.iter().all(|result| *result == Greater || *result == Equal) {
            return Some(Greater);
        }

        if comparisons.iter().all(|result| *result == Less || *result == Equal) {
            return Some(Less);
        }

        return None;
    }
}

struct RobotCosts {
    ore: ResourceAmount,
    clay: ResourceAmount,
    obsidian: ResourceAmount,
    geode: ResourceAmount,
}

fn parse_costs(cost_string: &str) -> Vec<RobotCosts> {
    cost_string
        .lines().map(|line| {
        let relevant_line = line.split(":").skip(1).next().unwrap();
        let mut robots = relevant_line.split(".").map(|robot_cost| robot_cost.split("and").map(|cost| cost.chars().filter(|c| c.is_digit(10)).join("").parse::<usize>().unwrap()));

        let mut ore_robot = robots.next().unwrap();
        let mut clay_robot = robots.next().unwrap();
        let mut obsidian_robot = robots.next().unwrap();
        let mut geode_robot = robots.next().unwrap();

        RobotCosts {
            ore: ResourceAmount {
                ore: ore_robot.next().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay: ResourceAmount {
                ore: clay_robot.next().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian: ResourceAmount {
                ore: obsidian_robot.next().unwrap(),
                clay: obsidian_robot.next().unwrap(),
                obsidian: 0,
                geode: 0,
            },
            geode: ResourceAmount {
                ore: geode_robot.next().unwrap(),
                clay: 0,
                obsidian: geode_robot.next().unwrap(),
                geode: 0,
            },
        }
    }).collect()
}

fn already_explored(state: &State, seen_states: &HashMap<ResourceAmount, Vec<(usize, ResourceAmount)>>) -> bool {
    if let Some(resource_amounts) = seen_states.get(&state.robot_counts) {
        return resource_amounts.iter().any(|resource| {
            let result = resource.1.partial_cmp(&state.resource_counts);
            return state.length >= resource.0 && ( result == Some(Equal) || result == Some(Greater));
        });
    }

    return false;
}

fn get_max_geode_production_with_building(state: &State, at: usize) -> usize {
    let a = ((state.robot_counts.geode)..).take(at - state.length).sum::<usize>() + state.resource_counts.geode;

    let q = "";


    return a;
}

fn find_best_system(costs: RobotCosts, size: usize) -> usize {
    let mut stack = VecDeque::new();
    let mut seen_states: HashMap<ResourceAmount, Vec<(usize, ResourceAmount)>> = HashMap::new();

    stack.push_front(State {
        robot_counts: ResourceAmount {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        resource_counts: ResourceAmount {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        length: 0,
    });

    let mut counter: usize = 0;
    let mut max_geodes = 0;
    let mut min_required_obsidian = (0..=size).map(|s| get_min_required_obsidians(s, costs.geode.obsidian, size)).collect::<Vec<_>>();


    while let Some(current_state) = stack.pop_back() {
        if !already_explored(&current_state, &seen_states)
            && get_max_geode_production_with_building(&current_state, size) > max_geodes
            && !(current_state.robot_counts.geode == 0 && current_state.robot_counts.obsidian + current_state.resource_counts.obsidian < min_required_obsidian[current_state.length])

        {
            let now = Instant::now();
            if counter % 1000 == 0 {
                println!("Max: {}, {}, {}", max_geodes, current_state.length, stack.len());
            }

            if current_state.length == size {
                max_geodes = max(max_geodes, current_state.resource_counts.geode);
            }

            if let Some(seen_with_current_robots) = seen_states.get_mut(&current_state.robot_counts) {
                seen_with_current_robots.retain(|resource| resource.0 < current_state.length || resource.1.partial_cmp(&current_state.resource_counts) == None);
                seen_with_current_robots.push((current_state.length, current_state.resource_counts.clone()));
            }
            else {
                seen_states.insert(current_state.robot_counts.clone(), vec![(current_state.length, current_state.resource_counts.clone())]);
            }

            stack.append(&mut get_nexts(current_state, &costs, size));
            counter += 1;

            if(counter % 100000 == 0) {
                println!("Time: {}", now.elapsed().as_micros());
            }
        }
    }

    max_geodes
}

fn get_score(costs: Vec<RobotCosts>, size: usize) -> usize {
    costs.into_iter().enumerate().map(|(i, el)| find_best_system(el, size) * (i + 1)).sum()
}

fn get_large_score(costs: Vec<RobotCosts>, size: usize) -> usize {
    costs.into_iter().enumerate().take(3).map(|(i, el)| find_best_system(el, size)).reduce(|a, b| a * b).unwrap()
}

fn get_min_required_obsidians(index: usize, cost: usize, size: usize) -> usize {
    let from_end = size - index;

    return cost - min(cost, (0..).take(from_end).sum::<usize>());
}

fn calculate_next_base_state(state: &State) -> State {
    let mut next = state.clone();

    next.resource_counts = &next.resource_counts + &state.robot_counts;
    next.length += 1;

    next
}

fn get_nexts(state: State, costs: &RobotCosts, size: usize) -> VecDeque<State> {
    let mut nexts = VecDeque::new();

    if state.length < size {
        if state.resource_counts >= costs.ore && state.length < size - 2 {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.ore += 1;
            next.resource_counts = &next.resource_counts - &costs.ore;

            nexts.push_front(next);
        }

        if state.resource_counts >= costs.clay && state.length < size - 1 {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.clay += 1;
            next.resource_counts = &next.resource_counts - &costs.clay;

            nexts.push_back(next);
        }

        if nexts.len() == 2 && state.robot_counts.clay == 0 {
            return nexts;
        }

        if state.resource_counts >= costs.obsidian && state.length < size - 1 {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.obsidian += 1;
            next.resource_counts = &next.resource_counts - &costs.obsidian;

            nexts.push_back(next);
        }

        if nexts.len() == 3 && state.robot_counts.obsidian == 0 {
            return nexts;
        }

        if state.resource_counts >= costs.geode {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.geode += 1;
            next.resource_counts = &next.resource_counts - &costs.geode;

            nexts.push_back(next);
        }

        if nexts.len() == 4 {
            return nexts;
        }

        nexts.push_front(calculate_next_base_state(&state));
    }

    nexts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let costs = parse_costs(include_str!("day19/test_simple.txt"));

        assert_eq!(get_score(costs, 24), 33);
    }


    #[test]
    fn simple_test_2() {
        let costs = parse_costs(include_str!("day19/test_simple.txt"));

        assert_eq!(get_score(costs, 32), 33);
    }

    #[test]
    fn large_test_2() {
        let costs = parse_costs(include_str!("day19/test_large.txt"));

        assert_eq!(get_large_score(costs, 32), 6804);
    }

    #[test]
    fn large_test() {
        let costs = parse_costs(include_str!("day19/test_large.txt"));

        assert_eq!(get_score(costs, 24), 1659);
    }
}