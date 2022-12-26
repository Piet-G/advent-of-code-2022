use std::collections::HashMap;
use std::convert::identity;
use crate::day22::Command::{Move, RotateLeft, RotateRight};
use crate::day22::Rotation::{Left, Up};
use crate::day22::Tile::{Open, Wall};
use crate::vector2::Vector2i;

 #[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Open,
}

impl Tile {
    fn parse(string: char) -> Option<Tile> {
        match string {
            '.' => Some(Open),
            '#' => Some(Wall),
            _ => None
        }
    }
}

type World = HashMap<Vector2i, Tile>;

#[derive(PartialEq, Clone, Copy)]
enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    fn to_vector(&self) -> Vector2i {
        match self {
            Rotation::Up => Vector2i{x: 0, y: -1},
            Rotation::Right => Vector2i{x: 1, y: 0},
            Rotation::Down => Vector2i{x: 0, y: 1},
            Rotation::Left => Vector2i{x: -1, y: 0}
        }
    }

    fn rotate_right(&self) -> Rotation {
        match self {
            Rotation::Up => Rotation::Right,
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
        }
    }

    fn to_code(&self) -> i32 {
        match self {
            Rotation::Up => 3,
            Rotation::Right => 0,
            Rotation::Down => 1,
            Rotation::Left => 2,
        }
    }

    fn rotate_left(&self) -> Rotation {
        match self {
            Rotation::Up => Rotation::Left,
            Rotation::Right => Rotation::Up,
            Rotation::Down => Rotation::Right,
            Rotation::Left => Rotation::Down,
        }
    }
}

enum Command {
    RotateRight,
    RotateLeft,
    Move(i32),
}

impl Command {
    fn parse_from(string: &str) -> Vec<Command> {
        let mut commands = vec![];
        let mut digit_chars = "".to_string();

        for char in string.chars() {
            if char.is_digit(10) {
                digit_chars += char.to_string().as_str();
            }
            else {
                if !digit_chars.is_empty() {
                    commands.push(Move(digit_chars.to_string().parse().unwrap()));
                    digit_chars = "".to_string();
                }

                match char {
                    'R' => commands.push(RotateRight),
                    'L' => commands.push(RotateLeft),
                    _ => panic!()
                }
            }
        }

        if !digit_chars.is_empty() {
            commands.push(Move(digit_chars.to_string().parse().unwrap()))
        }

        return commands;
    }
}

struct You {
    pos: Vector2i,
    rot: Rotation
}

impl You {
    fn execute_command(&mut self, command: Command, world: &World, edges: &Edges, edge_length: i32) {
        match command {
            RotateRight => {
                println!("Rotate right");
                self.rot = self.rot.rotate_right()
            },
            RotateLeft => {
                println!("Rotate left");
                self.rot = self.rot.rotate_left()
            },
            Move(distance) => self.move_by( distance, world, edges, edge_length)
        }
    }

    fn move_by(&mut self, distance: i32, world: &World, edges: &Edges, edge_length: i32) {
        for _ in 0..distance {
            let (new_pos, rot) = if edges.len() > 0 {self.calculate_position_2(world, edges, edge_length)} else {self.calculate_position(world)};

            match world.get(&new_pos) {
                None => panic!(),
                Some(Wall) => return,
                Some(Open) => {
                    self.pos = new_pos;
                    self.rot = rot;
                },
            }
        }
    }

    fn calculate_position_2(&self, world: &World, edges: &Edges, edge_length: i32) -> (Vector2i, Rotation){
        let new_position = self.pos + self.rot.to_vector();

        println!("{}, {}", new_position.x, new_position.y);

        if world.contains_key(&new_position) {
            return (new_position, self.rot);
        }

        for (edge1, edge2) in edges {
            if let Some(wrapped_index) = edge1.is_wrapped(new_position, edge_length) {
                println!("{}", edge1.origin);

                let index = (edge_length) - wrapped_index;
                let nth = edge2.get_nth(index - 1, edge_length);
                return (nth + edge2.rot.rotate_right().to_vector(), edge2.rot.rotate_right());
            }

            if let Some(wrapped_index) = edge2.is_wrapped(new_position, edge_length) {
                println!("{}", edge2.origin);
                return (edge1.get_nth((edge_length) - wrapped_index - 1, edge_length) + edge1.rot.rotate_right().to_vector(), edge1.rot.rotate_right());
            }
        }

        panic!();
    }

