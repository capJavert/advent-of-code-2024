use std::{cmp::Ordering, vec};

use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(2, 2024).expect("failed to fetch input");

    let mut safe_reports = vec![];

    let mut potential_reports: Vec<Vec<Vec<i32>>> = vec![];

    for (line_index, line) in input.lines().enumerate() {
        let levels = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        potential_reports.push(vec![levels.clone()]);

        for (index, _) in levels.iter().enumerate() {
            let mut adjusted_levels = levels.clone();
            adjusted_levels.remove(index);

            potential_reports
                .get_mut(line_index)
                .unwrap()
                .push(adjusted_levels);
        }
    }

    for reports in potential_reports.iter() {
        'report_loop: for levels in reports.iter() {
            let mut direction = 0;

            for (index, level) in levels.iter().enumerate() {
                let next_level = match levels.get(index + 1) {
                    Some(level) => level,
                    None => {
                        safe_reports.push(levels.clone());

                        break 'report_loop;
                    }
                };

                let next_direction = match next_level.cmp(&level) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };

                if direction == 0 {
                    direction = next_direction;
                } else if direction != next_direction {
                    break;
                }

                let diff = (next_level - level).abs();

                if diff < 1 || diff > 3 {
                    break;
                }
            }
        }
    }

    println!("{}", safe_reports.len());

    Ok(())
}
