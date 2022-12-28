use std::ops::Add;
use itertools::Itertools;

struct Snafu {
    numbers: Vec<i32>
}

impl Add for Snafu {
    type Output = Snafu;

    fn add(self, other: Self) -> Self::Output {
        let mut added = vec![0; 100];

        for i in 0..added.len() {
            added[i] = self.numbers.get(i).unwrap_or(&0) + other.numbers.get(i).unwrap_or(&0);
        }

        for i in 0..added.len() {
            if added[i] < -2 {
                added[i] += 5;
                added[i + 1] -= 1;
            }
        }

        for i in 0..added.len() {
            if added[i] >= 5 {
                added[i] %= 5;
                added[i + 1] += 1
            }
        }

        for i in 0..added.len() {
            if added[i] >= 3 {
                added[i] = added[i] - 5;
                added[i + 1] += 1
            }
        }

        return Snafu {
            numbers: added
        }
    }
}

impl Snafu {
    fn to_snafu_char(number: i32) -> char {
        match number {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!()
        }
    }

    fn from_snafu_char(char: char) -> i32 {
        match char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!()
        }
    }

    fn to_snafu_string(&self) -> String {
        self.numbers.iter().map(|number| Snafu::to_snafu_char(*number)).rev().join("").trim_start_matches(|char| char == '0').to_string()
    }

    fn parse_from(string: &str) -> Snafu {
        Snafu{
            numbers: string.chars().rev().map(|char| Snafu::from_snafu_char(char)).collect()
        }
    }
}

fn get_sum_of_snafus(string: &str) -> Snafu {
    string.lines().map(Snafu::parse_from).reduce(|a, b| a + b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut sum = get_sum_of_snafus(include_str!("day25/test_simple.txt"));

        assert_eq!(sum.to_snafu_string(), "2=-1=0")
    }

    #[test]
    fn large_test() {
        let mut sum = get_sum_of_snafus(include_str!("day25/test_large.txt"));

        assert_eq!(sum.to_snafu_string(), "2=-0=1-0012-=-2=0=01")
    }
}