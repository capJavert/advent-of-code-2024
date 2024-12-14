use advent_of_code_2024::fetch_input;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(14, 2024).expect("failed to fetch input");
    let width = 101;
    let height = 103;

    let line_match = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let mut robots = input
        .lines()
        .map(|line| {
            let captures = line_match.captures(line).unwrap();

            Robot {
                position: (
                    captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ),
                velocity: (
                    captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Robot>>();

    let teleport = |position: i64, bound: i64| -> i64 {
        if position < 0 {
            bound + position
        } else if position >= bound {
            position - bound
        } else {
            position
        }
    };

    for second in 0..10000 {
        for robot_index in 0..robots.len() {
            let robot = robots.get_mut(robot_index).unwrap();
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;

            robot.position.0 = teleport(robot.position.0, width);
            robot.position.1 = teleport(robot.position.1, height);
        }

        if second > 5000 {
            println!("Second: {}", second);

            for y in 0..height {
                let mut line = String::from("");

                for x in 0..width {
                    if robots.iter().any(|robot| robot.position == (x, y)) {
                        line += " # ";
                    } else {
                        line += " . ";
                    }
                }

                println!("{}", line);
            }
        }
    }

    Ok(())
}
