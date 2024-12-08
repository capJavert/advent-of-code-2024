use std::vec;

use advent_of_code_2024::fetch_input;
use itertools::Itertools;

#[derive(Debug)]
struct Test {
    expect: i64,
    input: Vec<i64>,
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(7, 2024).expect("failed to fetch input");

    let tests: Vec<Test> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let expect = parts.next().unwrap().parse().unwrap();
            let input = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            Test { expect, input }
        })
        .collect();

    let operators = Vec::from(['+', '*', '|']);

    let mut valid_tests = vec![];

    for item in tests {
        for operators in (0..item.input.len() - 1)
            .map(|_| operators.clone())
            .multi_cartesian_product()
        {
            let mut inputs_iter = item.input.iter();
            let mut result = *inputs_iter.next().unwrap();

            for op in operators.iter() {
                let item = inputs_iter.next().unwrap();

                match op {
                    '+' => result += item,
                    '*' => result *= item,
                    '|' => result = format!("{}{}", result, item).parse().unwrap(),
                    _ => unreachable!(),
                };
            }

            if result == item.expect {
                valid_tests.push(item);

                break;
            }
        }
    }

    let result = valid_tests
        .into_iter()
        .fold(0, |acc, item| acc + item.expect);

    println!("{}", result);

    Ok(())
}
