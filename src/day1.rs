fn get_max_calories(calories_string: &str) -> i32 {
    return calories_string
        .split("\n\n")
        .map(|elf_list_string| get_calories_of_elf(elf_list_string))
        .max()
        .unwrap_or(0)
}

fn get_calories_of_elf(elf_calories_string: &str) -> i32 {
    return elf_calories_string
        .split("\n")
        .map(|calorie_string| calorie_string.parse::<i32>().unwrap_or(0))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calorie_count_sum_is_largest() {
        let calories_string = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(get_max_calories(calories_string), 24000);
    }

    #[test]
    fn calorie_count_when_largest_is_standalone() {
        let calories_string = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

100000";
        assert_eq!(get_max_calories(calories_string), 100000);
    }

    #[test]
    fn calorie_count_invalid_ignored() {
        let calories_string = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

100000
blabla
";
        assert_eq!(get_max_calories(calories_string), 100000);
    }

    #[test]
    fn calorie_count_nothing_found_max_is_0() {
        let calories_string = "";
        assert_eq!(get_max_calories(calories_string), 0);
    }
}
