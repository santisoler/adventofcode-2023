use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 1320);
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
    let content = read_file(fname);
    let sequence: Vec<&str> = content.lines().nth(0).unwrap().split(",").collect();
    let mut result = 0;
    for code in sequence.iter() {
        let mut current_value = 0;
        for char in code.chars() {
            current_value += char as u32;
            current_value *= 17;
            current_value %= 256;
        }
        result += current_value;
    }
    result
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
