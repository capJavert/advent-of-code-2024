use advent_of_code_2024::fetch_input;
use regex::Regex;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(3, 2024).expect("failed to fetch input");

    let instructions_match = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
    let mul_match = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut total = 0;
    let mut instructions = vec![];

    for (line, []) in instructions_match
        .captures_iter(&input)
        .map(|c| c.extract())
    {
        instructions.push(line);
    }

    let mut enabled = true;

    for instruction in instructions.into_iter() {
        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else {
            let captures = mul_match.captures(instruction).unwrap();

            let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

            if enabled {
                total += a * b;
            }
        }
    }

    println!("{:?}", total);

    Ok(())
}
