use anyhow::Result;
use itertools::Itertools;
use std::{collections::BTreeMap, fs};

type Crate = Vec<char>;
type Move = (usize, usize, usize);

fn parse_crates(grid: &str) -> BTreeMap<usize, Crate> {
    let mut crates: BTreeMap<usize, Crate> = BTreeMap::new();

    grid.lines()
        .flat_map(|line| {
            line.char_indices()
                .filter_map(|(i, c)| c.is_alphabetic().then_some((1 + i / 4, c)))
        })
        .for_each(|(i, c)| crates.entry(i).or_default().insert(0, c));

    crates
}

fn parse_moves(list: &str) -> Vec<Move> {
    list.lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn part1(mut crates: BTreeMap<usize, Crate>, moves: Vec<Move>) -> String {
    for (n, src, dst) in moves {
        for _ in 0..n {
            let c = crates.get_mut(&src).unwrap().pop().unwrap();
            crates.get_mut(&dst).unwrap().push(c);
        }
    }

    crates
        .values()
        .filter_map(|c| c.last().map(|c| c.to_string()))
        .collect_vec()
        .join("")
}

fn part2(mut crates: BTreeMap<usize, Crate>, moves: Vec<Move>) -> String {
    for (n, src, dst) in moves {
        let cutoff = crates.get(&src).unwrap().len() - n;
        let mut to_move = crates.get_mut(&src).unwrap().drain(cutoff..).collect_vec();
        crates.get_mut(&dst).unwrap().append(&mut to_move);
    }

    crates
        .values()
        .filter_map(|c| c.last().map(|c| c.to_string()))
        .collect_vec()
        .join("")
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let (crates, moves) = input
        .split_once("\n\n")
        .map(|(c, m)| (parse_crates(c), parse_moves(m)))
        .unwrap();

    println!("part 1: {}", part1(crates.clone(), moves.clone()));
    println!("part 2: {}", part2(crates, moves));

    Ok(())
}
