use std::{
    collections::{HashSet, VecDeque},
    vec,
};

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
    let input = fetch_input(18, 2024).expect("failed to fetch input");

    let size = (71, 71);
    let limit = 1024;

    let mut bytes_map = vec![];

    for line in input.lines() {
        let chars = line
            .split(',')
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        bytes_map.push((chars[0], chars[1]));
    }

    let mut matrix = vec![vec!['.'; size.0]; size.1];

    let mut byte = 0;

    loop {
        let byte_location = bytes_map.get(byte).unwrap();

        matrix[byte_location.1][byte_location.0] = '#';

        if byte > limit {
            match bfs((0, 0), (70, 70), &matrix) {
                Some(_) => {}
                None => {
                    println!("{:?}", byte_location);

                    break;
                }
            }
        }

        byte += 1;
    }

    Ok(())
}
