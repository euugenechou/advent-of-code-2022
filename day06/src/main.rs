use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let chars = input.chars().collect_vec();

    println!(
        "part 1: {}",
        chars
            .windows(4)
            .enumerate()
            .find_map(|(i, w)| (w
                .iter()
                .map(|c| (1 << (u32::from(*c) - 'a' as u32)) as usize)
                .fold(0, |a, c| a | c)
                .count_ones()
                == 4)
                .then_some(i + 4))
            .unwrap()
            .to_string()
    );

    println!(
        "part 2: {}",
        chars
            .windows(14)
            .enumerate()
            .find_map(|(i, w)| (w
                .iter()
                .map(|c| (1 << (u32::from(*c) - 'a' as u32)) as usize)
                .fold(0, |a, c| a | c)
                .count_ones()
                == 14)
                .then_some(i + 14))
            .unwrap()
            .to_string()
    );

    Ok(())
}
