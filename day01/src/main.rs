use anyhow::Result;
use std::{collections::BTreeSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let calories = input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|c| c.parse::<usize>().ok())
                .sum::<usize>()
        })
        .collect::<BTreeSet<_>>();

    println!("part 1: {}", calories.iter().rev().take(1).sum::<usize>());
    println!("part 2: {}", calories.iter().rev().take(3).sum::<usize>());

    Ok(())
}
