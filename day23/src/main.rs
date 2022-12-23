use anyhow::Result;
use itertools::iproduct;
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Pos = (isize, isize);

fn isolated((r, c): Pos, positions: &HashSet<Pos>) -> bool {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .all(|(dr, dc)| !positions.contains(&(r + dr, c + dc)))
}

fn north((r, c): Pos, positions: &HashSet<Pos>) -> Option<(Pos, Pos)> {
    [(-1, -1), (-1, 0), (-1, 1)]
        .iter()
        .all(|(dr, dc)| !positions.contains(&(r + dr, c + dc)))
        .then_some(((r, c), (r - 1, c)))
}

fn south((r, c): Pos, positions: &HashSet<Pos>) -> Option<(Pos, Pos)> {
    [(1, -1), (1, 0), (1, 1)]
        .iter()
        .all(|(dr, dc)| !positions.contains(&(r + dr, c + dc)))
        .then_some(((r, c), (r + 1, c)))
}

fn west((r, c): Pos, positions: &HashSet<Pos>) -> Option<(Pos, Pos)> {
    [(-1, -1), (0, -1), (1, -1)]
        .iter()
        .all(|(dr, dc)| !positions.contains(&(r + dr, c + dc)))
        .then_some(((r, c), (r, c - 1)))
}

fn east((r, c): Pos, positions: &HashSet<Pos>) -> Option<(Pos, Pos)> {
    [(-1, 1), (0, 1), (1, 1)]
        .iter()
        .all(|(dr, dc)| !positions.contains(&(r + dr, c + dc)))
        .then_some(((r, c), (r, c + 1)))
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let mut positions = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(c, x)| (x == '#').then_some((r as isize, c as isize)))
        })
        .collect::<HashSet<_>>();

    let mut checks = [north, south, west, east];

    for rounds in 1.. {
        let prev_positions = positions.clone();

        let proposals = positions
            .iter()
            .map(|pos| {
                if !isolated(*pos, &positions) {
                    for check in &checks {
                        if let Some(proposal) = check(*pos, &positions) {
                            return proposal;
                        }
                    }
                }
                (*pos, *pos)
            })
            .fold(HashMap::new(), |mut acc, (pos, new_pos)| {
                acc.entry(new_pos).or_insert((pos, new_pos, 0)).2 += 1;
                acc
            });

        proposals.values().for_each(|(pos, new_pos, count)| {
            if *count == 1 {
                positions.remove(pos);
                positions.insert(*new_pos);
            }
        });

        if rounds == 10 {
            let min_bound = (
                *positions.iter().map(|(r, _)| r).min().unwrap(),
                *positions.iter().map(|(_, c)| c).min().unwrap(),
            );
            let max_bound = (
                *positions.iter().map(|(r, _)| r).max().unwrap(),
                *positions.iter().map(|(_, c)| c).max().unwrap(),
            );
            println!(
                "part 1: {}",
                iproduct!((min_bound.0)..=(max_bound.0), (min_bound.1)..=(max_bound.1))
                    .into_iter()
                    .filter(|pos| !positions.contains(pos))
                    .count()
            );
        }

        if prev_positions == positions {
            println!("part 2: {rounds}");
            break;
        }

        checks.rotate_left(1);
    }

    Ok(())
}
