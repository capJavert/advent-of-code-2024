use advent_of_code_2024::fetch_input;
use regex::Regex;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(3, 2024).expect("failed to fetch input");

    let mul_match = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut total = 0;

    for (_, [a, b]) in mul_match.captures_iter(&input).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    println!("{:?}", total);

    Ok(())
}
