use std::{collections::HashMap, fs, iter::zip};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_min_movements() {
        let phases = vec![2, 3];
        let freqs = vec![4, 8];
        assert_eq!(get_min_movements(&freqs, &phases), 6);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input_1");
        let result = solve_part1(&fname);
        assert_eq!(result, 2);
        let fname = String::from("data/test_input_2");
        let result = solve_part1(&fname);
        assert_eq!(result, 6);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input_3");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 6);
    // }
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

fn solve_part1(fname: &String) -> u32 {
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

fn solve_part2(fname: &String) -> u32 {
    let (map, movements) = parse_file(&fname);
    // Get initial positions
    let initial_positions: Vec<String> = map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.clone())
        .collect();
    // Explore graph for each initial position
    let mut phases = vec![];
    let mut freqs = vec![];
    for p in initial_positions.iter() {
        let (phase, period) = explore_graph_to_goal(p, &map, &movements);
        phases.push(phase);
        freqs.push(period);
    }
    println!("{:?} {:?}", freqs, phases);
    get_min_movements(&freqs, &phases)
}

// Explore graph for a single starting position until round loop is found
//
// # Returns
//
// * `n_movements` -  Number of movements to the first element that finishes with Z
fn explore_graph_to_goal(
    initial_position: &String,
    map: &HashMap<String, Node>,
    movements: &Vec<Movement>,
) -> (u32, u32) {
    // Copy initial position
    let mut position = initial_position.clone();

    // Define history vector
    let mut history_starts: Vec<String> = vec![position.clone()];

    let mut n_movements = 0;
    let mut phases = vec![];

    loop {
        for movement in movements.iter() {
            position = map.get(&position).unwrap().move_to(movement);
            n_movements += 1;
            if position.ends_with("Z") {
                phases.push(n_movements);
            }
        }
        if history_starts.contains(&position) {
            break;
        }
        history_starts.push(position.clone());
    }
    phases = reduce_common_factors(&phases);
    if phases.len() > 1 {
        panic!("Found more than a single number of movements to the goal");
    }
    (phases[0], n_movements)
}

fn reduce_common_factors(sorted_vector: &Vec<u32>) -> Vec<u32> {
    let mut factors = vec![];
    for (i, element) in sorted_vector.iter().enumerate() {
        let divisible = sorted_vector[..i].iter().any(|x| element % x == 0);
        if !divisible {
            factors.push(*element);
        }
    }
    factors
}

fn get_min_movements(freqs: &Vec<u32>, phases: &Vec<u32>) -> u32 {
    let mut n = 0;
    // Sort freqs and phases according to freqs
    let indices = argsort(&freqs);
    let freqs: Vec<u32> = indices.iter().map(|i| freqs[*i]).collect();
    let phases: Vec<u32> = indices.iter().map(|i| phases[*i]).collect();
    // Compute minimum numbers of movements to reach goal
    let n_sequences = freqs.len() as u32;
    let phases_sum: u32 = phases.iter().sum();
    let mut cycles: Vec<u32> = vec![0; n_sequences as usize];
    let mut i = 0;
    loop {
        let freqs_cycles_sum: u32 = zip(freqs.clone(), cycles.clone()).map(|(f, p)| f * p).sum();
        println!("{:?} {:?} {:?}", freqs_cycles_sum, phases_sum, n_sequences);
        println!("  {:?}", (freqs_cycles_sum + phases_sum) % n_sequences);
        if freqs_cycles_sum > 50 {
            break;
        }
        if (freqs_cycles_sum + phases_sum) % n_sequences == 0 {
            n = (freqs_cycles_sum + phases_sum) / n_sequences;
            break;
        };
        cycles[i] += 1;
        if i < cycles.len() - 1 {
            i += 1
        } else {
            i = 0
        };
    }
    n
}

fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let fname = String::from("data/test_input_3");
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
