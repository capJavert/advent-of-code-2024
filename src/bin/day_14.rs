use std::collections::HashMap;

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

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;

            robot.position.0 = teleport(robot.position.0, width);
            robot.position.1 = teleport(robot.position.1, height);
        }
    }

    let quadrants = [
        (0, width / 2, 0, height / 2),
        (width / 2 + 1, width, 0, height / 2),
        (0, width / 2, height / 2 + 1, height),
        (width / 2 + 1, width, height / 2 + 1, height),
    ];

    let result = robots.iter().fold(HashMap::new(), |mut acc, robot| {
        match quadrants.iter().position(|(x1, x2, y1, y2)| {
            robot.position.0 >= *x1
                && robot.position.0 < *x2
                && robot.position.1 >= *y1
                && robot.position.1 < *y2
        }) {
            Some(index) => {
                let entry = acc.entry(index).or_insert(0);
                *entry += 1;
            }
            None => (),
        };

        acc
    });

    println!("{:?}", result.values().fold(1, |acc, value| acc * value));

    Ok(())
}
