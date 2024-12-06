use advent_of_code_2024::fetch_input;
use itertools::Itertools;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(5, 2024).expect("failed to fetch input");
    let sections: Vec<&str> = input.split("\n\n").collect();

    let page_rules = sections[0]
        .lines()
        .map(|line| {
            line.split("|")
                .map(|item| item.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(usize, usize)>>();

    let page_updates = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut valid_updates = vec![];

    for page_update in page_updates.iter() {
        let mut valid = true;

        'checker: for (index, page) in page_update.iter().enumerate() {
            for (a, b) in page_rules.iter() {
                for (index_check, page_check) in page_update.iter().enumerate() {
                    if a == page {
                        if b == page_check && index > index_check {
                            valid = false;

                            break 'checker;
                        }
                    }

                    if b == page {
                        if a == page_check && index < index_check {
                            valid = false;

                            break 'checker;
                        }
                    }
                }
            }
        }

        if valid {
            valid_updates.push(page_update);
        }
    }

    let total = valid_updates.iter().fold(0, |acc, page_update| {
        let middle_index = page_update.len() / 2;
        let middle_page = page_update[middle_index];

        acc + middle_page
    });

    println!("{}", total);

    Ok(())
}
