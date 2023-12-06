use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

fn is_symbol(c: char) -> bool {
    "!@#$%^&*()-=_+/".contains(c)
}

fn get_neighbouring_coords(
    y: &usize,
    x: &usize,
    part_number: &i32,
    max_y: usize,
    max_x: usize,
) -> Vec<(usize, usize)> {
    (std::cmp::max(x, &(1 as usize)) - 1
        ..std::cmp::min(part_number.to_string().len() + x + 1, max_x))
        .flat_map(|x| {
            [
                (std::cmp::max(y, &(1 as usize)) - 1, x),
                (y.to_owned(), x),
                (std::cmp::min(y + 1, max_y - 1), x),
            ]
        })
        .collect_vec()
}

fn main() {
    let input = include_str!("./input.txt");
    let char_list: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let re = Regex::new(r"\d+").unwrap();
    let parts: Vec<(usize, usize, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(c_idx, line)| {
            re.captures_iter(line)
                .flat_map(|captures| {
                    captures
                        .iter()
                        .filter_map(|cap| {
                            if let Some(capture) = cap {
                                Some((c_idx, capture.start(), capture.as_str()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter_map(|(y, x, d)| {
            if let Some(number) = d.parse::<i32>().ok() {
                return Some((y, x, number));
            }
            None
        })
        .collect();

    let valid_parts_sum: i32 = parts
        .iter()
        .filter_map(|(y, x, part_number)| {
            if get_neighbouring_coords(y, x, part_number, char_list.len(), char_list[0].len())
                .iter()
                .any(|(y, x)| is_symbol(char_list[y.to_owned()][x.to_owned()]))
            {
                return Some(part_number);
            }

            Some(&0)
        })
        .sum();

    println!("part 1: {valid_parts_sum}");

    let gear_ratio_sum: i32 = parts
        .iter()
        .combinations(2)
        .filter_map(|parts| {
            let (y1, x1, part_number_1) = parts.get(0).unwrap();
            let (y2, x2, part_number_2) = parts.get(1).unwrap();
            let n1 =
                get_neighbouring_coords(y1, x1, part_number_1, char_list.len(), char_list[0].len());
            let n2 = get_neighbouring_coords(
                &y2,
                &x2,
                &part_number_2,
                char_list.len(),
                char_list[0].len(),
            );

            let hash1: HashSet<_> = n1.into_iter().collect();
            let hash2: HashSet<_> = n2.into_iter().collect();

            if hash1
                .intersection(&hash2)
                .any(|(y, x)| char_list[y.to_owned()][x.to_owned()] == '*')
            {
                return Some(part_number_1 * part_number_2);
            }

            Some(0)
        })
        .sum();

    println!("part 2: {gear_ratio_sum}")
}
