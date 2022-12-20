use std::cmp::{max, min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashSet, VecDeque};
use std::io::Lines;
use std::ops::{Add, Sub};
use itertools::{enumerate, Itertools};

#[derive(Clone)]
struct State {
    robot_counts: ResourceAmount,
    resource_counts: ResourceAmount,
    length: usize
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

        if comparisons.iter().all(|result| *result == Greater || *result == Equal) {
            return Some(Greater);
        }

        if comparisons.iter().all(|result| *result == Less || *result == Equal) {
            return Some(Less);
        }

        if comparisons.iter().all(|result| *result == Equal) {
            return Some(Equal);
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
                    geode: 0
                },
                geode: ResourceAmount {
                    ore: geode_robot.next().unwrap(),
                    clay: 0,
                    obsidian: geode_robot.next().unwrap(),
                    geode: 0,
                }
            }
    }).collect()
}

fn find_best_system(costs: RobotCosts) -> usize{
    let mut stack = VecDeque::new();
    let mut seen_states = HashSet::new();

    stack.push_front(State{
        robot_counts: ResourceAmount {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0
        },
        resource_counts: ResourceAmount {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0
        },
        length: 0
    });

    let mut counter: usize = 0;
    let mut max_geodes = 0;

    while let Some(current_state) = stack.pop_front() {
        let a = get_min_required_obsidians(23, costs.geode.obsidian);
        if !(current_state.robot_counts.geode + current_state.resource_counts.geode == 0 && current_state.robot_counts.obsidian + current_state.resource_counts.obsidian < get_min_required_obsidians(current_state.length, costs.geode.obsidian)) && !seen_states.contains(&(current_state.robot_counts.clone(), current_state.resource_counts.clone())) {
            if counter % 1000 == 0 {
                println!("{}, {}",current_state.length, stack.len());
            }

            max_geodes = max(max_geodes, current_state.resource_counts.geode);

            seen_states.insert((current_state.robot_counts.clone(), current_state.resource_counts.clone()));

            stack.append(&mut get_nexts(current_state, &costs));
            counter += 1;
        }

    }

    max_geodes
}

fn get_score(costs: Vec<RobotCosts>) -> usize {
    costs.into_iter().enumerate().map(|(i, el)| find_best_system(el) * (i + 1)).sum()
}

fn get_min_required_obsidians(index: usize, cost: usize) -> usize {
    let from_end = 24 - index;

    return cost - min(cost, (0..).take(from_end).sum::<usize>());
}


fn calculate_next_base_state(state: &State) -> State {
    let mut next = state.clone();

    next.resource_counts = &next.resource_counts + &state.robot_counts;
    next.length += 1;

    next
}

fn get_nexts(state: State, costs: &RobotCosts) -> VecDeque<State> {
    let mut nexts = VecDeque::new();

    if state.length < 24 {
        if state.resource_counts >= costs.ore && state.length < 22 {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.ore += 1;
            next.resource_counts = &next.resource_counts - &costs.ore;

            nexts.push_front(next);
        }

        if state.resource_counts >= costs.clay && state.length < 23 {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.clay += 1;
            next.resource_counts = &next.resource_counts - &costs.clay;

            nexts.push_front(next);
        }

        if state.resource_counts >= costs.obsidian && state.length < 23{
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.obsidian += 1;
            next.resource_counts = &next.resource_counts - &costs.obsidian;

            nexts.push_front(next);
        }

        if state.resource_counts >= costs.geode {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.geode += 1;
            next.resource_counts = &next.resource_counts - &costs.geode;

            nexts.push_front(next);
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

        assert_eq!(get_score(costs), 33);
    }

    #[test]
    fn large_test() {
        let costs = parse_costs(include_str!("day19/test_large.txt"));

        assert_eq!(get_score(costs), 1659);
    }
}