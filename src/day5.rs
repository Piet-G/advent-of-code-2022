use std::iter::{Skip, StepBy};
use std::str::Chars;
use itertools::chain;
use regex;

fn get_tops(text: String, command_executor: fn(stacks: Vec<Vec<char>>, commands: Vec<(usize, usize, usize)>) -> String) -> String {
    let mut text_parts = text.split("\n\n");
    let stacks = text_parts.next().unwrap();
    let moves = text_parts.next().unwrap();

    let stacks = parse_stacks(stacks);
    let commands = parse_commands(moves);
    return command_executor(stacks, commands);
}

fn parse_commands(commands_string: &str) -> Vec<(usize, usize, usize)> {
    let split_regex = regex::Regex::new(r"move|from|to").unwrap();

    return commands_string
        .lines()
        .map(|line| split_regex.split(line).map(|str| str.trim()).skip(1))
        .map(|mut split| (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap()))
        .collect();
}

fn parse_stacks(stacks_string: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

   for stack_layer in stacks_string.lines().rev().skip(1) {
       for (i, el) in stack_layer.chars().skip(1).step_by(4).enumerate(){
           if stacks.len() <= i {
               stacks.push(Vec::new());
           }

           if !el.is_whitespace(){
               stacks[i].push(el);
           }
       }
   }

    return stacks;
}

fn execute_commands_one_at_time(mut stacks: Vec<Vec<char>>, commands: Vec<(usize, usize, usize)>) -> String{
    for command in commands {
        for _ in 0..command.0{
            let element = stacks[command.1 - 1].pop().unwrap();
            stacks[command.2 - 1].push(element);
        }
    }

    return stacks.iter().map(|stack| stack.last().unwrap().clone()).collect()
}

fn execute_commands_all_at_once(mut stacks: Vec<Vec<char>>, commands: Vec<(usize, usize, usize)>) -> String{
    for command in commands {
        let final_length = stacks[command.1 - 1].len().saturating_sub(command.0);
        let tail = stacks[command.1 - 1].split_off(final_length);
        stacks[command.2 - 1].extend(tail);

    }

    return stacks.iter().map(|stack| stack.last().unwrap().clone()).collect()
}


#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(get_tops(fs::read_to_string("src/day5/test_simple.txt").unwrap(), execute_commands_one_at_time), "CMZ");
    }

    #[test]
    fn large_test() {
        assert_eq!(get_tops(fs::read_to_string("src/day5/test_large.txt").unwrap(), execute_commands_one_at_time), "FRDSQRRCD");
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(get_tops(fs::read_to_string("src/day5/test_simple.txt").unwrap(), execute_commands_all_at_once), "MCD");
    }

    #[test]
    fn large_test_2() {
        assert_eq!(get_tops(fs::read_to_string("src/day5/test_large.txt").unwrap(), execute_commands_all_at_once), "HRFTQVWNN");
    }
}