use advent_of_code_2024::fetch_input;

fn get_next_position(position: (usize, usize), movement: &char) -> (usize, usize) {
    match movement {
        '^' => (position.0, position.1 - 1),
        'v' => (position.0, position.1 + 1),
        '<' => (position.0 - 1, position.1),
        '>' => (position.0 + 1, position.1),
        _ => panic!("invalid movement"),
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(15, 2024).expect("failed to fetch input");

    let (input_matrix, input_movements) = input.split_once("\n\n").unwrap();

    let mut matrix = input_matrix
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let movements = input_movements
        .replace("\n", "")
        .chars()
        .collect::<Vec<char>>();

    let start_item = matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (x, y, cell)))
        .find(|(_, _, cell)| *cell == '@')
        .unwrap();
    matrix[start_item.1][start_item.0] = '.';
    let mut position = (start_item.0, start_item.1);

    for movement in movements.iter() {
        let next_position = get_next_position(position, movement);

        let next_item = matrix[next_position.1][next_position.0];

        if next_item == '#' {
            continue;
        } else if next_item == 'O' {
            let mut empty_space_position = Option::None;
            let mut next_empty_position = next_position;

            loop {
                next_empty_position = get_next_position(next_empty_position, movement);
                let next_item = matrix[next_empty_position.1][next_empty_position.0];

                if next_item == '.' {
                    empty_space_position = Some(next_empty_position);

                    break;
                }

                if next_item == '#' {
                    break;
                }
            }

            if let Some(empty_space_position) = empty_space_position {
                matrix[next_position.1][next_position.0] = '.';
                matrix[empty_space_position.1][empty_space_position.0] = 'O';

                position = next_position;
            }
        } else {
            position = next_position;
        }
    }

    let mut result = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let char = matrix[y][x];

            if char == 'O' {
                result += 100 * y + x;
            }
        }
    }

    println!("{}", result);

    Ok(())
}
