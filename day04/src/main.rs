use anyhow::Result;
use std::fs;

fn parse(range: &str) -> (usize, usize) {
    range
        .split_once("-")
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap()
}

fn contains(&((a1, a2), (b1, b2)): &((usize, usize), (usize, usize))) -> bool {
    a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2
}

fn overlaps(&((a1, a2), (b1, b2)): &((usize, usize), (usize, usize))) -> bool {
    !(a1 > b2 || b1 > a2)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    println!(
        "part 1: {}",
        input
            .split("\n")
            .filter_map(|line| line.split_once(","))
            .map(|(a, b)| (parse(a), parse(b)))
            .filter(|ranges| contains(ranges))
            .count()
    );

    println!(
        "part 2: {}",
        input
            .split("\n")
            .filter_map(|line| line.split_once(","))
            .map(|(a, b)| (parse(a), parse(b)))
            .filter(|ranges| overlaps(ranges))
            .count()
    );

    Ok(())
}
