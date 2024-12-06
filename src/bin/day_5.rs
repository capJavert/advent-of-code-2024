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

    let mut page_updates = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut valid_updates = vec![];

    let is_page_update_valid = |page_update: &Vec<usize>| -> (Vec<usize>, bool) {
        let mut valid = true;
        let mut new_page_update = page_update.clone();

        'checker: for (index, page) in page_update.iter().enumerate() {
            for (a, b) in page_rules.iter() {
                for (index_check, page_check) in page_update.iter().enumerate() {
                    if a == page {
                        if b == page_check && index > index_check {
                            valid = false;
                            new_page_update[index] = *b;
                            new_page_update[index_check] = *a;

                            break 'checker;
                        }
                    }

                    if b == page {
                        if a == page_check && index < index_check {
                            valid = false;
                            new_page_update[index] = *a;
                            new_page_update[index_check] = *b;

                            break 'checker;
                        }
                    }
                }
            }
        }

        (new_page_update, valid)
    };

    let mut index = 0;
    let mut invalid_count = 0;

    while index < page_updates.len() {
        let page_update = &page_updates[index];
        let (new_page_update, valid) = is_page_update_valid(page_update);

        if valid {
            if invalid_count > 0 {
                valid_updates.push(new_page_update);
            }

            index += 1;
            invalid_count = 0;

            continue;
        } else {
            invalid_count += 1;

            page_updates[index] = new_page_update;
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
