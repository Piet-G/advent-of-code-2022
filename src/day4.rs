use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};
use std::str::Lines;
use itertools::Itertools;

fn count_overlapping_assignments(assignments_string: String, overlap_function: fn(&RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool) -> usize {
    return assignments_string
        .lines()
        .map(|schedule_line| do_schedules_overlap(schedule_line, overlap_function))
        .filter(|result| *result)
        .count()
}

fn do_schedules_overlap(schedule_line: &str, overlap_function: fn(&RangeInclusive<i32>, &RangeInclusive<i32>) -> bool) -> bool {
    return schedule_line
        .split(",")
        .map(|schedule_string| schedule_to_range(schedule_string))
        .combinations(2)
        .any(|combination| overlap_function(&combination[0], &combination[1]) || overlap_function(&combination[1], &combination[0]));
}

fn is_subrange(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    return range2.contains(range1.start()) && range2.contains(range1.end());
}

fn is_overlapping_at_all(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    return range2.contains(range1.start()) || range2.contains(range1.end());
}

fn schedule_to_range(schedule: &str) -> RangeInclusive<i32> {
    let mut values = schedule.split("-").map(|value| value.parse::<i32>().unwrap());

    return values.next().unwrap()..=values.next().unwrap()
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_overlapping_assignments(fs::read_to_string("src/day4/test_simple.txt").unwrap(), is_subrange), 2);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_overlapping_assignments(fs::read_to_string("src/day4/test_large.txt").unwrap(), is_subrange), 462);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(count_overlapping_assignments(fs::read_to_string("src/day4/test_simple.txt").unwrap(), is_overlapping_at_all), 4);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(count_overlapping_assignments(fs::read_to_string("src/day4/test_large.txt").unwrap(), is_overlapping_at_all), 835);
    }
}