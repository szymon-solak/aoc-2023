fn get_move_counter(
    moves: &str,
    networks: &Vec<(String, String, String)>,
    from_point: &(String, String, String),
) -> u64 {
    let mut move_counter = 0;
    let mut move_list = moves.chars().cycle();
    let mut current_move = &from_point.clone();

    while !current_move.0.ends_with('Z') {
        move_counter += 1;
        let next_move = move_list.next().unwrap();

        if next_move == 'L' {
            current_move = networks.iter().find(|net| net.0 == current_move.1).unwrap();
        } else {
            current_move = networks.iter().find(|net| net.0 == current_move.2).unwrap();
        }
    }

    move_counter
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { return a }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn get_move_counter_for_points(
    moves: &str,
    networks: &Vec<(String, String, String)>,
    from_points: Vec<&(String, String, String)>,
) -> u64 {
    from_points
        .iter()
        .map(|point| get_move_counter(moves, networks, point))
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

fn main() {
    let (moves, network_input) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let networks: Vec<_> = network_input
        .lines()
        .map(|line| {
            let without_symbols = line.replace(" = (", " ").replace(",", "").replace(")", "");

            let mut net = without_symbols.split_ascii_whitespace();
            let current = net.next().unwrap().to_owned();
            let left = net.next().unwrap().to_owned();
            let right = net.next().unwrap().to_owned();
            (current, left, right)
        })
        .collect();

    println!(
        "part 1: {}",
        get_move_counter(
            moves,
            &networks,
            networks.iter().find(|net| net.0 == "AAA").unwrap()
        )
    );

    println!(
        "part 2: {}",
        get_move_counter_for_points(
            moves,
            &networks,
            networks.iter().filter(|net| net.0.ends_with('A')).collect()
        )
    );
}
