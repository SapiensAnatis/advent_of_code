use crate::util;

#[derive(Debug)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        value >= self.start && value <= self.start + self.length
    }
}

#[derive(Debug)]
struct RangePair {
    source: Range,
    dest: Range,
}

impl RangePair {
    fn transform_value(&self, value: i64) -> Option<i64> {
        if !self.source.contains(value) {
            return None;
        }

        let offset = self.dest.start - self.source.start;
        Some(value + offset)
    }

    fn transform_value_rev(&self, value: i64) -> Option<i64> {
        if !self.dest.contains(value) {
            return None;
        }

        let offset = -(self.dest.start - self.source.start);
        Some(value + offset)
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<RangePair>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1();
        println!("Final result: {}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2();
        println!("Final result: {}", result);
    }
}

fn part1() -> i64 {
    let (seeds, maps) = parse_input();

    println!("Seeds: {:?}", seeds);

    let process_seed = |seed| {
        println!("Processing seed {}", seed);
        let mut processed_value = seed;

        for map in &maps {
            println!("Map: {}", map.name);
            for range in &map.ranges {
                if let Some(new_value) = range.transform_value(processed_value) {
                    processed_value = new_value;
                    break;
                }
            }
            println!("Mapped value: {}", processed_value);
        }

        println!("\n");

        processed_value
    };

    let min_location = [60].into_iter().map(process_seed).min().unwrap();

    min_location
}

fn part2() -> i64 {
    let (seeds, maps) = parse_input_part2();

    println!("Seeds: {:?}, maps {:?}", seeds, maps);

    for test_end_value in (0..=60_568_881).step_by(1) {
        if test_end_value % 1000 == 0 {
            println!("Tested {} values", test_end_value)
        }

        let mut processed_value = test_end_value;

        for map in (&maps).iter().rev() {
            for range in &map.ranges {
                if let Some(new_value) = range.transform_value_rev(processed_value) {
                    processed_value = new_value;
                    break;
                }
            }
        }

        if let Some(matching_seed) = seeds.iter().find(|seed| seed.contains(processed_value)) {
            println!("Found matching seed {:?}", matching_seed);
            return test_end_value;
        }
    }

    panic!("Failed to find answer")
}

fn parse_input() -> (Vec<i64>, Vec<Map>) {
    let content = util::read_example(5);
    let mut lines = content.split("\n\n");

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|s| s.parse::<i64>())
        .flat_map(|parse| parse)
        .collect::<Vec<i64>>();

    let maps = lines.map(parse_map).collect::<Vec<Map>>();

    (seeds, maps)
}

fn parse_input_part2() -> (Vec<Range>, Vec<Map>) {
    let content = util::read_real(5);
    let mut lines = content.split("\n\n");

    let seeds = parse_ranges(lines.next().unwrap().trim_start_matches("seeds: "));
    let maps = lines.map(parse_map).collect::<Vec<Map>>();

    (seeds, maps)
}

fn parse_map(map: &str) -> Map {
    let mut map_lines = map.split('\n');

    let name = map_lines.next().unwrap().trim_end_matches(':').to_string();
    let ranges = map_lines
        .flat_map(parse_range_pairs)
        .collect::<Vec<RangePair>>();

    Map { name, ranges }
}

fn parse_range_pairs(number_line: &str) -> Vec<RangePair> {
    number_line
        .split(' ')
        .map(|num| num.parse::<i64>())
        .flat_map(|parse| parse)
        .array_chunks::<3>()
        .map(|arr| RangePair {
            dest: Range {
                start: arr[0],
                length: arr[2],
            },
            source: Range {
                start: arr[1],
                length: arr[2],
            },
        })
        .collect::<Vec<RangePair>>()
}

fn parse_ranges(number_line: &str) -> Vec<Range> {
    number_line
        .split(' ')
        .map(|num| num.parse::<i64>())
        .flat_map(|parse| parse)
        .array_chunks::<2>()
        .map(|arr| Range {
            start: arr[0],
            length: arr[1],
        })
        .collect::<Vec<Range>>()
}
