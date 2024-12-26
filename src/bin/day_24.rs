use std::collections::HashMap;

use advent_of_code_2024::fetch_input;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Connection {
    a: String,
    b: String,
    operation: String,
    output: String,
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(24, 2024).expect("failed to fetch input");

    let start_values_match = Regex::new(r"([a-z0-9]{3}): (1|0)").unwrap();
    let connections_match =
        Regex::new(r"([a-z0-9]{3}) (AND|XOR|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut values = sections[0].lines().fold(HashMap::new(), |mut acc, line| {
        let captures = start_values_match.captures(line).unwrap();

        acc.insert(
            captures.get(1).unwrap().as_str().to_string(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        );

        acc
    });

    let connections = sections[1]
        .lines()
        .map(|line| {
            let captures = connections_match.captures(line).unwrap();

            Connection {
                a: captures.get(1).unwrap().as_str().to_string(),
                operation: captures.get(2).unwrap().as_str().to_string(),
                b: captures.get(3).unwrap().as_str().to_string(),
                output: captures.get(4).unwrap().as_str().to_string(),
            }
        })
        .collect::<Vec<Connection>>();

    loop {
        let connection = match connections.iter().find(|connection| {
            values.contains_key(&connection.a)
                && values.contains_key(&connection.b)
                && !values.contains_key(&connection.output)
        }) {
            Some(connection) => connection,
            None => break,
        };

        let a = values.get(&connection.a).unwrap();
        let b = values.get(&connection.b).unwrap();

        let result = match connection.operation.as_str() {
            "AND" => {
                if *a == 1 && *b == 1 {
                    1
                } else {
                    0
                }
            }
            "XOR" => {
                if *a != *b {
                    1
                } else {
                    0
                }
            }
            "OR" => {
                if *a == 1 || *b == 1 {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };

        values.insert(connection.output.clone(), result);
    }

    let mut binary = String::new();

    for (key, value) in values.iter().sorted().rev() {
        if key.starts_with("z") {
            binary.push_str(&value.to_string());
        }
    }

    println!("{:?}", usize::from_str_radix(&binary, 2).unwrap());

    Ok(())
}
