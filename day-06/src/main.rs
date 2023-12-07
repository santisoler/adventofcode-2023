use std::time::Instant;
use std::{fs, iter::zip};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1_brute_force(&fname);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2_brute_force(&fname);
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_part2_binary_search() {
        let fname = String::from("data/test_input");
        let result = solve_part2_binary_search(&fname);
        assert_eq!(result, 71503);
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file_part1(content: &String) -> (Vec<u64>, Vec<u64>) {
    let mut lines = content.lines();
    let times = lines
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| x.parse::<u64>().is_ok())
        .map(|x| x.parse().unwrap())
        .collect();
    let distances = lines
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| x.parse::<u64>().is_ok())
        .map(|x| x.parse().unwrap())
        .collect();
    (times, distances)
}

fn parse_file_part2(content: &String) -> (u64, u64) {
    let mut lines = content.lines();
    let time = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance_record = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();
    (time, distance_record)
}

fn get_number_winning_solutions(time: &u64, distance_record: &u64) -> u64 {
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

fn get_distance(hold_time: &u64, total_time: &u64) -> u64 {
    *hold_time * (*total_time - *hold_time)
}

fn binary_search(time: &u64, distance_record: &u64) -> u64 {
    let min_time = time.div_ceil(2);
    let max_time = *time;
    let mut left = max_time;
    let mut right = min_time;
    let mut middle: u64;
    loop {
        if left < right {
            panic!("Unsuccessful")
        }
        middle = (left + right) / 2;
        let distance = get_distance(&middle, time);
        if distance < *distance_record {
            left = middle - 1;
        } else if distance > *distance_record {
            right = middle + 1;
        } else {
            break;
        }
        if left + 1 == right {
            middle = right;
            break;
        }
    }
    let mut result = 2 * (middle - min_time);
    if time % 2 == 0 {
        result -= 1
    };
    result
}

fn solve_part1_brute_force(fname: &String) -> u64 {
    let content = read_file(fname);
    let (times, distances) = parse_file_part1(&content);
    let mut result = 1;
    for (time, distance_record) in zip(times, distances) {
        result *= get_number_winning_solutions(&time, &distance_record);
    }
    result
}

fn solve_part1_binary_search(fname: &String) -> u64 {
    let content = read_file(fname);
    let (times, distances) = parse_file_part1(&content);
    let mut result = 1;
    for (time, distance_record) in zip(times, distances) {
        result *= binary_search(&time, &distance_record);
    }
    result
}

fn solve_part2_brute_force(fname: &String) -> u64 {
    let content = read_file(fname);
    let (time, distance_record) = parse_file_part2(&content);
    let result = get_number_winning_solutions(&time, &distance_record);
    result
}

fn solve_part2_binary_search(fname: &String) -> u64 {
    let content = read_file(fname);
    let (time, distance_record) = parse_file_part2(&content);
    let result = binary_search(&time, &distance_record);
    result
}

fn main() {
    let fname = String::from("data/input");

    let now = Instant::now();
    let result = solve_part1_brute_force(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 1 (brute force): {}", result);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let result = solve_part1_binary_search(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 1 (binary search): {}", result);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let result = solve_part2_brute_force(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 2 (brute force): {}", result);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let result = solve_part2_binary_search(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 2 (binary search): {}", result);
    println!("Elapsed: {:.2?}", elapsed);
}
