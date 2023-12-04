use std::fs;

pub fn read_example(day: i32) -> String {
    let path = format!("input/example/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn read_real(day: i32) -> String {
    let path = format!("input/real/day{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn find_between<'a>(
    source: &'a str,
    left: &'static str,
    right: &'static str,
) -> Option<&'a str> {
    let start_bytes = source.find(left)? + left.len();
    let end_bytes = source.find(right)?;

    Some(&source[start_bytes..end_bytes].trim())
}
