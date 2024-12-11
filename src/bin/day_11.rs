use advent_of_code_2024::fetch_input;

#[derive(Debug, Clone)]
struct Stone {
    value: usize,
    digits: Vec<usize>,
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(11, 2024).expect("failed to fetch input");

    let mut stones = input
        .trim()
        .split(" ")
        .map(|item| {
            let value = item.parse::<usize>().unwrap();
            let digits = value
                .to_string()
                .chars()
                .map(|item| item.to_string().parse().unwrap())
                .collect::<Vec<usize>>();

            Stone { value, digits }
        })
        .collect::<Vec<Stone>>();

    for _ in 0..25 {
        let mut stones_new = vec![];

        for stone in stones {
            if stone.value == 0 {
                stones_new.push(Stone {
                    value: 1,
                    digits: vec![1],
                });
            } else if stone.digits.len() % 2 == 0 {
                let split_index = stone.digits.len() / 2;

                for slice in [&stone.digits[0..split_index], &stone.digits[split_index..]] {
                    let mut slice_padded = vec![];

                    let mut zero = true;

                    for digit in slice {
                        if zero {
                            if digit == &0 {
                                continue;
                            } else {
                                zero = false;

                                slice_padded.push(*digit);
                            }
                        } else {
                            slice_padded.push(*digit);
                        }
                    }

                    stones_new.push(Stone {
                        value: slice
                            .iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                            .parse::<usize>()
                            .unwrap(),
                        digits: slice_padded,
                    });
                }
            } else {
                let new_stone_value = stone.value * 2024;

                stones_new.push(Stone {
                    value: new_stone_value,
                    digits: new_stone_value
                        .to_string()
                        .chars()
                        .map(|item| item.to_string().parse().unwrap())
                        .collect::<Vec<usize>>(),
                });
            }
        }

        stones = stones_new.clone();
    }

    println!("{:?}", stones.len());

    Ok(())
}
