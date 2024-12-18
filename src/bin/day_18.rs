use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2024::fetch_input;

fn bfs(
    start: (isize, isize),
    end: (isize, isize),
    bounds: &HashMap<(usize, usize), bool>,
) -> Option<isize> {
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
                && np.0 <= 70
                && 0 <= np.1
                && np.1 <= 70
                && !seen.contains(&np)
                && !bounds.contains_key(&(np.0 as usize, np.1 as usize))
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

    let mut bytes_map = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let chars = line
            .split(',')
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if index >= limit {
            break;
        }

        bytes_map.insert((chars[0], chars[1]), true);
    }

    let mut matrix = vec![vec!['.'; size.0]; size.1];

    for y in 0..size.1 + 1 {
        for x in 0..size.0 + 1 {
            if bytes_map.contains_key(&(x, y)) {
                matrix[y][x] = '#';
            }
        }
    }

    match bfs((0, 0), (70, 70), &bytes_map) {
        Some(result) => println!("{:?}", result),
        None => panic!("no path found"),
    }

    Ok(())
}
