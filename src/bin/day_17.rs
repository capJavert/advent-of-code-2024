use advent_of_code_2024::fetch_input;
use regex::Regex;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(17, 2024).expect("failed to fetch input");

    let program_parser = Regex::new(r"[0-9]+").unwrap();
    let captures = program_parser
        .find_iter(&input)
        .map(|item| item.as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let program = &captures[3..];

    let mut registers = (captures[0], captures[1], captures[2]);
    let mut pointer = 0;
    let mut did_output = false;

    loop {
        let opcode = match program.get(pointer) {
            Some(&op) => op,
            None => break,
        };

        let operand = match program.get(pointer + 1) {
            Some(&op) => op,
            None => break,
        };

        let combo = match operand {
            4 => registers.0,
            5 => registers.1,
            6 => registers.2,
            7 => 7,
            _ => operand,
        };

        pointer += 2;

        match opcode {
            0 => {
                let numerator = registers.0;
                let denominator = (2 as u32).pow(combo);
                let result = numerator / denominator;

                registers.0 = result;
            }
            1 => {
                let result = registers.1 ^ operand;

                registers.1 = result;
            }
            2 => {
                let result = combo % 8;

                registers.1 = result;
            }
            3 => {
                if registers.0 != 0 {
                    pointer = operand as usize;
                }
            }
            4 => {
                let result = registers.1 ^ registers.2;

                registers.1 = result;
            }
            5 => {
                let result = combo % 8;

                if did_output {
                    print!(",")
                }

                did_output = true;

                print!("{}", result);
            }
            6 => {
                let numerator = registers.0;
                let denominator = (2 as u32).pow(combo);
                let result = numerator / denominator;

                registers.1 = result;
            }
            7 => {
                let numerator = registers.0;
                let denominator = (2 as u32).pow(combo);
                let result = numerator / denominator;

                registers.2 = result;
            }
            _ => panic!("invalid opcode"),
        }
    }

    println!();

    Ok(())
}
