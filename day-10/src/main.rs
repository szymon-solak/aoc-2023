fn get_starting_point(pipes: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..pipes.len() {
        for x in 0..pipes[y].len() {
            if pipes[y][x] == 'S' {
                return Some((y, x));
            }
        }
    }

    None
}

fn get_starting_point_pipe(pipes: &Vec<Vec<char>>, starting_point: (usize, usize)) -> char {
    let down = pipes
        .get(starting_point.0 + 1)
        .map(|p| p.get(starting_point.1).unwrap_or(&'.'))
        .unwrap_or(&'.')
        .to_string();
    let left = pipes
        .get(starting_point.0)
        .map(|p| p.get(starting_point.1 - 1).unwrap_or(&'.'))
        .unwrap_or(&'.')
        .to_string();
    let right = pipes
        .get(starting_point.0)
        .map(|p| p.get(starting_point.1 + 1).unwrap_or(&'.'))
        .unwrap_or(&'.')
        .to_string();

    match ("LF-".contains(&left), "7J-".contains(&right), "LJ|".contains(&down)) {
        (false, false, true) => '|',
        (true, true, false) => '-',
        (false, true, false) => 'L',
        (true, false, false) => 'J',
        (true, false, true) => '7',
        (false, true, true) => 'F',
        _ => '.',
    }
}

fn get_adjacent_coords(pipe: char) -> Vec<(i32, i32)> {
    match pipe {
        '|' => vec![(1, 0), (-1, 0)],
        '-' => vec![(0, 1), (0, -1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(0, -1), (1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        _ => unreachable!(),
    }
}

fn get_main_pipe(pipes: &Vec<Vec<char>>, starting_point: (usize, usize)) -> Vec<(i32, i32)> {
    let mut seen = vec![(starting_point.0 as i32, starting_point.1 as i32)];
    let mut last = (starting_point.0 as i32, starting_point.1 as i32);

    loop {
        let curr = seen.last().unwrap().to_owned();
        let adj = get_adjacent_coords(pipes[curr.0 as usize][curr.1 as usize])
            .iter()
            .map(|(dy, dx)| (curr.0 + dy, curr.1 + dx))
            .find(|(y, x)| !(y == &last.0 && x == &last.1));

        let adj = adj.unwrap();

        if adj.0 == starting_point.0 as i32 && adj.1 == starting_point.1 as i32 {
            break;
        }

        seen.push(adj.to_owned());
        last = curr;
    }

    seen
}

fn count_spaces_inside(
    pipes: &Vec<Vec<char>>,
    main_loop: &Vec<(i32, i32)>,
) -> u32 {
    let mut inside_count = 0;
    
    pipes.iter().enumerate().for_each(|(row_idx, row)| {
        let mut inside_pipe = false;

        row.iter().enumerate().for_each(|(col_idx, elem)| {
            let is_main_pipe = main_loop.iter().find(|(y, x)| y == &(row_idx as i32) && x == &(col_idx as i32)).is_some();

            if is_main_pipe && "LJ|".contains(&elem.to_string()) {
                inside_pipe = !inside_pipe;
            } 

            if !is_main_pipe && inside_pipe {
                inside_count += 1;
            }
        });
    });

    inside_count
}

fn main() {
    let mut pipes: Vec<_> = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let starting_point = get_starting_point(&pipes).expect("starting point not found");
    let starting_point_pipe = get_starting_point_pipe(&pipes, starting_point);
    pipes[starting_point.0][starting_point.1] = starting_point_pipe;
    let main_pipe = get_main_pipe(&pipes, starting_point);

    println!(
        "part 1: {:#?}",
        main_pipe.len() / 2
    );
    println!(
        "part 2: {:#?}",
        count_spaces_inside(&pipes, &main_pipe)
    )
}
