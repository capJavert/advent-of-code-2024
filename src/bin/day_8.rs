use std::collections::HashSet;

use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(8, 2024).expect("failed to fetch input");

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for ya in 0..matrix.len() {
        for xa in 0..matrix[ya].len() {
            let point_a = matrix[ya][xa];

            if point_a == '.' {
                continue;
            }

            for yb in 0..matrix.len() {
                for xb in 0..matrix[yb].len() {
                    let point_b = matrix[yb][xb];

                    if point_a != point_b {
                        continue;
                    }

                    if xa == xb && ya == yb {
                        continue;
                    }

                    let distance_x = xb as i32 - xa as i32;
                    let distance_y = yb as i32 - ya as i32;

                    let new_antinodes = [(xb as i32 + distance_x, yb as i32 + distance_y)];

                    for antinode in new_antinodes {
                        if antinode.0 < 0 || antinode.1 < 0 {
                            continue;
                        }

                        if antinode.0 >= matrix[0].len() as i32 || antinode.1 >= matrix.len() as i32
                        {
                            continue;
                        }

                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}
