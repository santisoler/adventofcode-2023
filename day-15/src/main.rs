use std::{collections::HashMap, fs};

use indexmap::IndexMap;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 145);
    }
}

#[derive(Debug)]
struct Box {
    lenses: IndexMap<String, u32>,
}

impl Box {
    fn new() -> Self {
        let lenses = IndexMap::new();
        Box { lenses }
    }

    fn is_empty(&self) -> bool {
        self.lenses.is_empty()
    }

    fn insert(&mut self, lens: String, focal_lenght: u32) {
        self.lenses.insert(lens, focal_lenght);
    }

    fn remove(&mut self, lens: String) {
        self.lenses.shift_remove(&lens);
    }

    fn focusing_power(&self) -> Option<u32> {
        if self.lenses.is_empty() {
            return None;
        }
        let box_index = {
            let (label, _) = self.lenses.get_index(0).unwrap();
            hash_algorithm(label) + 1 // sum 1 to start counting boxes from 1
        };
        let mut result = 0;
        for (slot, (_, focal_length)) in self.lenses.iter().enumerate() {
            result += box_index * (slot + 1) as u32 * focal_length;
        }
        Some(result)
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn hash_algorithm(string: &str) -> u32 {
    let mut result = 0;
    for char in string.chars() {
        result += char as u32;
        result *= 17;
        result %= 256;
    }
    result
}

fn solve_part1(fname: &String) -> u32 {
    let content = read_file(fname);
    let sequence: Vec<&str> = content.lines().nth(0).unwrap().split(",").collect();
    let mut result = 0;
    for code in sequence.iter() {
        result += hash_algorithm(code);
    }
    result
}

fn parse_code(code: &str) -> (String, Option<u32>) {
    let label;
    let focal_length;
    if code.contains('=') {
        let parts: Vec<&str> = code.split('=').collect();
        label = String::from(*parts.iter().nth(0).unwrap());
        focal_length = Some(parts.iter().last().unwrap().parse().unwrap());
    } else if code.contains('-') {
        let parts: Vec<&str> = code.split('-').collect();
        label = String::from(*parts.iter().nth(0).unwrap());
        focal_length = None;
    } else {
        panic!("Invalid code: '{}'", code);
    }
    (String::from(label), focal_length)
}

fn solve_part2(fname: &String) -> u32 {
    let content = read_file(fname);
    let sequence: Vec<&str> = content.lines().nth(0).unwrap().split(",").collect();
    let mut boxes: HashMap<u32, Box> = HashMap::new();
    for code in sequence.iter() {
        let (label, focal_length) = parse_code(code);
        match focal_length {
            Some(f) => {
                let box_index = hash_algorithm(&label);
                let box_i = boxes.entry(box_index).or_insert(Box::new());
                box_i.insert(label, f);
            }
            None => {
                let box_index = hash_algorithm(&label);
                if let Some(b) = boxes.get_mut(&box_index) {
                    b.remove(label);
                    if b.is_empty() {
                        boxes.remove(&box_index);
                    }
                };
            }
        }
    }
    let mut focusing_power = 0;
    for box_i in boxes.values() {
        if let Some(x) = box_i.focusing_power() {
            focusing_power += x
        }
    }
    focusing_power
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
