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
}

#[derive(Debug)]
struct Galaxy {
    x: u32,
    y: u32,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        (dx + dy) as u32
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
                    x: x as u32,
                    y: y as u32,
                })
            }
        }
    }
    galaxies
}

fn expand_galaxies(galaxies: &mut Vec<Galaxy>) {
    let mut xs: Vec<u32> = galaxies.iter().map(|g| g.x).collect();
    xs.sort();
    let mut ys: Vec<u32> = galaxies.iter().map(|g| g.y).collect();
    ys.sort();
    let missing_x: Vec<u32> = (xs[0]..*xs.iter().last().unwrap())
        .filter(|&x| !xs.contains(&(x as u32)))
        .map(|x| x as u32)
        .collect();
    let missing_y: Vec<u32> = (ys[0]..*ys.iter().last().unwrap())
        .filter(|&y| !ys.contains(&(y as u32)))
        .map(|y| y as u32)
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
        galaxy.x += n_xs;
        galaxy.y += n_ys;
    }
}

fn solve_part1(fname: &String) -> u32 {
    let mut galaxies = parse_file(fname);
    expand_galaxies(&mut galaxies);
    let mut distances = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            distances += galaxies[i].distance(&galaxies[j])
        }
    }
    distances
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
