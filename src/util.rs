use std::fs::File;
use std::io::BufRead;
use std::{fs, io};

pub fn read_example(day: i32) -> String {
    let path = format!("input/example/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn read_real(day: i32) -> String {
    let path = format!("input/real/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn read_example_lines(day: i32) -> io::Lines<io::BufReader<File>> {
    let path = format!("input/example/day{}.txt", day);
    let file = File::open(path).unwrap();

    io::BufReader::new(file).lines()
}

pub fn find_between<'a, 'b>(source: &'a str, left: &'b str, right: &'b str) -> Option<&'a str> {
    let start_bytes = source.find(left)? + left.len();
    let end_bytes = source.find(right)?;

    Some(&source[start_bytes..end_bytes].trim())
}
