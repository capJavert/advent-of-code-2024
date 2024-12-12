use std::{
    collections::{HashMap, HashSet},
    vec,
};

use advent_of_code_2024::fetch_input;

fn find_path(next: (i32, i32), fields: &mut HashSet<(i32, i32)>, matrix: &Vec<Vec<char>>) {
    fields.insert(next);

    let field = matrix
        .get(next.1 as usize)
        .unwrap()
        .get(next.0 as usize)
        .unwrap();

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let next_paths = directions.iter().fold(vec![], |mut acc, (dx, dy)| {
        let next_x = next.0 + dx;
        let next_y = next.1 + dy;

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        let next_field = match matrix.get(next_y as usize) {
            Some(row) => row.get(next_x as usize).unwrap_or(&'Č'),
            None => &'Č',
        };

        if field != &'Č' && field == next_field && !fields.contains(&(next_x as i32, next_y as i32))
        {
            acc.push((next_x as i32, next_y as i32));
        }

        acc
    });

    for next_path in next_paths {
        find_path(next_path, fields, matrix);
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(12, 2024).expect("failed to fetch input");

    let matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut visited = HashMap::new();

    let mut groups = vec![];

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if visited.contains_key(&(x, y)) {
                continue;
            }

            visited.insert((x, y), true);

            let mut fields = HashSet::new();

            find_path((x as i32, y as i32), &mut fields, &matrix);

            for field in &fields {
                visited.insert((field.0 as usize, field.1 as usize), true);
            }

            groups.push(fields);
        }
    }

    let mut price = 0;

    for group in groups.iter() {
        let mut medja = vec![];

        for field in group {
            let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

            for (dx, dy) in directions.iter() {
                let next_x = field.0 + dx;
                let next_y = field.1 + dy;

                let next_x = next_x as usize;
                let next_y = next_y as usize;

                if !group.contains(&(next_x as i32, next_y as i32)) {
                    medja.push((next_x as i32, next_y as i32));
                }
            }
        }

        let group_price = group.len() * medja.len();

        price += group_price;
    }

    println!("{:?}", price);

    Ok(())
}
