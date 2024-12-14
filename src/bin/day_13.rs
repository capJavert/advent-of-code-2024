use advent_of_code_2024::fetch_input;
use regex::Regex;

fn solve(machine: &Machine) -> Option<(i64, i64)> {
    // Exquations example
    // Equation 1: 94x + 22y = 8400
    // Equation 2: 34x + 67y = 5400

    let a = machine.a_x;
    let b = machine.b_x;
    let c = machine.prize_x;

    let d = machine.a_y;
    let e = machine.b_y;
    let f = machine.prize_y;

    // Determinant of the system
    let det = a * e - b * d;

    // Cramer's rule
    let x = (c * e - b * f) / det;
    let y = (a * f - c * d) / det;

    if (machine.a_x * x + machine.b_x * y) == machine.prize_x
        && (machine.a_y * x + machine.b_y * y) == machine.prize_y
    {
        Some((x, y))
    } else {
        None
    }
}

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(13, 2024).expect("failed to fetch input");

    let button_match = Regex::new(r"Button (A|B): X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_match = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let offset = 10000000000000;

    let mut machines = input.lines().fold(
        vec![Machine {
            a_x: 0,
            a_y: 0,
            b_x: 0,
            b_y: 0,
            prize_x: 0,
            prize_y: 0,
        }],
        |mut acc, line| {
            if line != "" {
                if line.starts_with("Button") {
                    let current_machine = acc.last_mut().unwrap();

                    let captures = button_match.captures(line).unwrap();

                    if captures.get(1).unwrap().as_str() == "A" {
                        current_machine.a_x =
                            captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                        current_machine.a_y =
                            captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
                    } else {
                        current_machine.b_x =
                            captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                        current_machine.b_y =
                            captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
                    }
                } else {
                    let current_machine = acc.last_mut().unwrap();

                    let captures = prize_match.captures(line).unwrap();

                    current_machine.prize_x =
                        captures.get(1).unwrap().as_str().parse::<i64>().unwrap() + offset;
                    current_machine.prize_y =
                        captures.get(2).unwrap().as_str().parse::<i64>().unwrap() + offset;

                    acc.push(Machine {
                        a_x: 0,
                        a_y: 0,
                        b_x: 0,
                        b_y: 0,
                        prize_x: 0,
                        prize_y: 0,
                    });
                }
            }

            acc
        },
    );

    machines.pop();

    let mut tokens = 0;

    for machine in machines.iter() {
        match solve(machine) {
            Some((a, b)) => {
                tokens += 3 * a;
                tokens += 1 * b;
            }
            None => (),
        }
    }

    println!("{}", tokens);

    Ok(())
}
