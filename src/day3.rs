use std::collections::HashMap;
use std::ops::Range;

use regex::Regex;

use crate::util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1();
        println!("Final result: {}", result)
    }

    #[test]
    fn test_part2() {
        let result = part2();
        println!("Final result: {}", result)
    }

    #[test]
    fn test_get_adjacent_squares() {
        assert_eq!(
            get_adjacent_squares(1, 1),
            &[
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
        assert_eq!(
            get_adjacent_squares(0, 0),
            &[(0, 0), (0, 1), (1, 0), (1, 1)]
        );
        assert_eq!(
            get_adjacent_squares(1, 0),
            &[(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]
        );
        assert_eq!(
            get_adjacent_squares(0, 1),
            &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        )
    }
}

fn part1() -> i32 {
    let input = util::read_real(3);

    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let num_regex = Regex::new(r"\d+").unwrap();

    for (line_no, line) in input.split('\n').enumerate() {
        let result: i32 = num_regex
            .find_iter(&line)
            .filter_map(|m| match validate_number(line_no, m.range(), &grid) {
                true => Some(m.as_str()),
                false => None,
            })
            .map(|m| m.parse::<i32>().unwrap())
            .sum();

        sum += result;
    }

    return sum;
}

fn part2() -> i32 {
    let input = util::read_real(3);

    let mut sum = 0;

    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let num_regex = Regex::new(r"\d+").unwrap();

    let it = input
        .split('\n')
        .enumerate()
        .flat_map(|(line_no, line)|
            num_regex
                .find_iter(line)
                .filter_map(|m| match get_gear(line_no, m.range(), &grid) {
                    Some(gear_coord) => Some(GearResult {
                        number: m.as_str().parse::<i32>().unwrap(),
                        gear_coord,
                    }),
                    None => None,
                }).collect::<Vec<GearResult>>());

    let mut coord_hashmap = HashMap::new();

    for result in it {
        let entry = coord_hashmap.entry(result.gear_coord).or_insert(Vec::new());
        entry.push(result.number);
    }

    for (_, gears) in coord_hashmap {
        if gears.len() > 1 {
            sum += gears[0] * gears[1];
        }
    }

    sum
}

fn validate_number(row: usize, range: Range<usize>, grid: &Vec<Vec<char>>) -> bool {
    println!("Checking row {}, range {:?}", row, range);
    let col_len = grid[0].len();

    for i in range.start..range.end {
        let squares = get_adjacent_squares(row, i);
        for (j_row, j_col) in squares {
            println!("Checking adjacent {}, {}", j_row, j_col);

            if j_row >= grid.len() || j_col >= col_len {
                continue;
            }

            let char = grid[j_row][j_col];
            println!("Found char {}", char);

            if char != '.' && !char.is_digit(10) {
                return true;
            }
        }
    }

    false
}

fn get_gear(row: usize, range: Range<usize>, grid: &Vec<Vec<char>>) -> Option<Coordinate> {
    let col_len = grid[0].len();

    for i in range.start..range.end {
        let squares = get_adjacent_squares(row, i);
        for (j_row, j_col) in squares {
            // println!("Checking adjacent {}, {}", j_row, j_col);

            if j_row >= grid.len() || j_col >= col_len {
                continue;
            }

            let char = grid[j_row][j_col];

            if char == '*' {
                return Some(Coordinate {
                    row: j_row,
                    col: j_col,
                });
            }
        }
    }

    None
}

fn get_adjacent_squares(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut row_start = row;
    let mut col_start = col;

    if row_start > 0 {
        row_start -= 1;
    }
    if col_start > 0 {
        col_start -= 1;
    }

    for curr_row in row_start..=row + 1 {
        for curr_col in col_start..=col + 1 {
            result.push((curr_row, curr_col))
        }
    }

    result
}

#[derive(PartialEq, Debug, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}


#[derive(Debug)]
struct GearResult {
    number: i32,
    gear_coord: Coordinate,
}