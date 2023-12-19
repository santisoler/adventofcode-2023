use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_solution_any_factor() {
        let fname = String::from("data/test_input");
        let result = solution(&fname, &10);
        assert_eq!(result, 1030);
        let result = solution(&fname, &100);
        assert_eq!(result, 8410);
    }
}

#[derive(Debug)]
struct Galaxy {
    x: u64,
    y: u64,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u64 {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        (dx + dy) as u64
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn parse_file(fname: &String) -> Vec<Galaxy> {
    let content = read_file(fname);
    let mut galaxies = vec![];
    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Galaxy {
                    x: x as u64,
                    y: y as u64,
                })
            }
        }
    }
    galaxies
}

fn expand_galaxies(galaxies: &mut Vec<Galaxy>, factor: &u64) {
    let mut xs: Vec<u64> = galaxies.iter().map(|g| g.x).collect();
    xs.sort();
    let mut ys: Vec<u64> = galaxies.iter().map(|g| g.y).collect();
    ys.sort();
    let missing_x: Vec<u64> = (xs[0]..*xs.iter().last().unwrap())
        .filter(|&x| !xs.contains(&(x as u64)))
        .map(|x| x as u64)
        .collect();
    let missing_y: Vec<u64> = (ys[0]..*ys.iter().last().unwrap())
        .filter(|&y| !ys.contains(&(y as u64)))
        .map(|y| y as u64)
        .collect();
    for galaxy in galaxies.iter_mut() {
        let n_xs = {
            let mut n = 0;
            for x in missing_x.iter() {
                if x < &galaxy.x {
                    n += 1
                };
            }
            n
        };
        let n_ys = {
            let mut n = 0;
            for y in missing_y.iter() {
                if y < &galaxy.y {
                    n += 1
                };
            }
            n
        };
        galaxy.x += n_xs * (factor - 1);
        galaxy.y += n_ys * (factor - 1);
    }
}

fn solution(fname: &String, factor: &u64) -> u64 {
    let mut galaxies = parse_file(fname);
    expand_galaxies(&mut galaxies, &factor);
    let mut distances = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            distances += galaxies[i].distance(&galaxies[j])
        }
    }
    distances
}

fn solve_part1(fname: &String) -> u64 {
    solution(fname, &2)
}

fn solve_part2(fname: &String) -> u64 {
    solution(fname, &1_000_000)
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
