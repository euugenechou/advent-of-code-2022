use anyhow::Result;
use std::io;

fn unsnafu(s: &str) -> isize {
    s.chars().fold(0, |acc, c| {
        (acc * 5)
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                _ => -2,
            }
    })
}

fn snafu(mut n: isize) -> String {
    let mut s = String::new();
    while n != 0 {
        let r = (n + 2) % 5 - 2;
        s.push(match r {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            _ => '=',
        });
        n = (n - r) / 5;
    }
    s.chars().rev().collect()
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    println!(
        "part 1: {}",
        snafu(input.trim().lines().map(unsnafu).sum::<isize>())
    );

    Ok(())
}
