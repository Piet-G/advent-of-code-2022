use std::collections::HashSet;
use std::ops::Add;
use crate::vector3::Vector3;

type CubePosition = Vector3<i32>;

fn parse_positions(positions_string: &str) -> HashSet<CubePosition> {
    positions_string.lines().map(|line| {
        let mut elements = line.split(",").map(|str| str.parse::<i32>().unwrap());

        Vector3 {
            x: elements.next().unwrap(),
            y: elements.next().unwrap(),
            z: elements.next().unwrap(),
        }
    }).collect()
}

fn add_outside(initial: CubePosition, cubes: &HashSet<CubePosition>, outside: &mut HashSet<CubePosition>){

    let mut to_do = Vec::new();

    to_do.push(initial);

    while let Some(pos) = to_do.pop() {
        if !cubes.contains(&pos) && !outside.contains(&pos) && pos.x >= -5 && pos.y >= -5 && pos.z >= -5 && pos.x <= 25 && pos.y <= 25 && pos.z <= 25 {
            outside.insert(pos);
            println!("Added {},{},{}", pos.x, pos.y, pos.z);

            if(pos.x == 2 && pos.y == 2 && pos.z == 5){
                println!("Really? {}", !cubes.contains(&pos));
                let a = 0;
            }

            to_do.push(pos + Vector3 { x: 1, y: 0, z: 0 });
            to_do.push(pos + Vector3 { x: -1, y: 0, z: 0 });
            to_do.push(pos + Vector3 { x: 0, y: 1, z: 0 });
            to_do.push(pos + Vector3 { x: 0, y: -1, z: 0 });
            to_do.push(pos + Vector3 { x: 0, y: 0, z: 1 });
            to_do.push(pos + Vector3 { x: 0, y: 0, z: -1 });
        }

    }
}

fn count_non_touching_edges(cubes: HashSet<CubePosition>) -> usize{
    let mut outside = HashSet::new();

    add_outside(Vector3{x: -5, y: -5, z: -5}, &cubes, &mut outside);



    println!("Does it? {}", outside.contains(&Vector3{x: 2, y: 2, z: 5}));
    cubes.iter().map(|cube|
        [
            Vector3 { x: -1, y: 0, z: 0 },
            Vector3 { x: 1, y: 0, z: 0 },
            Vector3 { x: 0, y: 1, z: 0 },
            Vector3 { x: 0, y: -1, z: 0 },
            Vector3 { x: 0, y: 0, z: 1 },
            Vector3 { x: 0, y: 0, z: -1 },
        ].map(|vec| vec.add(*cube)).into_iter().filter(|pos| !cubes.contains(pos) && outside.contains(pos)).count()).sum()
}

fn is_not_edge(position: Vector3<i32>, cubes: &HashSet<CubePosition>) -> bool {
    if !(0..=position.x).any(|x| cubes.contains(&Vector3 { x, y: position.y, z: position.z })) {
        return false;
    }

    if !(0..=position.y).any(|y| cubes.contains(&Vector3 { x: position.x, y, z: position.z })) {
        return false;
    }

    if !(0..=position.z).any(|z| cubes.contains(&Vector3 { x: position.x, y: position.y, z })) {
        return false;
    }

    if !(position.x..=20).any(|x| cubes.contains(&Vector3 { x, y: position.y, z: position.z })) {
        return false;
    }

    if !(position.y..=20).any(|y| cubes.contains(&Vector3 { x: position.x, y, z: position.z })) {
        return false;
    }

    if !(position.z..=20).any(|z| cubes.contains(&Vector3 { x: position.x, y: position.y, z })) {
        return false;
    }

    return true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let cubes = parse_positions(include_str!("day18/test_simple.txt"));
        assert_eq!(count_non_touching_edges(cubes), 64);
    }

    #[test]
    fn large_test() {
        let cubes = parse_positions(include_str!("day18/test_large.txt"));
        assert_eq!(count_non_touching_edges(cubes), 64);
    }

}