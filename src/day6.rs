use std::collections::HashSet;

fn get_indicator_index(string: &str, window_size: usize) -> usize {
   return string
       .chars()
       .collect::<Vec<_>>()
       .windows(window_size)
       .position(|window| HashSet::<&char>::from_iter(window.into_iter()).len() == window_size)
       .unwrap() + window_size;
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(get_indicator_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(get_indicator_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_indicator_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_indicator_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_indicator_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
        assert_eq!(get_indicator_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn large_test() {
       assert_eq!(get_indicator_index(fs::read_to_string("src/day6/test_large.txt").unwrap().as_mut_str(), 4), 1757);
       assert_eq!(get_indicator_index(fs::read_to_string("src/day6/test_large.txt").unwrap().as_mut_str(), 14), 2950);
    }
}