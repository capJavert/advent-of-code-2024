use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2024::fetch_input;

fn bfs(start: (isize, isize), end: (isize, isize), matrix: &Vec<Vec<char>>) -> Option<isize> {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::from([(start, 0)]);

    while let Some((cp, cd)) = to_visit.pop_front() {
        if seen.contains(&cp) {
            continue;
        }

        if cp == end {
            return Some(cd);
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let np = (cp.0 + dx, cp.1 + dy);
            if 0 <= np.0
                && np.0 < matrix[0].len() as isize
                && 0 <= np.1
                && np.1 < matrix.len() as isize
                && !seen.contains(&np)
                && matrix[np.1 as usize][np.0 as usize] != '#'
            {
                to_visit.push_back((np, cd + 1));
            }
        }

        seen.insert(cp);
    }

    None
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(20, 2024).expect("failed to fetch input");

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let get_position = |selector: char| -> (usize, usize) {
        let (x, y, _) = matrix
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (x, y, cell)))
            .find(|(_, _, cell)| *cell == selector)
            .unwrap();

        (x, y)
    };

    let start = get_position('S');
    let end = get_position('E');

    let mut cheats = HashMap::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];
            if x == 0
                || y == 0
                || x == matrix[y].len() - 1
                || y == matrix.len() - 1
                || cheats.contains_key(&(x, y))
            {
                continue;
            }

            if item == '#' {
                let mut cheat = matrix.clone();
                cheat[y][x] = '.';

                cheats.insert((x, y), cheat);
            }
        }
    }

    let mut good_cheats = 0;

    let fair_time = bfs(
        (start.0 as isize, start.1 as isize),
        (end.0 as isize, end.1 as isize),
        &matrix,
    )
    .unwrap();

    for cheat in cheats.values() {
        if let Some(result) = bfs(
            (start.0 as isize, start.1 as isize),
            (end.0 as isize, end.1 as isize),
            cheat,
        ) {
            if fair_time - result >= 100 {
                good_cheats += 1;
            }
        }
    }

    println!("{}", good_cheats);

    Ok(())
}
