use std::collections::HashSet;
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, ops};
use itertools::Itertools;

struct Monkey {
    items: Vec<i64>,
    operation: OperationExpression,
    test: ModuloTest,
    true_result: usize,
    false_result: usize,
    inspected_count: usize,
}

trait Expression {
    fn resolve(&self, old: i64) -> i64;
}

struct VariableExpression {}

impl Expression for VariableExpression {
    fn resolve(&self, old: i64) -> i64 {
        old
    }
}

struct OperationExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: fn(i64, i64) -> i64
}

impl Expression for OperationExpression {
    fn resolve(&self, old: i64) -> i64 {
        (self.operator)(self.left.resolve(old), self.right.resolve(old))
    }
}

struct LiteralExpression {
    literal: i64
}

impl Expression for LiteralExpression {
    fn resolve(&self, old: i64) -> i64 {
        self.literal
    }
}

struct ModuloTest{
    number: i64
}

impl ModuloTest {
    fn resolve(&self, worry: i64) -> bool {
        worry % self.number == 0
    }
}

fn parse_monkies(monkies_string: &str) -> Vec<Monkey> {
    monkies_string
        .split("\n\n")
        .map(|monkey_string| parse_monkey(monkey_string))
        .collect()
}

fn parse_monkey(monkey_string: &str) -> Monkey {
    let mut monkey_lines = monkey_string
        .lines()
        .skip(1)
        .map(|line| line.trim());

    let items = monkey_lines
        .next()
        .unwrap()
        .replace("Starting items: ", "")
        .split(",")
        .map(|number| number.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let operation = parse_operation(monkey_lines.next().unwrap());
    let test = parse_test(monkey_lines.next().unwrap());

    let true_result: usize = monkey_lines.next().unwrap().replace("If true: throw to monkey ", "").parse().unwrap();
    let false_result: usize = monkey_lines.next().unwrap().replace("If false: throw to monkey ", "").parse().unwrap();

    Monkey{
        items,
        operation,
        test,
        true_result,
        false_result,
        inspected_count: 0
    }
}

fn parse_test(test_string: &str) -> ModuloTest {
    ModuloTest {number: test_string.replace("Test: divisible by ", "").parse().unwrap()}
}

fn parse_operation(operation_string: &str) -> OperationExpression {
    let string = operation_string.replace("Operation: new = ", "");
    let mut operation_tokens = string.split(" ");

    let left = parse_number_token(operation_tokens.next().unwrap());
    let operator: fn(i64, i64) -> i64 = match operation_tokens.next().unwrap() {
        "*" => |a,b| a * b,
        "+" => |a,b| a + b,
        _ => panic!()
    };
    let right = parse_number_token(operation_tokens.next().unwrap());

    OperationExpression {
        left,
        right,
        operator
    }
}

fn parse_number_token(number_token: &str) -> Box<dyn Expression> {
    match number_token {
        "old" => Box::new(VariableExpression {}),
        number => Box::new(LiteralExpression{literal: number.parse().unwrap()})
    }
}

fn execute_monkies(mut monkies: Vec<Monkey>, rounds: usize, divisor: i64) -> usize {
     for _ in 0..rounds{
         for i in 0..monkies.len() {
             let mut mutable_monkies: &mut Vec<_> = &mut monkies;
             let monkey = &mutable_monkies[i];

             let thrown_items = monkey.items
                 .iter()
                 .map(|item| (monkey.operation.resolve(*item )/ divisor) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23))
                 .map(|item| (if monkey.test.resolve(item) {monkey.true_result} else {monkey.false_result}, item))
                 .collect::<Vec<_>>();

            for (index, item) in thrown_items{
                mutable_monkies[index].items.push(item);
                mutable_monkies[i].inspected_count += 1;
            }

             mutable_monkies[i].items.clear()
         }
     }

    let test : Vec<_> = monkies
        .into_iter()
        .map(|monkey| monkey.inspected_count)
        .collect();

    test.into_iter()
        .sorted()
        .rev()
        .take(2)
        .reduce(|a,b| (a) * (b))
        .unwrap()
}


#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(execute_monkies(parse_monkies(fs::read_to_string("src/day11/test_simple.txt").unwrap().as_str()), 20, 3), 10605);
    }

    #[test]
    fn large_test() {
        assert_eq!(execute_monkies(parse_monkies(fs::read_to_string("src/day11/test_large.txt").unwrap().as_str()), 20, 3), 56350);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(execute_monkies(parse_monkies(fs::read_to_string("src/day11/test_simple.txt").unwrap().as_str()), 10000,1), 2713310158);
    }

    #[test]
    fn large_test_2() {
        assert_eq!(execute_monkies(parse_monkies(fs::read_to_string("src/day11/test_large.txt").unwrap().as_str()), 10000,1), 13954061248);
    }
}