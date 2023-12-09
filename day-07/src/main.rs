mod part1;
pub mod utils;

use crate::part1::solve_part1;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 6440);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 30);
    // }
}

fn main() {
    let fname = String::from("data/input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
