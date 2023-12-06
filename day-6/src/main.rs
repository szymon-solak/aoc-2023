use num::BigUint;

fn get_distance(hold_for: &BigUint, duration: &BigUint) -> BigUint {
    let remaining_time = duration - hold_for;

    hold_for * remaining_time
}

fn get_product_of_ways_to_win(races: Vec<(BigUint, BigUint)>) -> BigUint {
    races
        .iter()
        .map(|(duration, record)| {
            let mut f: Option<BigUint> = None;
            let mut t: Option<BigUint> = None;
            for hold_for in num_iter::range(BigUint::from(1 as u64), duration.to_owned()) {
                if f.is_none() && &get_distance(&hold_for, duration) > record {
                    f = Some(hold_for.clone())
                }

                let from_end = duration - hold_for.clone();

                if t.is_none() && &get_distance(&from_end, duration) > record {
                    t = Some(from_end)
                }

                if f.is_some() && t.is_some() {
                    break;
                }
            }

            t.unwrap() - f.unwrap() + BigUint::from(1 as u64)
        })
        .product()
}

fn main() {
    let (times_input, distances_input) = include_str!("./input.txt").split_once('\n').unwrap();
    let times = times_input
        .split_whitespace()
        .filter_map(|t| t.parse::<BigUint>().ok());
    let distances = distances_input
        .split_whitespace()
        .filter_map(|t| t.parse::<BigUint>().ok());

    let races: Vec<_> = times.zip(distances).collect();
    let ways_to_win = get_product_of_ways_to_win(races);

    println!("part 1: {ways_to_win:?}");

    let time = times_input
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap();
    let distance = distances_input
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap();

    println!(
        "part 2: {}",
        get_product_of_ways_to_win(vec![(time, distance)])
    )
}
