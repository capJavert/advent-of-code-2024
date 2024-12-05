use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(4, 2024).expect("failed to fetch input");

    let mut matrix: Vec<Vec<char>> = vec![];
    let mut total = 0;

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let character = matrix[i][j];

            if character == 'X' {
                let directions = [
                    (0, 1),   // right
                    (1, 0),   // down
                    (0, -1),  // left
                    (-1, 0),  // up
                    (1, 1),   // down-right
                    (1, -1),  // down-left
                    (-1, 1),  // up-right
                    (-1, -1), // up-left
                ];

                for &(dx, dy) in &directions {
                    let mut collected = String::new();

                    for k in 0..4 {
                        let x = i as isize + k * dx;
                        let y = j as isize + k * dy;

                        if x >= 0
                            && x < matrix.len() as isize
                            && y >= 0
                            && y < matrix[i].len() as isize
                        {
                            collected.push(matrix[x as usize][y as usize]);
                        } else {
                            break;
                        }
                    }
                    if collected == "XMAS" {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}
