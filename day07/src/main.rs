use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs, path::PathBuf};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut pwd = PathBuf::new();
    let mut dirs = HashMap::new();

    input
        .lines()
        .map(|line| line.split(" ").collect_vec())
        .for_each(|cmd| match cmd[..] {
            ["$", "cd", ".."] => {
                pwd.pop();
            }
            ["$", "cd", path] => {
                pwd.push(path);
                dirs.entry(pwd.clone()).or_default();
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                pwd.iter().fold(PathBuf::new(), |mut acc, c| {
                    acc.push(c);
                    *dirs.entry(acc.clone()).or_default() += size.parse::<usize>().unwrap();
                    acc
                });
            }
            _ => {}
        });

    println!(
        "part 1: {}",
        dirs.values().filter(|size| **size <= 100000).sum::<usize>()
    );

    println!(
        "part 2: {}",
        dirs.values()
            .filter(|size| 70000000 - dirs[&PathBuf::from("/")] + **size >= 30000000)
            .min()
            .unwrap()
    );

    Ok(())
}
