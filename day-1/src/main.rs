use regex::Regex;

fn as_digit(input: &str) -> i32 {
    match input {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => {
            println!("unhandled case: {input}");
            unreachable!();
        },
    }
}

fn main() {
    let match_first_digit =
        Regex::new(r"^(?:.*?)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|0)")
            .unwrap();
    let match_last_digit =
        Regex::new(r"^(?:.*)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|0)")
            .unwrap();

    let calibration_sum = include_str!("./input.txt")
        .split("\n")
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            format!(
                "{}{}",
                as_digit(match_first_digit.captures(line).unwrap().get(1).unwrap().as_str()),
                as_digit(match_last_digit.captures(line).unwrap().get(1).unwrap().as_str()),
            )
            .parse::<i32>()
            .ok()
        })
        .sum::<i32>();

    println!("part 1: {calibration_sum}");
}
