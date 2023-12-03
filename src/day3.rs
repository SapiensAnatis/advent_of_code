use std::ops::Range;

use regex::Regex;

use crate::util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = crate::day3::part1();
        println!("Final result: {}", result)
    }

    #[test]
    fn test_get_adjacent_squares() {
        assert_eq!(get_adjacent_squares(1, 1), &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)]);
        assert_eq!(get_adjacent_squares(0, 0), &[(0, 0), (0, 1), (1, 0), (1, 1)]);
        assert_eq!(get_adjacent_squares(1, 0), &[(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]);
        assert_eq!(get_adjacent_squares(0, 1), &[(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)])
    }
}

fn part1() -> i32 {
    let input = util::read_real(3);

    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let mut line_no: usize = 0;
    let num_regex = Regex::new(r"\d+").unwrap();

    for line in input.split('\n') {
        let ranges: Vec<_> = num_regex.find_iter(&line).map(|m| (m.range(), m.as_str())).collect();

        for range in ranges {
            if validate_number(line_no, range.0, &grid) {
                println!("{} was a part number", range.1);
                sum += range.1.parse::<i32>().unwrap();
            }
        }

        line_no += 1;
    }

    return sum;
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

fn get_adjacent_squares(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut row_start = row;
    let mut col_start = col;

    if row_start > 0 { row_start -= 1; }
    if col_start > 0 { col_start -= 1; }

    for curr_row in row_start..=row + 1 {
        for curr_col in col_start..=col + 1 {
            if curr_row >= 0 && curr_col >= 0 {
                result.push((curr_row, curr_col))
            }
        }
    }

    result
}