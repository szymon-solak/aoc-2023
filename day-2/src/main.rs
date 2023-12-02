#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<(i32, i32, i32)>, // r, g, b
}

impl<'a> Game<'a> {
    pub fn from_str(input: &'a str) -> Game<'a> {
        let (game, game_rounds) = input.split_once(':').unwrap();
        let id = game.rsplit_once(' ').unwrap().1;

        let rounds = game_rounds
            .split(';')
            .map(|round| {
                let mut r = (0, 0, 0);
                round.split(',').for_each(|count_color| {
                    let (count, color) = count_color.trim().split_once(' ').unwrap();
                    let parsed_count = count.parse::<i32>().unwrap();

                    match color.trim() {
                        "red" => {
                            r.0 = parsed_count;
                        }
                        "green" => {
                            r.1 = parsed_count;
                        }
                        "blue" => {
                            r.2 = parsed_count;
                        }
                        _ => unreachable!(),
                    }
                });

                r
            })
            .collect::<Vec<(i32, i32, i32)>>();

        Game { id, rounds }
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_str(line))
        .collect::<Vec<Game>>()
}

fn get_id_sum_of_possible_games(games: Vec<Game>, cube_count: (i32, i32, i32)) -> i32 {
    games
        .iter()
        .filter_map(|game| {
            if game.rounds.iter().all(|round| {
                round.0 <= cube_count.0 && round.1 <= cube_count.1 && round.2 <= cube_count.2
            }) {
                return game.id.parse::<i32>().ok();
            };

            None
        })
        .sum::<i32>()
}

fn get_power_of_lowest_cube_set(games: Vec<Game>) -> i32 {
    games
        .iter()
        .map(|game| {
            let r = (
                game.rounds.iter().map(|r| r.0).max().unwrap(),
                game.rounds.iter().map(|r| r.1).max().unwrap(),
                game.rounds.iter().map(|r| r.2).max().unwrap(),
            );

            r.0 * r.1 * r.2
        })
        .sum()
}

fn main() {
    println!(
        "part 1: {}",
        get_id_sum_of_possible_games(parse_input(include_str!("./input.txt")), (12, 13, 14))
    );

    println!(
        "part 2: {}",
        get_power_of_lowest_cube_set(parse_input(include_str!("./input.txt")))
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_part_1_test_input() {
        assert_eq!(
            crate::get_id_sum_of_possible_games(
                crate::parse_input(include_str!("./test_input.txt")),
                (12, 13, 14)
            ),
            8
        )
    }
}
