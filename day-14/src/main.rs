use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 136);
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(fname: &String) -> Vec<String> {
    let content = read_file(fname);
    content.lines().map(|l| String::from(l)).collect()
}

fn solve_part1(fname: &String) -> u32 {
    let map = parse_file(fname);
    let n_rows = map.len() as u32;
    let mut load: u32 = 0;
    for j in 0..map[0].len() {
        let column: String = map.iter().map(|row| row.chars().nth(j).unwrap()).collect();
        let mut position = 0;
        for portion in column.split("#") {
            let n_rounded = portion.chars().filter(|c| *c == 'O').count() as u32;
            if n_rounded > 0 {
                load += n_rounded * (n_rows - position as u32) - (n_rounded - 1) * n_rounded / 2;
            }
            position += portion.len() + 1;
        }
    }
    load
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
