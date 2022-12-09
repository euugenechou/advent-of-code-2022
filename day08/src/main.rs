use anyhow::Result;
use itertools::{iproduct, Itertools};
use std::{convert::identity, fs};

fn views((r, c): (usize, usize), trees: &[Vec<usize>]) -> Vec<Vec<usize>> {
    vec![
        (0..r).rev().map(|i| trees[i][c]).collect_vec(),
        (0..c).rev().map(|i| trees[r][i]).collect_vec(),
        (r + 1..trees.len()).map(|i| trees[i][c]).collect_vec(),
        (c + 1..trees[0].len()).map(|i| trees[r][i]).collect_vec(),
    ]
}

fn is_visible((r, c): (usize, usize), trees: &[Vec<usize>]) -> bool {
    views((r, c), trees)
        .iter()
        .map(|view| (view.iter().all(|tree| *tree < trees[r][c])))
        .any(identity)
}

fn scenic_score((r, c): (usize, usize), trees: &[Vec<usize>]) -> usize {
    views((r, c), trees)
        .iter()
        .map(|view| {
            view.iter()
                .position(|tree| *tree >= trees[r][c])
                .map(|i| i + 1)
                .unwrap_or(view.len())
        })
        .product()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_string().parse::<usize>().ok())
                .collect_vec()
        })
        .collect_vec();

    println!(
        "part 1: {}",
        iproduct!(0..trees.len(), 0..trees[0].len())
            .into_iter()
            .filter(|&pos| is_visible(pos, &trees))
            .count()
    );

    println!(
        "part 2: {}",
        iproduct!(0..trees.len(), 0..trees[0].len())
            .into_iter()
            .map(|pos| scenic_score(pos, &trees))
            .max()
            .unwrap()
    );

    Ok(())
}
