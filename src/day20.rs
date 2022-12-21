fn parse_list(list_string: &str) -> Vec<i32> {
    list_string.lines()
        .map(|item| item.parse().unwrap())
        .collect()
}

fn move_me(arr: &mut Vec<i32>, old_index: usize, new_index: usize) {
    if old_index < new_index {
        let removed = arr.remove(old_index);
        arr.insert(new_index, removed);
    } else {
        let removed = arr.remove(old_index);
        arr.insert(new_index , removed);
    }
}

fn get_at_index(list: &Vec<i32>, i: i32) -> i32 {
    return list[wrap(i, list.len())]
}

fn wrap(i: i32, len: usize) -> usize{
    let len_i = len as i32;
    let mut changed_i = i;

    if(i < 0){
        changed_i = i - 1;
    }

    if i >= len_i {
        changed_i = i + 1;
    }

    (((changed_i  % len_i) + len_i) % len_i) as usize
}

fn permute_list(list: Vec<i32>) -> (i32, i32, i32) {
    let original_list = list.clone();
    let mut permuted_list = list.clone();

    for number in original_list {
        let from = permuted_list.iter().position(|el| *el == number).unwrap();
        let to = wrap(from as i32 + number, permuted_list.len());

        println!("Move {} From: {}, To: {}", number, from, to);
        move_me(&mut permuted_list, from, to);
        println!("List {}, Move {} From: {}, To: {}", permuted_list.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","), number, from, to);

    }

    let zero_index = permuted_list.iter().position(|el| *el == 0).unwrap() as i32;

    println!("List {}", permuted_list.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
    (get_at_index(&permuted_list, zero_index + 1000), get_at_index(&permuted_list, zero_index + 2000), get_at_index(&permuted_list, zero_index + 3000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let costs = parse_list(include_str!("day20/test_simple.txt"));

        assert_eq!(permute_list(costs), (4, -3, 2));
    }

    #[test]
    fn large_test() {
        let costs = parse_list(include_str!("day20/test_large.txt"));
        let result = permute_list(costs);
        assert_eq!(result.0 + result.1 + result.2, 0);
    }
}