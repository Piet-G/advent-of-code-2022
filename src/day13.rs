use std::collections::{HashSet, TryReserveError};
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, fmt, ops};
use std::cmp::{Ordering, Reverse};
use std::fmt::Display;
use std::ptr::replace;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::day13::CompareResult::{Continue, Right, Wrong, ListEnd};
use crate::vector2::Vector2;

#[derive(Eq, PartialEq)]
enum ListElement {
    LIST(Vec<ListElement>),
    INT(i32),
}

#[derive(Eq, PartialEq)]
enum StringLine {
    Line(String)
}

impl StringLine {
    fn to_line(&self) -> &str {
        match self {
            StringLine::Line(str) => {
                str
            }
        }
    }
}

impl ToString for ListElement {
    fn to_string(&self) -> String {
        match self {
            ListElement::LIST(list) => ("[".to_owned() + &list.iter().map(|item| item.to_string()).join(",").as_str().to_owned() + "]"),
            ListElement::INT(int) => int.to_string()
        }
    }
}

impl PartialOrd for StringLine {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for StringLine {
    fn cmp(&self, right: &Self) -> Ordering {

        match compare_lists(recursive_list_parser(self.to_line()).0, recursive_list_parser(right.to_line()).0) {
            ListEnd => Ordering::Equal,
            Continue => Ordering::Equal,
            Right => Ordering::Less,
            Wrong => Ordering::Greater,
        }
    }
}

#[derive(PartialEq)]
enum CompareResult {
    ListEnd,
    Continue,
    Right,
    Wrong,
}

fn recursive_list_parser(string: &str) -> (Vec<ListElement>, &str) {
    let mut list = Vec::new();
    let mut iterator = string.chars();
    let mut number_str = String::from("");;

    while let Some(char) = iterator.next(){
        match char {
            '[' => {
                let (mut parsed_list, remaining_string) = recursive_list_parser(iterator.as_str());
                iterator = remaining_string.chars();
                list.push(ListElement::LIST(parsed_list))
            },
            ']' => {
                if !number_str.is_empty(){
                    list.push(ListElement::INT(number_str.parse::<i32>().unwrap() as i32))
                }
                return (list, iterator.clone().as_str())
            },
            ',' => {
                if !number_str.is_empty() {
                    list.push(ListElement::INT(number_str.parse::<i32>().unwrap() as i32));
                    number_str.clear();
                }
            },
            c => {
                number_str.push(c);
            }
        }
    }

    return (list, "");
}


fn compare_lists(left: Vec<ListElement>, right: Vec<ListElement>) -> CompareResult{

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let mut compare_result = Continue;

    while compare_result == Continue {
        let left_element = left_iter.next();
        let right_element = right_iter.next();

        compare_result = compare_optional_element(left_element, right_element)
    }

    compare_result
}

fn mutate(result: CompareResult) -> CompareResult {
    match result {
        ListEnd => Continue,
        other => other
    }
}

fn compare_optional_element(left_element: Option<ListElement> ,right_element: Option<ListElement>) -> CompareResult {
    match (left_element, right_element) {
        (None, Some(_)) => {
            Right
        },
        (Some(_), None) => {
            Wrong
        },
        (None, None) => ListEnd,
        (Some(left), Some(right)) => mutate(compare_element(left, right))
    }
}

fn compare_element(left: ListElement, right: ListElement) -> CompareResult {
    match (left, right) {
        (ListElement::INT(left_number), ListElement::INT(right_number)) => {
            if left_number == right_number {Continue} else {
                if left_number < right_number {
                    Right
                } else {
                    Wrong
                }}
        }
        (ListElement::INT(left_number), ListElement::LIST(right_list)) => {
            compare_element(ListElement::LIST(vec![ListElement::INT(left_number)]), ListElement::LIST(right_list))
        },
        (ListElement::LIST(left_list), ListElement::INT(right_number)) => {
            compare_element(ListElement::LIST(left_list), ListElement::LIST(vec![ListElement::INT(right_number)]))
        },
        (ListElement::LIST(left_list), ListElement::LIST(right_list)) => {
            compare_lists(left_list, right_list)
        }
    }
}

fn count_right_orders(lists_string: &str) -> usize {
    lists_string
        .split("\n\n")
        .enumerate()
        .map(|(i, lists_string)| {
            let mut lists = lists_string.lines().map(|line| recursive_list_parser(line));
            (i, compare_lists(lists.next().unwrap().0, lists.next().unwrap().0))
        })
        .filter(|(i, result)| *result == Right)
        .map(|(i, result)| i + 1)
        .sum()
}

fn sort(lists_string: &str) -> usize {
    let mut lines: Vec<_> = lists_string
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    lines.push("[[6]]");
    lines.push("[[2]]");


    let sorted: Vec<_> = lines
        .iter()
        .map(|line| StringLine::Line(line.parse().unwrap()))
        .sorted()
        .collect();

    return (sorted.iter().position(|StringLine::Line(line)| line == "[[2]]").unwrap() + 1) * (sorted.iter().position(|StringLine::Line(line)| line == "[[6]]").unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_right_orders(fs::read_to_string("src/day13/test_simple.txt").unwrap().as_str()), 13);
    }

    #[test]
    fn larges_test() {
        assert_eq!(count_right_orders(fs::read_to_string("src/day13/test_large.txt").unwrap().as_str()), 6395);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(sort(fs::read_to_string("src/day13/test_simple.txt").unwrap().as_str()), 140);
    }

    #[test]
    fn larges_test_2() {
        assert_eq!(sort(fs::read_to_string("src/day13/test_large.txt").unwrap().as_str()), 24921);
    }
}