use crate::util;
use std::collections::HashMap;

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
        println!("Final result: {}", result)
    }
}

fn part1() -> u32 {
    let mut sum_possible_ids = 0;

    for game in util::read_real(2).split('\n') {
        let game_id_tokens: Vec<&str> = game.split(':').flat_map(|s| s.split(' ')).collect();

        let game_id = game_id_tokens[1].parse::<u32>().unwrap();
        println!("Game id {:?}", game_id);

        if game_possible(game) {
            println!("Possible!");
            sum_possible_ids += game_id;
        } else {
            println!("Impossible!");
        }
    }

    sum_possible_ids
}

fn part2() -> i32 {
    let mut sum_power = 0;

    for game in util::read_real(2).split('\n') {
        let result = get_min_cube_count(game);
        println!("{:?}", result);

        let power = result["red"] * result["green"] * result["blue"];
        println!("Power {}", power);

        sum_power += power;
    }

    sum_power
}

fn game_possible(game: &str) -> bool {
    let cube_quantities = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let draws = game.split(":").last().unwrap().split(';');

    for draw in draws {
        let cubes = draw.split(',').map(|draw| draw.trim());

        for cube in cubes {
            let mut cube_split = cube.split(' ');

            let quantity = cube_split.next().unwrap().parse::<i32>().unwrap();
            let color = cube_split.next().unwrap();

            let allowed_quantity = cube_quantities[color];

            // println!("Color {}, quantity {}, allowed {}", color, quantity, allowed_quantity);

            if quantity > allowed_quantity {
                return false;
            }
        }
    }

    true
}

fn get_min_cube_count(game: &str) -> HashMap<&str, i32> {
    let mut cube_quantities = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    let draws = game.split(":").last().unwrap().split(';');

    for draw in draws {
        let cubes = draw.split(',').map(|draw| draw.trim());

        for cube in cubes {
            let mut cube_split = cube.split(' ');

            let quantity = cube_split.next().unwrap().parse::<i32>().unwrap();
            let color = cube_split.next().unwrap();

            cube_quantities.entry(color).and_modify(|q| {
                if quantity > *q {
                    *q = quantity
                }
            });
        }
    }

    cube_quantities
}

