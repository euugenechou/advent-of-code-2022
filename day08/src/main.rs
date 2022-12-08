use anyhow::Result;
use itertools::{iproduct, izip, Itertools};
use std::fs;

fn eval_direction(
    (r, c): (usize, usize),
    range: impl Iterator<Item = (usize, usize)>,
    trees: &[Vec<usize>],
) -> (bool, usize) {
    let mut visible = true;
    let mut score = 0;

    for (i, j) in range.into_iter() {
        score += 1;
        if trees[i][j] >= trees[r][c] {
            visible = false;
            break;
        }
    }

    (visible, score)
}

fn eval((r, c): (usize, usize), trees: &[Vec<usize>]) -> (bool, usize) {
    let top = eval_direction((r, c), izip!((0..r).rev(), [c].into_iter().cycle()), &trees);

    let left = eval_direction((r, c), izip!([r].into_iter().cycle(), (0..c).rev()), &trees);

    let bottom = eval_direction(
        (r, c),
        izip!(r + 1..trees.len(), [c].into_iter().cycle()),
        &trees,
    );

    let right = eval_direction(
        (r, c),
        izip!([r].into_iter().cycle(), c + 1..trees[0].len()),
        &trees,
    );

    let visible = top.0 || left.0 || bottom.0 || right.0;
    let score = top.1 * left.1 * bottom.1 * right.1;

    (visible, score)
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
            .map(|coord| eval(coord, &trees))
            .filter(|e| e.0)
            .count()
    );

    println!(
        "part 2: {}",
        iproduct!(0..trees.len(), 0..trees[0].len())
            .into_iter()
            .map(|coord| eval(coord, &trees))
            .map(|e| e.1)
            .max()
            .unwrap()
    );

    Ok(())
}
