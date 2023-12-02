use std::fs;

pub fn read_example(day: i32) -> String {
    let path = format!("input/example/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn read_real(day: i32) -> String {
    let path = format!("input/real/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}