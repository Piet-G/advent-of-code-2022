use std::collections::HashSet;
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::{cmp, ops};
use itertools::Itertools;

fn execute_program(program_string: &str) -> Vec<i32> {
    program_string
        .lines()
        .fold(vec![1], |mut cycles, line | {
            cycles.append(&mut execute_line(line, *cycles.last().unwrap()));
            cycles
        })
}

fn execute_line(program_line: &str, current_x: i32) -> Vec<i32> {
    let mut tokens = program_line.split(" ");

    return match tokens.next().unwrap() {
        "noop" => vec![current_x],
        "addx" => vec![current_x, current_x + tokens.next().unwrap().parse::<i32>().unwrap()],
        _ => panic!()
    }
}

fn count_signal_strength(results: Vec<i32>) -> i32 {
    let results_vec = results.iter().enumerate().skip(19).step_by(40).collect::<Vec<_>>();
    return results_vec.into_iter().map(|(i, el)| el * ((i + 1) as i32)).sum();
}

fn crt(results: Vec<i32>, line_width: usize) -> String {
     results
        .into_iter().enumerate()
        .chunks(line_width).into_iter()
        .map(|line| line.map(|(i, el) | to_pixel(i, el, line_width)).join(""))
        .join("\n")
}

fn to_pixel(i: usize, el: i32, line_width: usize) -> &'static str {
    let i_int =  i as i32;
    let line_width_int = line_width as i32;
    if (i_int % line_width_int - 1)  <= el && el <= (i_int % line_width_int + 1)  {"#"} else { "." }
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_signal_strength(execute_program(fs::read_to_string("src/day10/test_simple.txt").unwrap().as_str())), 13140);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_signal_strength(execute_program(fs::read_to_string("src/day10/test_large.txt").unwrap().as_str())), 13140);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(crt(execute_program(fs::read_to_string("src/day10/test_simple.txt").unwrap().as_str()), 40),
                   "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
."
        );
    }

    #[test]
    fn large_test_2() { println!("{}", crt(execute_program(fs::read_to_string("src/day10/test_large.txt").unwrap().as_str()), 40));
    }
}