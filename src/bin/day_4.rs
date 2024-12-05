use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(4, 2024).expect("failed to fetch input");

    let mut matrix: Vec<Vec<char>> = vec![];
    let mut total = 0;

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let character = matrix[y][x];

            if character == 'A' {
                if y > 0 && y < matrix.len() - 1 && x > 0 && x < matrix[y].len() - 1 {
                    let left_right = [matrix[y - 1][x - 1], matrix[y][x], matrix[y + 1][x + 1]]
                        .iter()
                        .collect::<String>();

                    let right_left = [matrix[y - 1][x + 1], matrix[y][x], matrix[y + 1][x - 1]]
                        .iter()
                        .collect::<String>();

                    if (left_right == "MAS" || left_right == "SAM")
                        && (right_left == "MAS" || right_left == "SAM")
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}
