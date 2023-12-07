use std::{cmp::Ordering, collections::HashMap};

fn get_frequency(input: &str) -> HashMap<char, i32> {
    input.chars().fold(HashMap::new(), |mut result, character| {
        *result.entry(character).or_insert(0) += 1;

        result
    })
}

fn card_to_strenth(card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn card_to_strenth_with_joker(card: &char) -> u32 {
    if card == &'J' {
        0
    } else {
        card_to_strenth(card)
    }
}

fn compare_high_cards(hand_a: &str, hand_b: &str, map_card: fn(&char) -> u32) -> Ordering {
    if let Some((l, r)) = hand_a.chars().zip(hand_b.chars()).find(|(l, r)| l != r) {
        if map_card(&l) > map_card(&r) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        Ordering::Equal
    }
}

fn get_hand_type(hand: &str) -> i32 {
    let frequency = get_frequency(hand);
    let max_occurances = frequency.values().max().unwrap();

    match frequency.len() {
        1 => 7,
        2 => match max_occurances {
            4 => 6,
            3 => 5,
            _ => unreachable!(),
        },
        3 => match max_occurances {
            3 => 4,
            2 => 3,
            _ => unreachable!(),
        },
        4 => 2,
        _ => 1,
    }
}

fn get_hand_type_with_joker(hand: &str) -> i32 {
    let frequency = get_frequency(hand);

    let mut most_common: Vec<_> = frequency.iter().collect();
    most_common.sort_by(|a, b| b.1.cmp(&a.1));

    let replace_with = {
        let first = most_common.get(0).unwrap();

        if first.0 == &'J' && hand != "JJJJJ" {
            most_common.get(1).unwrap().0
        } else {
            first.0
        }
    };

    get_hand_type(&hand.replace("J", &replace_with.to_string()))
}

fn get_bid_sum(hands: &Vec<(&str, &str)>) -> u64 {
    hands
        .iter()
        .filter_map(|(_, bid)| bid.parse::<u64>().ok())
        .enumerate()
        .map(|(rank, bid)| (rank + 1) as u64 * bid)
        .sum()
}

fn main() {
    let mut hands: Vec<_> = include_str!("./input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .collect();

    hands.sort_by(|a, b| {
        let hand_a = get_hand_type(a.0);
        let hand_b = get_hand_type(b.0);

        if hand_a == hand_b {
            return compare_high_cards(a.0, b.0, card_to_strenth);
        }

        hand_a.partial_cmp(&hand_b).unwrap()
    });

    println!("part 1: {:#?}", get_bid_sum(&hands));

    hands.sort_by(|a, b| {
        let hand_a = get_hand_type_with_joker(a.0);
        let hand_b = get_hand_type_with_joker(b.0);

        if hand_a == hand_b {
            return compare_high_cards(a.0, b.0, card_to_strenth_with_joker);
        }

        hand_a.partial_cmp(&hand_b).unwrap()
    });

    println!("part 2: {:#?}", get_bid_sum(&hands));
}
