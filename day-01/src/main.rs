use std::fs;

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input_2");
        let result = solve_part2(&fname);
        assert_eq!(result, 281);
    }
    #[test]
    fn test_part2_with_shared_chars() {
        // Test when numbers share chars
        let fname = String::from("data/test_input_3");
        let result = solve_part2(&fname);
        assert_eq!(result, 281 + 82);
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
    let content = read_file(&fname);
    let mut result = 0;

    for line in content.lines() {
        let values: Vec<char> = line
            .chars()
            .into_iter()
            .filter(|x| x.is_numeric())
            .collect();
        let first = values.first().unwrap();
        let last = values.last().unwrap();
        let value: u32 = format!("{}{}", first, last).parse().unwrap();
        result += value
    }
    result
}

fn solve_part2(fname: &String) -> u32 {
    let content = read_file(&fname);
    let mut result = 0;

    for line in content.lines() {
        let values = get_numbers_in_line(line);
        let first = values.first().unwrap();
        let last = values.last().unwrap();
        let value: u32 = format!("{}{}", first, last).parse().unwrap();
        result += value;
    }
    result
}

fn get_numbers_in_line(line: &str) -> Vec<char> {
    let numbers_as_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut numbers = Vec::new();

    for (i, character) in line.chars().enumerate() {
        // Check numeric character
        if character.is_numeric() {
            numbers.push(character);
        };
        // Check for numbers as words
        for (j, num_str) in numbers_as_str.iter().enumerate() {
            let len = num_str.len();
            if line.len() - i >= len && line[i..i + len].to_owned() == *num_str {
                numbers.push(char::from_digit(j as u32 + 1, 10).unwrap());
            };
        }
    }
    numbers
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
