use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(22, 2024).expect("failed to fetch input");

    let numbers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut secrets = vec![];

    for number in numbers {
        let mut secret = number;

        for _ in 0..2000 {
            secret = secret ^ (secret * 64);
            secret %= 16777216;

            let divider = secret as f64 / 32.0;
            secret = secret ^ divider.floor() as usize;
            secret %= 16777216;

            secret = secret ^ (secret * 2048);
            secret %= 16777216;

            secret = secret;
        }

        secrets.push(secret);
    }

    println!("{:?}", secrets.iter().fold(0, |acc, secret| acc + secret));

    Ok(())
}
