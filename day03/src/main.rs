use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs};

fn halve(line: &str) -> [&str; 2] {
    let halves = line.split_at(line.len() / 2);
    [halves.0, halves.1]
}

fn common(collections: &[&str]) -> HashSet<char> {
    collections
        .iter()
        .map(|collection| collection.chars().collect::<HashSet<_>>())
        .reduce(|a, b| &a & &b)
        .unwrap()
}

fn eval(item: char) -> u32 {
    match item {
        'a'..='z' => u32::from(item) - 96,
        _ => u32::from(item) - 38,
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    println!(
        "part 1: {:?}",
        input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(halve)
            .flat_map(|rucksack| common(&rucksack).into_iter())
            .map(eval)
            .sum::<u32>()
    );

    println!(
        "part 2: {:?}",
        input
            .split("\n")
            .filter(|line| !line.is_empty())
            .chunks(3)
            .into_iter()
            .flat_map(|group| common(&group.collect_vec()).into_iter())
            .map(eval)
            .sum::<u32>()
    );

    Ok(())
}
