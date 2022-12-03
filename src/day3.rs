use std::collections::HashSet;
use std::str::Lines;

fn get_all_priorities(rucksack_string: String) -> i32 {
    return rucksack_string
        .lines()
        .map(|rucksack_line| get_rucksack_priority(rucksack_line))
        .sum();
}

fn get_rucksack_priority(rucksack_string: &str) -> i32 {
    let half_length = (rucksack_string.len() / 2);
    let chars: Vec<char> = rucksack_string.chars().collect();
    let compartments = [&chars[0..half_length], &chars[half_length..rucksack_string.len()]];
    let charsets = compartments
        .map(|chars| get_charset(chars.to_vec()));

    return charsets[0].intersection(&charsets[1]).map(|char| char_to_priority(char.clone()).unwrap_or(0)).sum();
}

fn get_rucksack_priority_2(rucksack_string: &[&str]) -> Option<i32>{
    let mut charsets = rucksack_string
        .iter()
        .map(|rucksack_line| get_charset(rucksack_line.chars().collect()));

    let intersection = charsets.next().map(|set| charsets.fold(set, |set1, set2| &set1 & &set2))?;
    return char_to_priority(intersection.iter().next()?.clone())
}

fn get_rucksack_group_priority(rucksacks_string: String) -> Option<i32> {
    return rucksacks_string
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|rucksack| get_rucksack_priority_2(rucksack)).sum();
}

fn get_charset(chars: Vec<char>) -> HashSet<char> {
    let mut char_set = HashSet::new();

    for char in chars{
        char_set.insert(char.clone());
    }

    return char_set;
}

fn char_to_priority(char: char) -> Option<i32> {
    let result = Some(lowercase_char_to_priority(char).unwrap_or(lowercase_char_to_priority(char.to_lowercase().next()?)? + 26));

    return result;
}

fn lowercase_char_to_priority(char: char) -> Option<i32> {
    let mut alphabet = ('a'..='z');
    return Some(alphabet.position(|letter| letter == char)? as i32 + 1)
}
#[cfg(test)]
mod tests {
    use std::{env, fs};
    use super::*;

    #[test]
    fn simple_test_1() {
        assert_eq!(get_all_priorities(fs::read_to_string("src/day3/test_1_small.txt").unwrap()), 157);
    }

    #[test]
    fn large_test_1() {
        assert_eq!(get_all_priorities(fs::read_to_string("src/day3/test_1_large.txt").unwrap()), 7908);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(get_rucksack_group_priority(fs::read_to_string("src/day3/test_1_small.txt").unwrap()), Some(70));
    }

    #[test]
    fn large_test_2() {
        assert_eq!(get_rucksack_group_priority(fs::read_to_string("src/day3/test_1_large.txt").unwrap()), Some(2838));
    }
}