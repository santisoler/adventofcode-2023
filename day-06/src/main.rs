use std::{fs, iter::zip};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 288);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 30);
    // }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(content: &String) -> (Vec<u32>, Vec<u32>) {
    let mut lines = content.lines();
    let times = lines
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| x.parse::<u32>().is_ok())
        .map(|x| x.parse().unwrap())
        .collect();
    let distances = lines
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| x.parse::<u32>().is_ok())
        .map(|x| x.parse().unwrap())
        .collect();
    (times, distances)
}

fn get_number_winning_solutions(time: &u32, distance_record: &u32) -> u32 {
    let mut n_winning_solutions = 0;
    let mut hold_time = time.div_ceil(2);
    let mut distance = hold_time * (*time - hold_time);
    while distance > *distance_record {
        n_winning_solutions += 1;
        hold_time += 1;
        distance = hold_time * (*time - hold_time);
    }
    n_winning_solutions *= 2;
    if *time % 2 == 0 {
        n_winning_solutions -= 1;
    }
    n_winning_solutions
}

fn solve_part1(fname: &String) -> u32 {
    let content = read_file(fname);
    let (times, distances) = parse_file(&content);
    let mut result = 1;
    for (time, distance_record) in zip(times, distances) {
        result *= get_number_winning_solutions(&time, &distance_record);
    }
    result
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
