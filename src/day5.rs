use crate::util;

#[derive(Debug)]
struct Range {
    source_start: i64,
    dest_start: i64,
    length: i64,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

impl Range {
    fn transform_value(&self, value: i64) -> Option<i64> {
        if value > self.source_start + self.length || value < self.source_start {
            return None;
        }

        let offset = self.dest_start - self.source_start;
        Some(value + offset)
    }
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

    let min_location = seeds.into_iter().map(process_seed).min().unwrap();

    min_location
}

fn part2() -> i64 {
    0
}

fn parse_input() -> (Vec<i64>, Vec<Map>) {
    let content = util::read_real(5);
    let mut lines = content.split("\n\n");

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|s| s.parse::<i64>())
        .flat_map(|parse| parse)
        .collect::<Vec<i64>>();

    let maps = lines.map(|map| parse_map(&map)).collect::<Vec<Map>>();

    (seeds, maps)
}

fn parse_input_part2() -> (Vec<Range>, Vec<Map>) {
    let content = util::read_example(5);
    let mut lines = content.split("\n\n");

    let seeds = parse_ranges(lines.next().unwrap().trim_start_matches("seeds: "));
    let maps = lines.map(|map| parse_map(&map)).collect::<Vec<Map>>();

    (seeds, maps)
}

fn parse_map(map: &str) -> Map {
    let mut map_lines = map.split('\n');

    let name = map_lines.next().unwrap().trim_end_matches(':').to_string();
    let ranges = map_lines.flat_map(parse_ranges).collect::<Vec<Range>>();

    Map { name, ranges }
}

fn parse_ranges(number_line: &str) -> Vec<Range> {
    number_line
        .split(' ')
        .map(|num| num.parse::<i64>())
        .flat_map(|parse| parse)
        .array_chunks::<3>()
        .map(|arr| Range {
            dest_start: arr[0],
            source_start: arr[1],
            length: arr[2],
        })
        .collect::<Vec<Range>>()
}
