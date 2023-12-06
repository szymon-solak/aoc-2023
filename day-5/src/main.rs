#[derive(Debug)]
struct Mapping {
    source: i64,
    dest: i64,
    range_length: i64,
}

fn get_locations_for_seeds(seeds: Vec<(i64, i64)>, maps: &Vec<Vec<Mapping>>) -> Vec<(i64, i64)> {
    maps.iter().fold(seeds, |current_numbers, map| {
        current_numbers
            .iter()
            .flat_map(|num| {
                let all_mappings = map
                    .iter()
                    .filter(|mapping| {
                        (num.0 >= mapping.source && num.0 <= mapping.source + mapping.range_length)
                            || (num.1 >= mapping.source
                                && num.1 <= (mapping.source + mapping.range_length))
                    })
                    .map(|mapping| {
                        let lower_bound_in_mapping = i64::clamp(
                            mapping.dest + num.0 - mapping.source,
                            mapping.dest,
                            mapping.dest + mapping.range_length,
                        );

                        let higher_bound_in_mapping = i64::clamp(
                            mapping.dest + num.1 - mapping.source,
                            mapping.dest,
                            mapping.dest + mapping.range_length,
                        );

                        (lower_bound_in_mapping, higher_bound_in_mapping)
                    })
                    .collect::<Vec<(i64, i64)>>();

                if all_mappings.len() > 0 {
                    return all_mappings;
                } else {
                    vec![num.to_owned()]
                }
            })
            .collect::<Vec<_>>()
    })
}

fn main() {
    let (seed_input, map_input) = include_str!("./input.txt").split_once('\n').unwrap();

    let seeds: Vec<_> = seed_input
        .split_whitespace()
        .filter_map(|f| f.parse::<i64>().ok())
        .map(|v| (v, v))
        .collect();

    let maps: Vec<_> = map_input
        .trim()
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|v| v.parse::<i64>().ok())
                        .collect::<Vec<i64>>()
                })
                .fold(vec![], |mut m, range| {
                    let dest = range.get(0).unwrap().to_owned();
                    let source = range.get(1).unwrap().to_owned();
                    let range_length = range.get(2).unwrap().to_owned();

                    m.push(Mapping {
                        source,
                        dest,
                        range_length,
                    });

                    m
                })
        })
        .collect();

    println!(
        "part 1: {:?}",
        get_locations_for_seeds(seeds, &maps).iter().min()
    );

    let seed_ranges: Vec<_> = seed_input
        .split_whitespace()
        .filter_map(|f| f.parse::<i64>().ok())
        .collect();

    let seed_ranges = seed_ranges
        .chunks(2)
        .map(|c| {
            let start = c.get(0).unwrap().to_owned();
            let range_length = c.get(1).unwrap().to_owned();

            (start, start + range_length)
        })
        .collect();

    println!(
        "part 2: {:?}",
        get_locations_for_seeds(seed_ranges, &maps).iter().min()
    )
}
