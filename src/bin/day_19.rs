use std::collections::HashMap;

use advent_of_code_2024::fetch_input;

fn create_design(
    design: &str,
    patterns: &Vec<&str>,
    mut combinations: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if cache.contains_key(design) {
        return cache[design];
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if design == *pattern {
                combinations += 1;
            } else {
                combinations += create_design(&design[pattern.len()..], patterns, 0, cache);
            }
        }
    }

    cache.insert(design.to_string(), combinations);

    combinations
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(19, 2024).expect("failed to fetch input");

    let sections: Vec<&str> = input.split("\n\n").collect();

    let patterns = sections[0].trim().split(", ").collect::<Vec<_>>();

    let designs = sections[1].lines().collect::<Vec<_>>();

    let mut valid_designs = 0;

    let mut cache = HashMap::new();

    for design in designs {
        let combinations = create_design(design, &patterns, 0, &mut cache);

        if combinations > 0 {
            valid_designs += 1;
        }
    }

    println!("{}", valid_designs);

    Ok(())
}
