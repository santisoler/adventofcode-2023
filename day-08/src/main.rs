use num::integer::lcm;
use std::{collections::HashMap, fs};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input_1");
        let result = solve_part1(&fname);
        assert_eq!(result, 2);
        let fname = String::from("data/test_input_2");
        let result = solve_part1(&fname);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input_3");
        let result = solve_part2(&fname);
        assert_eq!(result, 6);
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn move_to(&self, movement: &Movement) -> String {
        match movement {
            Movement::Left => self.left.clone(),
            Movement::Right => self.right.clone(),
        }
    }
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(fname: &String) -> (HashMap<String, Node>, Vec<Movement>) {
    let content = read_file(fname);
    let mut lines = content.lines();
    // Read movements
    let movements = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Movement::Left,
            'R' => Movement::Right,
            _ => panic!("Invalid movement"),
        })
        .collect();
    // Read empty line
    lines.next().unwrap();
    // Read map
    let mut map = HashMap::new();
    loop {
        match lines.next() {
            Some(line) => {
                let (position, node) = parse_line(line);
                map.insert(position, node);
            }
            None => break,
        };
    }
    (map, movements)
}

fn parse_line(line: &str) -> (String, Node) {
    let mut parts = line.split("=");
    let position = String::from(parts.nth(0).unwrap().trim());
    let mut nodes = parts
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.replace("(", "").replace(")", "").trim().to_string());
    let left = nodes.next().unwrap();
    let right = nodes.next().unwrap();
    let node = Node { left, right };
    (position, node)
}

fn solve_part1(fname: &String) -> u64 {
    let (map, movements) = parse_file(&fname);
    let mut n_movements = 0;
    let mut position = String::from("AAA");
    let goal = String::from("ZZZ");
    let mut goal_reached = false;
    while !goal_reached {
        for movement in movements.iter() {
            let current_node = map.get(&position).unwrap();
            position = current_node.move_to(movement);
            n_movements += 1;
            // Check if goal was reached
            if position == goal {
                goal_reached = true;
                break;
            }
        }
    }
    n_movements
}

fn solve_part2(fname: &String) -> u64 {
    let (map, movements) = parse_file(&fname);
    // Get initial positions
    let initial_positions: Vec<String> = map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.clone())
        .collect();
    // Explore graph for each initial position
    let n_movements: Vec<u64> = initial_positions
        .iter()
        .map(|p| get_movements_until_goal(p, &map, &movements))
        .collect();
    let mut result = 1;
    for item in n_movements.iter() {
        result = lcm(result, *item);
    }
    result
}

fn get_movements_until_goal(
    position: &String,
    map: &HashMap<String, Node>,
    movements: &Vec<Movement>,
) -> u64 {
    let mut position = position.clone();
    let mut n_movements = 0;
    let mut goal_reached = false;
    while !goal_reached {
        for movement in movements.iter() {
            position = map.get(&position).unwrap().move_to(movement);
            n_movements += 1;
            // Check if goal was reached
            if position.ends_with("Z") {
                goal_reached = true;
                break;
            }
        }
    }
    n_movements
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
