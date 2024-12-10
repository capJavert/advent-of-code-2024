use std::collections::HashMap;

use advent_of_code_2024::fetch_input;

fn find_path(
    next: (i32, i32),
    path: &mut Vec<(i32, i32)>,
    paths: &mut Vec<Vec<(i32, i32)>>,
    matrix: &Vec<Vec<i32>>,
) {
    path.push(next);

    let field = matrix
        .get(next.1 as usize)
        .unwrap()
        .get(next.0 as usize)
        .unwrap();

    if field == &9 {
        paths.push(path.clone());
    } else {
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let next_paths = directions.iter().fold(vec![], |mut acc, (dx, dy)| {
            let next_x = next.0 + dx;
            let next_y = next.1 + dy;

            let next_x = next_x as usize;
            let next_y = next_y as usize;

            let next_field = match matrix.get(next_y as usize) {
                Some(row) => row.get(next_x as usize).unwrap_or(&-1),
                None => &-1,
            };

            if field + 1 == *next_field {
                acc.push((next_x as i32, next_y as i32));
            }

            acc
        });

        for next_path in next_paths {
            find_path(next_path, &mut path.clone(), paths, matrix);
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(10, 2024).expect("failed to fetch input");

    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|item| item.to_string().parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut trailheads = HashMap::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let point = matrix[y][x];

            if point != 0 {
                continue;
            }

            let mut paths = vec![];

            find_path((x as i32, y as i32), &mut vec![], &mut paths, &matrix);

            trailheads.insert((x as i32, y as i32), paths.len());
        }
    }

    println!("{:?}", trailheads.values().sum::<usize>());

    Ok(())
}
