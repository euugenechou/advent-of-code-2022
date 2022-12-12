use anyhow::Result;
use itertools::{iproduct, Itertools};
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn inbounds(pos: (isize, isize), map: &[Vec<isize>]) -> bool {
    let x_ok = 0 <= pos.0 && pos.0 < map.len() as isize;
    let y_ok = 0 <= pos.1 && pos.1 < map[0].len() as isize;
    x_ok && y_ok
}

fn traversable(pos: (isize, isize), new_pos: (isize, isize), map: &[Vec<isize>]) -> bool {
    map[pos.0 as usize][pos.1 as usize] - map[new_pos.0 as usize][new_pos.1 as usize] >= -1
}

fn bfs(start: (isize, isize), map: &[Vec<isize>]) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((pos, moves)) = queue.pop_front() {
        if map[pos.0 as usize][pos.1 as usize] == 27 {
            return Some(moves);
        }

        for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if inbounds(new_pos, map)
                && traversable(pos, new_pos, map)
                && !visited.contains(&new_pos)
            {
                visited.insert(new_pos);
                queue.push_back((new_pos, moves + 1));
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 27,
                    _ => c as isize - 96,
                })
                .collect_vec()
        })
        .collect_vec();

    println!(
        "part 1: {}",
        iproduct!(0..map.len() as isize, 0..map[0].len() as isize)
            .filter(|(r, c)| map[*r as usize][*c as usize] == 0)
            .filter_map(|start| bfs(start, &map))
            .min()
            .unwrap()
    );

    println!(
        "part 2: {}",
        iproduct!(0..map.len() as isize, 0..map[0].len() as isize)
            .filter(|(r, c)| map[*r as usize][*c as usize] <= 1)
            .filter_map(|start| bfs(start, &map))
            .min()
            .unwrap()
    );

    Ok(())
}
