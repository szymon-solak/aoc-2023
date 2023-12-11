use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
type GalaxyMap = Vec<Vec<char>>;

fn get_manhattan_distance(a: Point, b: Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn count_empty_rows(rows: &[Vec<char>]) -> usize {
    rows.iter()
        .filter(|line| line.iter().all(|c| c == &'.'))
        .count()
}

fn get_empty_rows_between_points(galaxy_map: &GalaxyMap, a: Point, b: Point) -> Point {
    let empty = vec![];
    let ys = galaxy_map
        .get((std::cmp::min(a.y, b.y) + 1) as usize..std::cmp::max(a.y, b.y) as usize)
        .unwrap_or(&empty);

    let transposed: Vec<Vec<_>> = (0..galaxy_map.len())
        .map(|col| {
            (0..galaxy_map[col].len())
                .map(|row| galaxy_map[row][col])
                .collect()
        })
        .collect();

    let xs = transposed
        .get((std::cmp::min(a.x, b.x) + 1) as usize..std::cmp::max(a.x, b.x) as usize)
        .unwrap_or(&empty);

    Point {
        x: count_empty_rows(xs) as i64,
        y: count_empty_rows(ys) as i64,
    }
}

fn get_distance_with_expansion(
    galaxy_map: &GalaxyMap,
    a: Point,
    b: Point,
    expansion_rate: i64,
) -> i64 {
    let expansion = get_empty_rows_between_points(galaxy_map, a, b);

    get_manhattan_distance(a, b)
        + expansion.x * (expansion_rate - 1)
        + expansion.y * (expansion_rate - 1)
}

fn get_distance_sum(galaxy_map: &GalaxyMap, point_pairs: &Vec<Point>, expansion_rate: i64) -> i64 {
    point_pairs
        .iter()
        .combinations(2)
        .map(|pair| {
            get_distance_with_expansion(
                &galaxy_map,
                **pair.get(0).unwrap(),
                **pair.get(1).unwrap(),
                expansion_rate,
            )
        })
        .sum::<i64>()
}

fn main() {
    let galaxy_map: GalaxyMap = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let pairs: Vec<_> = galaxy_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, ch)| {
                if ch == &'#' {
                    Some(Point {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    println!("part 1: {}", get_distance_sum(&galaxy_map, &pairs, 1));
    println!(
        "part 2: {}",
        get_distance_sum(&galaxy_map, &pairs, 1_000_000)
    );
}
