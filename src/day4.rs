use std::cmp::Reverse;
use std::fmt;
use std::fmt::Formatter;

use crate::util;

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

#[derive(Debug, Clone)]
struct Card {
    number: u32,
    winning_numbers: Vec<i32>,
    obtained_numbers: Vec<i32>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Card {}", self.number)
    }
}

fn part1() -> u32 {
    let cards = parse_input();
    let mut sum_points = 0;

    for card in cards {
        let win_count = card
            .obtained_numbers
            .iter()
            .filter(|n| card.winning_numbers.contains(n))
            .count();

        let exponent = u32::try_from(win_count).unwrap();
        let points = 2_u32.pow(exponent) / 2;

        println!("Card: {:?}, points: {:?}", card, points);

        sum_points += points;
    }

    sum_points
}

fn part2() -> u32 {
    let mut card_source: Vec<Card> = parse_input();
    let mut card_stack: Vec<Card> = parse_input();
    let mut card_count = 0;

    card_stack.sort_by_key(|card| Reverse(card.number));

    while let Some(new_card) = card_stack.pop() {
        println!("Processing {}", new_card);

        let win_count = win_count(&new_card);

        for i in 0..win_count {
            let card_idx = usize::try_from(new_card.number).unwrap();
            if let Some(extra_card) = card_source.get(card_idx + i) {
                println!("Adding extra card {}", extra_card);
                card_stack.push(extra_card.clone());
            }
        }

        card_stack.sort_by_key(|card| Reverse(card.number));

        card_count += 1;
    }

    card_stack.sort_by_key(|card| card.number);

    for card in card_stack {
        println!("{}", card)
    }

    card_count
}

fn parse_input() -> Vec<Card> {
    let input_data = util::read_real(4);

    let parse = input_data.split('\n').map(|line| {
        let card_no = util::find_between(line, "Card", ":")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let number_data = line[8..].split('|');
        let mut numbers = number_data.map(|list| {
            list.split(' ')
                .flat_map(|number| number.parse::<i32>())
                .collect::<Vec<i32>>()
        });

        Card {
            number: card_no,
            winning_numbers: numbers.next().unwrap(),
            obtained_numbers: numbers.next().unwrap(),
        }
    });

    parse.collect::<Vec<Card>>()
}

fn win_count(card: &Card) -> usize {
    card.obtained_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count()
}
