use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_set() {
        let set = Set {
            red: 3,
            green: 4,
            blue: 5,
        };
        assert!(set.is_possible(12, 14, 15));
        assert!(!set.is_possible(1, 14, 15));
        assert!(!set.is_possible(12, 1, 15));
        assert!(!set.is_possible(12, 14, 1));
    }

    #[test]
    fn test_game() {
        let set1 = Set {
            red: 3,
            green: 4,
            blue: 5,
        };
        let set2 = Set {
            red: 5,
            green: 6,
            blue: 7,
        };
        let game = Game {
            index: 0,
            sets: vec![set1, set2],
        };
        assert!(game.is_possible(12, 14, 15));
        assert!(game.is_possible(5, 6, 7));
        assert!(!game.is_possible(1, 14, 15));
        assert!(!game.is_possible(12, 1, 15));
        assert!(!game.is_possible(12, 14, 1));
        assert!(!game.is_possible(1, 1, 1));
        assert!(!game.is_possible(5, 5, 5));
    }

    #[test]
    fn test_game_minimum_set() {
        let set1 = Set {
            red: 3,
            green: 4,
            blue: 5,
        };
        let set2 = Set {
            red: 5,
            green: 2,
            blue: 7,
        };
        let game = Game {
            index: 0,
            sets: vec![set1, set2],
        };
        let minimum_set = game.minimum_set();
        assert_eq!(minimum_set.red, 5);
        assert_eq!(minimum_set.green, 4);
        assert_eq!(minimum_set.blue, 7);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 2286);
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    /// Check if the set is possible
    ///
    /// # Arguments
    ///
    /// * `red` - total ammount of red cubes in the bag
    /// * `green` - total ammount of green cubes in the bag
    /// * `blue` - total ammount of blue cubes in the bag
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        if self.red > red || self.green > green || self.blue > blue {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Game {
    index: u32,
    sets: Vec<Set>,
}

impl Game {
    /// Check if the game is possible
    ///
    /// # Arguments
    ///
    /// * `red` - total ammount of red cubes in the bag
    /// * `green` - total ammount of green cubes in the bag
    /// * `blue` - total ammount of blue cubes in the bag
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for set in self.sets.iter() {
            if !set.is_possible(red, green, blue) {
                return false;
            };
        }
        true
    }

    /// Return the minimum set of cubes needed to play the game
    fn minimum_set(&self) -> Set {
        let reds: Vec<u32> = self.sets.iter().map(|x| x.red).collect();
        let blues: Vec<u32> = self.sets.iter().map(|x| x.blue).collect();
        let greens: Vec<u32> = self.sets.iter().map(|x| x.green).collect();
        Set {
            red: reds.iter().max().unwrap().to_owned(),
            green: greens.iter().max().unwrap().to_owned(),
            blue: blues.iter().max().unwrap().to_owned(),
        }
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn solve_part1(fname: &String) -> u32 {
    let red_total = 12;
    let blue_total = 13;
    let green_total = 14;

    let content = read_file(fname);
    let mut result = 0;
    for line in content.lines() {
        let game = parse_line(&line);
        if game.is_possible(red_total, blue_total, green_total) {
            result += game.index
        }
    }
    result
}

fn solve_part2(fname: &String) -> u32 {
    let content = read_file(fname);
    let mut result = 0;
    for line in content.lines() {
        let game = parse_line(&line);
        let minimum_set = game.minimum_set();
        let product = minimum_set.red * minimum_set.green * minimum_set.blue;
        result += product;
    }
    result
}

fn parse_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(":").collect();
    // Parse game index
    let index = parts
        .first()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .replace(":", "")
        .parse::<u32>();
    let index = match index {
        Ok(index) => index,
        Err(error) => panic!("Cannot convert to numeric value: {}", error),
    };
    // Parse sets
    let mut sets: Vec<Set> = Vec::new();
    for set_str in parts.last().unwrap().split(";") {
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        for tuple in set_str.split(",") {
            let values: Vec<&str> = tuple.trim().split(" ").collect();
            let n = match values.first().unwrap().parse() {
                Ok(n) => n,
                Err(error) => panic!("Cannot convert to numeric value: {}", error),
            };
            let color = values.last().unwrap();
            match *color {
                "red" => red = n,
                "blue" => blue = n,
                "green" => green = n,
                _ => panic!("Cannot understand color {}", color),
            }
        }
        let set = Set { red, blue, green };
        sets.push(set);
    }
    Game { index, sets }
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
