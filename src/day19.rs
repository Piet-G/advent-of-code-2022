use std::cmp::Ordering;
use std::io::Lines;
use std::ops::{Add, Sub};

#[derive(Clone)]
struct State {
    robot_counts: ResourceAmount,
    resource_counts: ResourceAmount,
    length: usize
}

#[derive(Eq, PartialEq, Clone)]
struct ResourceAmount {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Sub for ResourceAmount {
    type Output = ResourceAmount;

    fn sub(&self, other: &ResourceAmount) -> Self::Output {
        ResourceAmount {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Add for ResourceAmount {
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
    fn partial_cmp(self, other: &ResourceAmount) -> Option<Ordering> {
        [
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.clay),
            self.geode.cmp(&other.geode)
        ].iter().reduce(|(a,b)| if a == b {a} else {None})
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
            let mut robots = line.split(".").map(|robot_cost| robot_cost.split("and").map(|cost| cost.chars().filter(|c| c.is_digit(10)).collect().parse::<usize>()));

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

fn find_best_system(costs: &Vec<RobotCosts>) {
    let mut stack = vec![State{
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
    }];

    while Some(current_state) = stack.pop() {
        stack.append(&mut get_nexts(current_state, &costs[0]))
    }
}

fn calculate_next_base_state(state: &State) -> State {
    let mut next = state.clone();

    next.resource_counts = next.resource_counts + &state.robot_counts;
    next.length += 1;

    next
}

fn get_nexts(state: State, costs: &RobotCosts) -> Vec<State> {
    let mut nexts = vec![];

    if state.length < 26 {
        if state.resource_counts >= costs.ore {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.ore += 1;
            next.resource_counts = next.resource_counts - &costs.ore;

            nexts.push(next);
        }

        if state.resource_counts >= costs.clay {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.clay += 1;
            next.resource_counts = next.resource_counts - &costs.clay;

            nexts.push(next);
        }

        if state.resource_counts >= costs.obsidian {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.obsidian += 1;
            next.resource_counts = next.resource_counts - &costs.obsidian;

            nexts.push(next);
        }

        if state.resource_counts >= costs.geode {
            let mut next = calculate_next_base_state(&state);

            next.robot_counts.geode += 1;
            next.resource_counts = next.resource_counts - &costs.geode;

            nexts.push(next);
        }
    }

    nexts.push(calculate_next_base_state(&state));

    nexts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let costs = parse_costs(include_str!("day18/test_simple.txt"));
        find_best_system(&costs);
        //assert_eq!(drop_shapes(shapes, jets, 7, 2022), 3068);
    }
}