use std::{env, fs};

pub fn fetch_input(day: u32, year: u32) -> Result<String, Box<reqwest::Error>> {
    dotenv::dotenv().ok();

    let file_name = format!("inputs/input-{}-{}.txt", day, year);

    match fs::read_to_string(&file_name) {
        Ok(input) => Ok(input),
        Err(_) => {
            println!("no local input, fetching");

            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day
                ))
                .header(
                    "cookie",
                    format!(
                        "session={}",
                        env::var("AOC_SESSION").expect("AOC_SESSION not set")
                    ),
                )
                .header(
                    "user-agent",
                    format!("github.com/capjavert/advent-of-code-{}", year),
                )
                .send()?;

            let input = response.text()?;
            fs::write(&file_name, &input).expect("failed to write input cache");

            Ok(input)
        }
    }
}

pub fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for item in row {
            print!("{}", item);
        }

        println!();
    }
}

pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
