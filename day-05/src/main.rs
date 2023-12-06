use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_map() {
        let mut map = Map::new();
        map.insert(50, 98, 2);
        map.insert(52, 50, 48);
        assert_eq!(map.get(&0), 0);
        assert_eq!(map.get(&1), 1);
        assert_eq!(map.get(&48), 48);
        assert_eq!(map.get(&49), 49);
        assert_eq!(map.get(&50), 52);
        assert_eq!(map.get(&51), 53);
        assert_eq!(map.get(&96), 98);
        assert_eq!(map.get(&97), 99);
        assert_eq!(map.get(&98), 50);
        assert_eq!(map.get(&99), 51);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 35);
    }
}

#[derive(Debug)]
struct Map {
    ranges: HashMap<RangeInclusive<u32>, u32>,
}

impl Map {
    fn new() -> Self {
        Self {
            ranges: HashMap::new(),
        }
    }

    fn insert(&mut self, dest: u32, source: u32, len: u32) {
        let range = RangeInclusive::new(source, source + len);
        self.ranges.insert(range, dest);
    }

    fn get(&self, value: &u32) -> u32 {
        for (range, dest) in self.ranges.iter() {
            if range.contains(&value) {
                return *value - range.start() + dest;
            }
        }
        *value
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_input(content: &String) -> (Vec<u32>, Vec<Map>) {
    // Define maps
    let mut maps: Vec<Map> = Vec::new();
    // Define iterator over lines
    let mut lines = content.lines();
    // Read seeds
    let seeds = lines
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| x.parse::<u32>().is_ok())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // Read empty line
    lines.next().unwrap();
    // Start reading maps
    loop {
        let mut eof = false;
        // Read line containing map title
        lines.next().unwrap();
        // Get next line
        let mut line = lines.next().unwrap();
        // Define map
        let mut map = Map::new();
        // Read mappings
        while !line.is_empty() {
            let map_values: Vec<u32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
            map.insert(map_values[0], map_values[1], map_values[2]);
            let next_line = lines.next();
            if next_line.is_none() {
                eof = true;
                break;
            }
            line = next_line.unwrap();
        }
        maps.push(map);
        if eof {
            break;
        };
    }
    (seeds, maps)
}

fn get_location(seed: &u32, maps: &Vec<Map>) -> u32 {
    let mut value = *seed;
    for map in maps.iter() {
        value = map.get(&value);
    }
    value
}

fn solve_part1(fname: &String) -> u32 {
    let content = read_file(&fname);
    let (seeds, maps) = parse_input(&content);
    let min_location = seeds.iter().map(|s| get_location(&s, &maps)).min().unwrap();
    min_location
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
