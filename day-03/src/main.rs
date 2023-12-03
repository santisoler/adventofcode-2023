use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_get_single_part_number() {
        let mut array: Vec<Vec<u32>> =
            vec![vec![99, 99, 99, 99, 99, 99], vec![99, 1, 2, 3, 99, 99]];
        let part_number = get_single_part_number(&mut array, &1, &3);
        assert_eq!(part_number, 123);
    }
}

fn read_file(fname: &String) -> String {
    let content = match fs::read_to_string(fname) {
        Err(why) => panic!("could't open {}: {}", fname, why),
        Ok(file_conent) => file_conent,
    };
    content
}

fn input_to_array(content: &String) -> Vec<Vec<u32>> {
    let mut array: Vec<Vec<u32>> = Vec::new();
    for line in content.lines() {
        let row: Vec<u32> = line.chars().map(|x| char_to_number(&x)).collect();
        array.push(row);
    }
    array
}

fn char_to_number(x: &char) -> u32 {
    if x.is_numeric() {
        return x.to_digit(10).unwrap();
    };
    if *x == '.' {
        return 99;
    }
    return 10;
}

fn get_part_numbers(array: &mut Vec<Vec<u32>>) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            if array[i][j] == 10 {
                let mut part_numbers_for_symbol = get_part_numbers_for_symbol(array, &i, &j);
                part_numbers.append(&mut part_numbers_for_symbol);
            }
        }
    }
    part_numbers
}

fn get_part_numbers_for_symbol(array: &mut Vec<Vec<u32>>, i: &usize, j: &usize) -> Vec<u32> {
    let n_rows = array.len();
    let n_cols = array[0].len();
    let mut part_numbers: Vec<u32> = Vec::new();
    for di in -1..1 + 1 {
        for dj in -1..1 + 1 {
            // Ignore the center (the symbol itself)
            if di == 0 && dj == 0 {
                continue;
            }
            // Ignore elements outside the array
            if *i == 0 && di == -1 {
                continue;
            }
            if *j == 0 && dj == -1 {
                continue;
            }
            if *i + 1 == n_cols && di == 1 {
                continue;
            }
            if *j + 1 == n_rows && dj == 1 {
                continue;
            }
            // Get part number
            let row_index = (*i as i32 + di) as usize;
            let col_index = (*j as i32 + dj) as usize;
            if array[row_index][col_index] < 10 {
                let part_number = get_single_part_number(array, &row_index, &col_index);
                part_numbers.push(part_number);
            }
        }
    }
    part_numbers
}

fn get_single_part_number(array: &mut Vec<Vec<u32>>, row_index: &usize, col_index: &usize) -> u32 {
    let mut start: usize = *col_index;
    let mut end: usize = *col_index;
    let row = &mut array[*row_index];
    loop {
        if start == 0 {
            break;
        }
        if row[start - 1] < 10 {
            start -= 1
        } else {
            break;
        }
    }
    loop {
        if end == row.len() - 1 {
            break;
        }
        if row[end + 1] < 10 {
            end += 1
        } else {
            break;
        }
    }
    let mut part_number = String::from("");
    for index in start..end + 1 {
        part_number.push(char::from_digit(array[*row_index][index], 10).unwrap());
        array[*row_index][index] = 99; // override so we don't read this part number again
    }
    part_number.parse().unwrap()
}

fn solve_part1(fname: &String) -> u32 {
    let content = read_file(&fname);
    let mut array = input_to_array(&content);
    let part_numbers = get_part_numbers(&mut array);
    part_numbers.iter().sum()
}

fn main() {
    // let fname = String::from("data/input");
    let fname = String::from("data/test_input");
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
