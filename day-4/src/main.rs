use std::collections::HashMap;

fn get_duplicated_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|seg| seg.parse::<i32>().ok())
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|f| *f += 1).or_insert(1);
            map
        })
        .into_iter()
        .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
        .collect::<Vec<_>>()
}

fn main() {
    let duplicated_numbers = include_str!("./input.txt")
        .lines()
        .map(|line| get_duplicated_numbers(line));

    let score = duplicated_numbers
        .clone()
        .map(|winning_numbers| {
            if winning_numbers.len() > 0 {
                i32::pow(2, (winning_numbers.len() - 1) as u32)
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("part 1: {score}");

    let cards = duplicated_numbers
        .enumerate()
        .fold(HashMap::new(), |mut map, (card_no, numbers)| {
            let mult = map.entry(card_no + 1).or_insert(1).clone();

            (0..numbers.len()).for_each(|offset| {
                map.entry(card_no + offset + 2)
                    .and_modify(|f| *f += mult)
                    .or_insert(mult + 1);
            });

            map
        })
        .values()
        .sum::<i32>();

    println!("part 2: {cards}");
}
