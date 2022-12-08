use std::collections::HashSet;
use std::rc::{Rc, Weak};
use regex::{Captures, Match, Regex};

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, Range};
use std::cmp;

struct Tree {
    visible: bool,
    height: u32,
    score: usize,
}

type TreeGrid = Vec<Vec<Tree>>;

fn parse_grid(str: &str) -> TreeGrid {
    str.lines().map(|line| line.chars().map(|char| Tree {
        height: char.to_digit(10).unwrap(),
        visible: false,
        score: 1,
    }).collect()).collect()
}

fn process_row_1(tree_grid: &mut TreeGrid, row_indices: Vec<usize>, col_indices: Vec<usize>) {
    let mut previous_heights: Vec<u32> = Vec::new();
    for row in &row_indices {
        for col in &col_indices {
            let mut tree: &mut Tree = &mut tree_grid[*row][*col];

            if previous_heights.iter().all(|prev_height| tree.height > *prev_height) {
                tree.visible = true
            }

            previous_heights.push(tree.height);
        }
    }
}

fn process_row_2(tree_grid: &mut TreeGrid, row_indices: Vec<usize>, col_indices: Vec<usize>) {
    let mut previous_heights: Vec<u32> = Vec::new();
    for row in &row_indices {
        for col in &col_indices {
            let mut tree: &mut Tree = &mut tree_grid[*row][*col];
            let direction_score = previous_heights
                .iter()
                .rev()
                .position(|prev_height| tree.height <= *prev_height)
                .map(|score| score + 1)
                .unwrap_or(previous_heights.len());

            tree.score *= direction_score;
            previous_heights.push(tree.height);
        }
    }
}

fn count_visible_trees(mut tree_grid: TreeGrid) -> usize {
    let col_count = tree_grid[0].len();
    let row_count = tree_grid.len();

    for col in 0..col_count {
        process_row_1(&mut tree_grid, (0..row_count).collect(), vec![col]);
        process_row_1(&mut tree_grid, (0..row_count).rev().collect(), vec![col]);
    }


    for row in 0..row_count {
        process_row_1(&mut tree_grid, vec![row], (0..col_count).collect());
        process_row_1(&mut tree_grid, vec![row], (0..col_count).rev().collect());
    }

    return tree_grid.iter().map(|row| row.iter().filter(|tree| tree.visible).count()).sum();
}

fn get_max_score(mut tree_grid: TreeGrid) -> usize {
    let col_count = tree_grid[0].len();
    let row_count = tree_grid.len();

    for col in 0..col_count {
        process_row_2(&mut tree_grid, (0..row_count).collect(), vec![col]);
        process_row_2(&mut tree_grid, (0..row_count).rev().collect(), vec![col]);
    }


    for row in 0..row_count {
        process_row_2(&mut tree_grid, vec![row], (0..col_count).collect());
        process_row_2(&mut tree_grid, vec![row], (0..col_count).rev().collect());
    }


    return tree_grid.iter().map(|row| row.iter().map(|tree| tree.score).max().unwrap()).max().unwrap();
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(count_visible_trees(parse_grid(fs::read_to_string("src/day8/test_simple.txt").unwrap().as_mut_str())), 21);
    }

    #[test]
    fn large_test() {
        assert_eq!(count_visible_trees(parse_grid(fs::read_to_string("src/day8/test_large.txt").unwrap().as_mut_str())), 1816);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(get_max_score(parse_grid(fs::read_to_string("src/day8/test_simple.txt").unwrap().as_mut_str())), 8);
    }

    #[test]
    fn large_test_2() {
       assert_eq!(get_max_score(parse_grid(fs::read_to_string("src/day8/test_large.txt").unwrap().as_mut_str())), 21);
    }
}