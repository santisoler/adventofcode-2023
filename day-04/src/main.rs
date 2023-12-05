use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 13);
    }
}

#[derive(Debug)]
struct Scratchcard {
    numbers: Vec<u32>,
    winning: Vec<u32>,
}

impl Scratchcard {
    fn points(&self) -> u32 {
        let mut hits = 0;
        for number in self.numbers.iter() {
            if self.winning.contains(&number) {
                hits += 1
            }
        }
        if hits < 2 {
            return hits;
        }
        2_u32.pow(hits - 1)
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_line(line: &str) -> Scratchcard {
    let parts: Vec<&str> = line.split(":").collect();
    let sets: Vec<&str> = parts.last().unwrap().trim().split("|").collect();
    let winning: Vec<u32> = sets
        .first()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let numbers: Vec<u32> = sets
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    Scratchcard { numbers, winning }
}

fn solve_part1(fname: &String) -> u32 {
    let content = read_file(fname);
    let cards: Vec<Scratchcard> = content.lines().map(|x| parse_line(&x)).collect();
    let points = cards.iter().map(|x| x.points()).sum();
    points
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
