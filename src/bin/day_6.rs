use std::collections::{HashMap, HashSet};

use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(6, 2024).expect("failed to fetch input");

    let mut visited = HashSet::new();

    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut current_position: (i32, i32) = (0, 0);

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];

            if item == '^' {
                matrix[y][x] = '.';
                current_position = (x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }

    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('N', (0, -1)), ('E', (1, 0)), ('S', (0, 1)), ('W', (-1, 0))]);
    let mut direction = 'N';

    loop {
        let next_movement = directions.get(&direction).unwrap();

        let next_position = (
            current_position.0 + next_movement.0,
            current_position.1 + next_movement.1,
        );

        let next_row = match matrix.get(next_position.1 as usize) {
            Some(row) => row,
            None => break,
        };

        let next_item = match next_row.get(next_position.0 as usize) {
            Some(item) => *item,
            None => break,
        };

        if next_item == '#' {
            direction = match direction {
                'N' => 'E',
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                _ => 'N',
            };
        }

        let movement = directions.get(&direction).unwrap();

        current_position = (
            current_position.0 + movement.0,
            current_position.1 + movement.1,
        );

        visited.insert(current_position);
    }

    println!("{:?}", visited.len());

    Ok(())
}
