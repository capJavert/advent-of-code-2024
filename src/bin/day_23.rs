use std::collections::HashSet;
use std::io;

use advent_of_code_2024::fetch_input;

fn main() -> io::Result<()> {
    let input = fetch_input(23, 2024).expect("failed to fetch input");

    // Parse the connections into a HashSet of tuples
    let mut connections: HashSet<(String, String)> = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            let a = parts[0].to_string();
            let b = parts[1].to_string();
            connections.insert((a.clone(), b.clone()));
            connections.insert((b, a));
        }
    }

    // Find all sets of three interconnected computers
    let mut sets_of_three = HashSet::new();
    let mut filtered_sets = HashSet::new();

    for (a, b) in &connections {
        for (c, d) in &connections {
            if b == d && connections.contains(&(a.clone(), c.clone())) {
                let mut trio = vec![a.clone(), b.clone(), c.clone()];
                trio.sort(); // Ensure consistent ordering
                sets_of_three.insert(trio.clone());
                if trio.iter().any(|name| name.starts_with('t')) {
                    filtered_sets.insert(trio);
                }
            }
        }
    }

    println!("{}", filtered_sets.len());

    Ok(())
}
