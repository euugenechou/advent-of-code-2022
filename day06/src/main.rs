use anyhow::Result;
use itertools::Itertools;

fn find_message(stream: &[char], marker_size: usize) -> String {
    stream
        .windows(marker_size)
        .enumerate()
        .find_map(|(i, window)| (window.iter().all_unique()).then_some(i + marker_size))
        .unwrap()
        .to_string()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let stream = input.chars().collect_vec();

    println!("part 1: {}", find_message(&stream, 4));
    println!("part 2: {}", find_message(&stream, 14));

    Ok(())
}
