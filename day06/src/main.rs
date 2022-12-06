use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let chars = input.chars().collect_vec();

    println!(
        "part 1: {}",
        chars
            .windows(4)
            .enumerate()
            .find_map(|(i, w)| (w.iter().collect::<HashSet<_>>().len() == 4).then_some(i + 4))
            .unwrap()
            .to_string()
    );

    println!(
        "part 2: {}",
        chars
            .windows(14)
            .enumerate()
            .find_map(|(i, w)| (w.iter().collect::<HashSet<_>>().len() == 14).then_some(i + 14))
            .unwrap()
            .to_string()
    );

    Ok(())
}
