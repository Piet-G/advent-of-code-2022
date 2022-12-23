fn parse_list(list_string: &str) -> Vec<Value> {
    list_string.lines()
        .enumerate()
        .map(|(i, item)| Value{value: item.parse().unwrap(), index: i})
        .collect()
}

fn get_at_index(list: &Vec<Value>, i: i64) -> i64 {
    return list[(i as usize % list.len())].value;
}

fn permute_list(list: Vec<Value>, amount: usize) -> (i64, i64, i64) {
    let original_list = list.clone();
    let mut permuted_list = list.clone();


    let size = list.len() as i64 - 1;

    for _ in 0..amount {
        for i in 0..original_list.len() {
            let from = permuted_list.iter().position(|el| el.index == i).unwrap();
            let value = permuted_list.remove(from);
            let to = ((from as i64 + value.value) % size + size) % size;

            permuted_list.insert(to as usize, value);
        }
    }

    let zero_index = permuted_list.iter().position(|el| el.value == 0).unwrap() as i64;

    println!("List {}", permuted_list.iter().map(|i| i.value.to_string()).collect::<Vec<_>>().join(","));
    (get_at_index(&permuted_list, zero_index + 1000), get_at_index(&permuted_list, zero_index + 2000), get_at_index(&permuted_list, zero_index + 3000))
}

fn apply_key(cost: Vec<Value>) -> Vec<Value> {
    cost.into_iter().map(|val| Value{value: val.value * 811589153, index: val.index}).collect()
}

use std::fs;

#[derive(Clone, Copy)]
struct Value{
    value: i64,
    index: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let costs = parse_list(include_str!("day20/test_simple.txt"));
        let result = permute_list(costs, 1);
        assert_eq!(result.0 + result.1 + result.2, 3);
    }

    #[test]
    fn large_test() {
        let costs = parse_list(include_str!("day20/test_large.txt"));
        let result = permute_list(costs, 1);
        assert_eq!(result.0 + result.1 + result.2, 27726);
    }

    #[test]
    fn large_test_2() {
        let costs = apply_key(parse_list(include_str!("day20/test_large.txt")));
        let result = permute_list(costs, 10);
        assert_eq!(result.0 + result.1 + result.2, 27726);
    }
}