    fn calculate_position(&self, world: &World) -> (Vector2i, Rotation){
        let new_position = self.pos + self.rot.to_vector();

        if world.contains_key(&new_position) {
            return (new_position, self.rot);
        }

        for i in 1.. {
            let position_to_check = self.pos + self.rot.to_vector() * i * -1;
            if !world.contains_key(&position_to_check) {
                return (self.pos + self.rot.to_vector() * (i - 1) * -1, self.rot);
            }
        }

        panic!();
    }
}

fn parse_file(string: &str) -> (You, World, Vec<Command>) {
    let mut iterator = string.split("\n\n");
    let (you, world) = parse_world(iterator.next().unwrap());

    (you, world, Command::parse_from(iterator.next().unwrap()))
}

fn parse_world(string: &str) -> (You, World) {
    let positions: Vec<(Vector2i, Option<Tile>)> = string
        .lines()
        .enumerate()
        .map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (Vector2i{x: (x + 1) as i32, y: (y + 1) as i32}, Tile::parse(char)))
            .filter(|(pos, result)| result.is_some())
        ).flatten().collect();

    let you = You {
        pos: positions[0].0,
        rot: Rotation::Right
    };

    let mut world = HashMap::new();

    for result in positions {
        //println!("{}, {}: {}", result.0.x, result.0.y, if(result.1.unwrap() == Open) {"."} else {"#"});
        world.insert(result.0, result.1.unwrap());
    }

    return (you, world);

}



fn decode(mut you: You, world: World, commands: Vec<Command>, edges: Edges, edge_length: i32) -> i32 {
    for command in commands {
        you.execute_command(command, &world, &edges, edge_length);
    }

    return you.pos.y * 1000 + you.pos.x * 4 + you.rot.to_code();
}

struct Edge {
    origin: Vector2i,
    rot: Rotation,
}

impl Edge {
    fn is_wrapped(&self, pos: Vector2i, edge_length: i32) -> Option<i32> {
        for i in 0..edge_length {
            if self.get_nth(i, edge_length) == pos {
                return Some(i);
            }
        }

        return None;
    }

    fn get_nth(&self, n: i32, edge_length: i32) -> Vector2i {
        let a = (self.origin) * edge_length + (self.rot.to_vector() * n) + Vector2i{x: 1, y: 1};

        a + match self.rot {
            Up => Vector2i{x: -1, y: -1},
            Rotation::Right => Vector2i{x: 0, y: -1},
            Rotation::Down => Vector2i{x: 0, y: 0},
            Left => Vector2i{x: -1, y: 0},
        }
    }
}

type Edges = Vec<(Edge, Edge)>;

#[cfg(test)]
mod tests {
    use crate::day22::Rotation::{Down, Left, Right, Up};
    use super::*;

    #[test]
    fn simple_test() {
        let (mut you, mut world, commands) = parse_file(include_str!("day22/test_simple.txt"));

        assert_eq!(decode(you, world, commands, vec![], 5), 6032)

    }

    #[test]
    fn large_test() {
        let (mut you, mut world, commands) = parse_file(include_str!("day22/test_large.txt"));

        assert_eq!(decode(you, world, commands, vec![], 50), 131052)

    }

    #[test]
    fn large_test_2() {
        let (mut you, mut world, commands) = parse_file(include_str!("day22/test_large.txt"));

        assert_eq!(decode(you, world, commands, vec![
            (
                Edge {
                    origin: Vector2i {x: 1, y: 0},
                    rot: Right,
                },
                Edge {
                    origin: Vector2i {x: 0, y: 4},
                    rot: Up,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 1, y: 1},
                    rot: Up,
                },
                Edge {
                    origin: Vector2i {x: 0, y: 3},
                    rot: Up,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 2, y: 0},
                    rot: Right,
                },
                Edge {
                    origin: Vector2i {x: 1, y: 4},
                    rot: Left,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 3, y: 1},
                    rot: Left,
                },
                Edge {
                    origin: Vector2i {x: 2, y: 1},
                    rot: Down,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 3, y: 0},
                    rot: Down,
                },
                Edge {
                    origin: Vector2i {x: 2, y: 2},
                    rot: Down,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 2, y: 3},
                    rot: Left,
                },
                Edge {
                    origin: Vector2i {x: 1, y: 3},
                    rot: Down,
                }
            ),
            (
                Edge {
                    origin: Vector2i {x: 1, y: 2},
                    rot: Up,
                },
                Edge {
                    origin: Vector2i {x: 0, y: 2},
                    rot: Right,
                }
            )
        ], 50), 4578)

    }
}