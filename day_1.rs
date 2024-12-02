use advent_of_code_2024::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(1, 2024).expect("failed to fetch input");

    let mut list_a = vec![];
    let mut list_b = vec![];

    for line in input.lines() {
        let [a, b] = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .as_slice()
            .try_into()
            .unwrap();

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort();
    list_b.sort();

    let mut total = 0;

    for (a, b) in list_a.iter().zip(list_b.iter()) {
        total += (a - b).abs();
    }

    println!("{}", total);

    Ok(())
}
