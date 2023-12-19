use std::fs;
use std::iter::zip;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input_1");
        let result = solve_part1(&fname);
        assert_eq!(result, 4);
        let fname = String::from("data/test_input_2");
        let result = solve_part1(&fname);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_map_walk() {
        let fname = String::from("data/test_input_1");
        let map = parse_file(&fname);
        let new_position = map.walk(&Position { x: 3, y: 1 }, &Position { x: 2, y: 1 });
        assert_eq!(new_position, Position { x: 3, y: 2 });
    }

    #[test]
    fn test_map_starting_positions() {
        let fname = String::from("data/test_input_1");
        let map = parse_file(&fname);
        let starting_positions = map.get_starting_directions();
        let expected = vec![Position { x: 1, y: 2 }, Position { x: 2, y: 1 }];
        assert_eq!(expected, starting_positions);
    }
}

struct Map {
    map: Vec<Vec<Pipe>>,
    start_position: Position,
}

impl Map {
    fn get(&self, position: &Position) -> &Pipe {
        &self.map[position.y][position.x]
    }

    fn get_starting_directions(&self) -> Vec<Position> {
        let mut positions = vec![];
        // Move north
        if self.start_position.y > 0 {
            let x = self.start_position.x;
            let y = self.start_position.y - 1;
            let neighbour = Position { x, y };
            match self.get(&neighbour) {
                Pipe::Vertical | Pipe::BendSE | Pipe::BendSW => positions.push(neighbour),
                _ => (),
            }
        }
        // Move south
        if self.start_position.y < self.map.len() - 1 {
            let x = self.start_position.x;
            let y = self.start_position.y + 1;
            let neighbour = Position { x, y };
            match self.get(&neighbour) {
                Pipe::Vertical | Pipe::BendNE | Pipe::BendNW => positions.push(neighbour),
                _ => (),
            }
        }
        // Move east
        if self.start_position.x < self.map[0].len() - 1 {
            let x = self.start_position.x + 1;
            let y = self.start_position.y;
            let neighbour = Position { x, y };
            match self.get(&neighbour) {
                Pipe::Horizontal | Pipe::BendSW | Pipe::BendNW => positions.push(neighbour),
                _ => (),
            }
        }
        // Move west
        if self.start_position.x > 0 {
            let x = self.start_position.x - 1;
            let y = self.start_position.y;
            let neighbour = Position { x, y };
            match self.get(&neighbour) {
                Pipe::Horizontal | Pipe::BendSE | Pipe::BendNE => positions.push(neighbour),
                _ => (),
            }
        }
        positions
    }

    fn walk(&self, position: &Position, avoid: &Position) -> Position {
        let (dx, dy) = match self.get(position) {
            Pipe::Vertical => (vec![0, 0], vec![1, -1]),
            Pipe::Horizontal => (vec![-1, 1], vec![0, 0]),
            Pipe::BendNE => (vec![0, 1], vec![-1, 0]),
            Pipe::BendNW => (vec![0, -1], vec![-1, 0]),
            Pipe::BendSW => (vec![0, -1], vec![1, 0]),
            Pipe::BendSE => (vec![0, 1], vec![1, 0]),
            Pipe::Start => panic!("Found starting position"),
            Pipe::Ground => panic!("Standing on ground, invalid pipe"),
        };
        let mut positions: Vec<Position> = vec![];
        for (dxi, dyi) in zip(&dx, &dy) {
            if position.x == 0 && *dxi < 0 {
                continue;
            }
            if position.y == 0 && *dyi < 0 {
                continue;
            }
            if position.x == self.map[0].len() - 1 && *dxi > 0 {
                continue;
            }
            if position.y == self.map.len() - 1 && *dyi > 0 {
                continue;
            }
            positions.push(Position {
                x: (position.x as i32 + *dxi) as usize,
                y: (position.y as i32 + *dyi) as usize,
            });
        }
        positions = positions.into_iter().filter(|p| p != avoid).collect();
        if positions.len() > 1 {
            panic!("Found multiple valid places to move")
        }
        positions[0]
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(fname: &String) -> Map {
    let content = read_file(fname);
    let mut start_position = Position { x: 0, y: 0 };
    let mut map: Vec<Vec<Pipe>> = vec![];
    for (i, line) in content.lines().enumerate() {
        let mut row: Vec<Pipe> = vec![];
        for (j, char) in line.chars().enumerate() {
            let value = match char {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::BendNE,
                'J' => Pipe::BendNW,
                '7' => Pipe::BendSW,
                'F' => Pipe::BendSE,
                '.' => Pipe::Ground,
                'S' => {
                    start_position.x = j;
                    start_position.y = i;
                    Pipe::Start
                }
                _ => panic!("invalid character"),
            };
            row.push(value);
        }
        map.push(row);
    }
    Map {
        map,
        start_position,
    }
}

fn solve_part1(fname: &String) -> u32 {
    let map = parse_file(fname);
    let starting_directions = map.get_starting_directions();
    if starting_directions.len() > 2 {
        panic!("Found more than two valid directions to start walking")
    }
    let mut position_1 = starting_directions[0];
    let mut position_2 = starting_directions[1];
    let mut prev_position_1 = map.start_position.clone();
    let mut prev_position_2 = map.start_position.clone();
    let mut n_steps = 1;
    loop {
        // Update position 1
        let new_position_1 = map.walk(&position_1, &prev_position_1);
        prev_position_1 = position_1.clone();
        position_1 = new_position_1.clone();
        n_steps += 1;
        if position_1 == position_2 {
            break;
        }
        // Update position 2
        let new_position_2 = map.walk(&position_2, &prev_position_2);
        prev_position_2 = position_2.clone();
        position_2 = new_position_2.clone();
        if position_1 == position_2 {
            break;
        }
    }
    n_steps
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
