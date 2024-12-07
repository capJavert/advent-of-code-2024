use std::collections::{HashMap, HashSet};

use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(6, 2024).expect("failed to fetch input");

    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut initial_position: (i32, i32) = (0, 0);

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];

            if item == '^' {
                initial_position = (x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }

    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('N', (0, -1)), ('E', (1, 0)), ('S', (0, 1)), ('W', (-1, 0))]);

    let mut loops = HashSet::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] != '.' {
                continue;
            }

            let mut direction = 'N';
            let mut current_position = initial_position.clone();
            let mut visited = HashMap::new();

            let before_char = matrix[y][x];
            matrix[y][x] = '#';

            'loop_checker: loop {
                loop {
                    let next_movement = directions.get(&direction).unwrap();

                    let next_position = (
                        current_position.0 + next_movement.0,
                        current_position.1 + next_movement.1,
                    );

                    let next_row = match matrix.get(next_position.1 as usize) {
                        Some(row) => row,
                        None => break 'loop_checker,
                    };

                    let next_item = match next_row.get(next_position.0 as usize) {
                        Some(item) => *item,
                        None => break 'loop_checker,
                    };

                    if next_item == '#' {
                        direction = match direction {
                            'N' => 'E',
                            'E' => 'S',
                            'S' => 'W',
                            'W' => 'N',
                            _ => 'N',
                        }
                    } else {
                        current_position = next_position;

                        break;
                    }
                }

                let current_visited = visited.get(&(current_position, direction)).unwrap_or(&0);
                let visited_count = current_visited + 1;

                visited.insert((current_position, direction), visited_count);

                if visited_count > 5 {
                    loops.insert((x, y));

                    break;
                }
            }

            matrix[y][x] = before_char;
        }
    }

    println!("{:?}", loops.len());

    Ok(())
}
