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

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 30);
    }
}

#[derive(Debug)]
struct Scratchcard {
    numbers: Vec<u32>,
    winning: Vec<u32>,
}

impl Scratchcard {
    fn matching_numbers(&self) -> u32 {
        let mut matching_numbers = 0;
        for number in self.numbers.iter() {
            if self.winning.contains(&number) {
                matching_numbers += 1
            }
        }
        matching_numbers
    }

    fn points(&self) -> u32 {
        let matching_numbers = self.matching_numbers();
        if matching_numbers < 2 {
            return matching_numbers;
        }
        2_u32.pow(matching_numbers - 1)
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

fn solve_part2(fname: &String) -> u32 {
    let content = read_file(fname);
    let cards: Vec<Scratchcard> = content.lines().map(|x| parse_line(&x)).collect();
    let mut copies: Vec<u32> = vec![0; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matching_numbers = card.matching_numbers() as usize;
        for j in i + 1..i + matching_numbers + 1 {
            copies[j] += 1 + copies[i]
        }
    }
    // Count total number of cards (including original and copies)
    let n_copies: u32 = copies.iter().sum();
    let n_originals = cards.len() as u32;
    let total_scratchcards = n_originals + n_copies;
    total_scratchcards
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
