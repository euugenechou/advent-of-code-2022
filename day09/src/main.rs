use anyhow::Result;
use itertools::Itertools;
use maplit::{hashmap, hashset};
use std::{cmp::Ordering, fs};

fn drag(from: (isize, isize), delta: (isize, isize)) -> (isize, isize) {
    (from.0 + delta.0, from.1 + delta.1)
}

fn clamp(n: isize) -> isize {
    match n.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn separated((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> Option<(isize, isize)> {
    let (dx, dy) = (x1 - x2, y1 - y2);
    (dx.abs().max(dy.abs()) > 1).then_some((clamp(dx), clamp(dy)))
}

fn simulate(knots: usize, motions: &[(&str, isize)]) -> usize {
    let mut knots = vec![(0, 0); knots];
    let mut visited = hashset![];

    let directions = hashmap![
        "U" => (0, 1),
        "L" => (-1, 0),
        "D" => (0, -1),
        "R" => (1, 0),
    ];

    for (dir, steps) in motions {
        for _ in 0..*steps {
            knots[0] = drag(knots[0], directions[dir]);

            for i in 1..knots.len() {
                if let Some(delta) = separated(knots[i - 1], knots[i]) {
                    knots[i] = drag(knots[i], delta);
                }
            }

            visited.insert(knots[knots.len() - 1]);
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let motions = input
        .lines()
        .filter_map(|line| {
            line.split_once(" ")
                .map(|(dir, steps)| (dir, steps.parse::<isize>().unwrap()))
        })
        .collect_vec();

    println!("part 1: {}", simulate(2, &motions[..]));
    println!("part 2: {}", simulate(10, &motions[..]));

    Ok(())
}
