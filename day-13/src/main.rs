use std::cmp;
use std::fmt;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_palindrome() {
        let vec: Vec<u32> = vec![1, 2, 2, 1, 3, 4];
        assert_eq!(is_palindrome(&vec, 0, 5), false);
        assert_eq!(is_palindrome(&vec, 0, 3), true);
        let vec: Vec<u32> = vec![1, 2, 1, 1];
        assert_eq!(is_palindrome(&vec, 0, 3), false);
        // let vec: Vec<u32> = vec![1, 2, 2, 1];
        // assert_eq!(is_palindrome(&vec), true);
        // let vec: Vec<u32> = vec![1, 2, 3, 5, 5, 3, 2, 1];
        // assert_eq!(is_palindrome(&vec), true);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 405);
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    pattern: Vec<Vec<bool>>,
}

impl Pattern {
    fn transpose(&self) -> Self {
        assert!(!self.pattern.is_empty());
        let transposed = (0..self.pattern[0].len())
            .map(|i| {
                self.pattern
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<bool>>()
            })
            .collect();
        Self {
            pattern: transposed,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows: Vec<String> = vec![];
        for row in self.pattern.iter() {
            rows.push(
                row.iter()
                    .map(|v| match v {
                        true => "#",
                        false => ".",
                    })
                    .collect(),
            );
        }
        write!(f, "{}\n", rows.join("\n"))
    }
}

fn is_palindrome<T: Eq>(sequence: &Vec<T>, start: usize, end: usize) -> bool {
    if (end - start) % 2 == 0 {
        panic!("Found sequence with odd number of elements.");
    };
    if end >= sequence.len() {
        panic!("End larger than the length of the sequence");
    }
    if start >= end {
        panic!("Start cannot be larger or equal than end");
    }
    if end - start == 1 {
        return sequence[start] == sequence[end];
    };
    if sequence[start] != sequence[end] {
        return false;
    }
    return is_palindrome(sequence, start + 1, end - 1);
}

fn find_symmetry_plane(pattern: &Pattern, vertical: &bool) -> Option<usize> {
    let length = match *vertical {
        true => pattern.pattern[0].len(),
        false => pattern.pattern.len(),
    };
    let pattern = match *vertical {
        true => pattern.clone(),
        false => pattern.transpose(),
    };
    let mut planes_stack: Vec<usize> = (1..length).collect();
    for row in pattern.pattern.iter() {
        if planes_stack.is_empty() {
            break;
        }
        let mut i = 0;
        while i < planes_stack.len() {
            let plane = planes_stack.remove(i);
            let (start, end) = get_start_end(&plane, &length);
            if is_palindrome(row, start, end) {
                planes_stack.insert(i, plane);
                i += 1;
            }
        }
    }

    if planes_stack.len() > 1 {
        panic!("found multiple symmetry planes")
    }

    if planes_stack.is_empty() {
        return None;
    }
    Some(planes_stack[0])
}

fn get_start_end(plane: &usize, length: &usize) -> (usize, usize) {
    let delta = cmp::min(*plane, *length - *plane);
    let start = *plane - delta;
    let end = *plane + delta - 1;
    (start, end)
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(fname: &String) -> Vec<Pattern> {
    let content = read_file(fname);
    let mut patterns = vec![];

    let mut lines = content.lines();
    let mut eof = false;
    while !eof {
        let mut pattern = vec![];
        loop {
            let line = match lines.next() {
                Some(x) => {
                    if x == "" {
                        break;
                    };
                    x
                }
                None => {
                    eof = true;
                    break;
                }
            };
            let row = line
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid character"),
                })
                .collect();
            pattern.push(row);
        }
        patterns.push(Pattern { pattern });
    }
    patterns
}

fn solve_part1(fname: &String) -> u32 {
    let patterns = parse_file(&fname);

    let mut result = 0;
    for pattern in patterns.iter() {
        match find_symmetry_plane(pattern, &true) {
            Some(symmetry_plane) => result += symmetry_plane as u32,
            None => match find_symmetry_plane(pattern, &false) {
                Some(symmetry_plane) => result += 100 * symmetry_plane as u32,
                None => panic!("Couldn't find symmetry plane"),
            },
        }
    }
    result
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
