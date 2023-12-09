fn get_diff(numbers: &Vec<i64>) -> Vec<i64> {
    numbers
        .windows(2)
        .map(|w| w.get(1).unwrap() - w.get(0).unwrap())
        .collect()
}

fn get_sequences(numbers: &mut Vec<Vec<i64>>) -> &Vec<Vec<i64>> {
    let last = numbers.last().unwrap();
    let last_diff = get_diff(last);

    if last_diff.iter().all(|v| v == &0) {
        return numbers;
    }

    numbers.push(last_diff);

    get_sequences(numbers)
}

fn extrapolate(numbers: &Vec<i64>) -> i64 {
    let mut c = vec![numbers.clone()];

    get_sequences(&mut c)
        .iter()
        .filter_map(|list| list.last())
        .sum()
}

fn main() {
    let values: Vec<_> = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    println!(
        "part 1 {:#?}",
        values.iter().map(|vals| extrapolate(vals)).sum::<i64>()
    );
    println!(
        "part 2 {:#?}",
        values
            .iter()
            .map(|vals| {
                let v = vals.iter().rev().map(|c| c.to_owned()).collect();
                extrapolate(&v)
            })
            .sum::<i64>()
    );
}
