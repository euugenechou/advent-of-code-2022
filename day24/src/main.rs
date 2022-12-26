use anyhow::Result;
use maplit::hashset;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

type Pos = (isize, isize);

fn inbounds(pos: Pos, bounds: Pos) -> bool {
    let r_ok = 0 <= pos.0 && pos.0 < bounds.0;
    let c_ok = 0 <= pos.1 && pos.1 < bounds.1;
    r_ok && c_ok
}

fn possible_positions((r, c): Pos) -> impl Iterator<Item = Pos> {
    [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
        .into_iter()
        .map(move |(dr, dc)| (r + dr, c + dc))
}

fn blizzard_positions((r, c): Pos, time: isize, bounds: Pos) -> impl Iterator<Item = (char, Pos)> {
    [('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0)), ('v', (1, 0))]
        .iter()
        .map(move |(dir, (dr, dc))| {
            (
                *dir,
                (
                    (r - dr * time).rem_euclid(bounds.0),
                    (c - dc * time).rem_euclid(bounds.1),
                ),
            )
        })
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: isize, b: isize) -> isize {
    (a * b) / gcd(a, b)
}

fn bfs(
    start: Pos,
    end: Pos,
    time: isize,
    bounds: Pos,
    blizzards: &HashMap<char, Vec<Pos>>,
) -> isize {
    let period = lcm(bounds.0, bounds.1);
    let mut seen = hashset![(start, time)];
    let mut queue = VecDeque::from([(start, time)]);

    while let Some((pos, time)) = queue.pop_front() {
        for (nr, nc) in possible_positions(pos) {
            if (nr, nc) == end {
                return time + 1;
            }
            if (nr, nc) == start
                || inbounds((nr, nc), bounds)
                    && !blizzard_positions((nr, nc), time + 1, bounds)
                        .any(|(dir, pos)| blizzards[&dir].contains(&pos))
            {
                {
                    let state = ((nr, nc), (time + 1) % period);
                    if !seen.contains(&state) {
                        seen.insert(state);
                        queue.push_back(((nr, nc), time + 1));
                    }
                }
            }
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let blizzards = input
        .lines()
        .enumerate()
        .skip(1)
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .skip(1)
                .map(move |(c, x)| (x, (r as isize - 1, c as isize - 1)))
        })
        .filter(|(x, _)| "><^v".contains(*x))
        .fold(HashMap::new(), |mut acc, (x, pos)| {
            acc.entry(x).or_insert(Vec::new()).push(pos);
            acc
        });

    let start = (
        -1,
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|x| x == '.')
            .unwrap() as isize
            - 1,
    );

    let end = (
        input.lines().count() as isize - 2,
        input
            .lines()
            .last()
            .unwrap()
            .chars()
            .position(|x| x == '.')
            .unwrap() as isize
            - 1,
    );

    let bounds = (
        input.lines().count() as isize - 2,
        input.lines().next().unwrap().len() as isize - 2,
    );

    println!("part 1: {}", bfs(start, end, 0, bounds, &blizzards));
    println!(
        "part 2: {}",
        [(start, end), (end, start), (start, end)]
            .iter()
            .fold(0, |acc, (start, end)| bfs(
                *start, *end, acc, bounds, &blizzards
            ))
    );

    Ok(())
}
