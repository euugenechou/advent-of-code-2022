use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, convert::identity, io};

type Pos = (isize, isize);

fn distance(pos1: &Pos, pos2: &Pos) -> isize {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

fn possible_beacon(pos: &Pos, sensors: &HashSet<(Pos, isize)>) -> bool {
    sensors
        .iter()
        .map(|(sensor, dist)| distance(sensor, pos) > *dist)
        .all(identity)
}

fn inbounds(pos: &Pos) -> bool {
    0 <= pos.0 && pos.0 <= 4000000 && 0 <= pos.1 && pos.1 <= 4000000
}

fn part1(sensors: &HashSet<(Pos, isize)>, beacons: &HashSet<Pos>) -> usize {
    let min_x = sensors.iter().map(|((x, _), dist)| x - dist).min().unwrap();
    let max_x = sensors.iter().map(|((x, _), dist)| x + dist).max().unwrap();
    (min_x..=max_x)
        .map(|x| (x, 2000000))
        .filter(|pos| !possible_beacon(pos, sensors) && !beacons.contains(pos))
        .count()
}

fn part2(sensors: &HashSet<(Pos, isize)>) -> isize {
    sensors
        .iter()
        .flat_map(|(sensor, dist)| {
            (0..=dist + 1).flat_map(move |dx| {
                let dy = (dist + 1) - dx;
                [(1, -1), (1, 1), (-1, 1), (-1, -1)]
                    .iter()
                    .filter_map(move |(sx, sy)| {
                        let pos = (sensor.0 + (sx * dx), sensor.1 + (sy * dy));
                        (inbounds(&pos) && possible_beacon(&pos, sensors)).then_some(pos)
                    })
            })
        })
        .map(|(x, y)| x * 4000000 + y)
        .next()
        .unwrap()
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let re = Regex::new(
        r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)",
    )?;

    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    input
        .lines()
        .filter_map(|line| re.captures(line))
        .for_each(|cap| {
            let sensor = (
                cap.name("sx").unwrap().as_str().parse::<isize>().unwrap(),
                cap.name("sy").unwrap().as_str().parse::<isize>().unwrap(),
            );
            let beacon = (
                cap.name("bx").unwrap().as_str().parse::<isize>().unwrap(),
                cap.name("by").unwrap().as_str().parse::<isize>().unwrap(),
            );
            sensors.insert((sensor, distance(&sensor, &beacon)));
            beacons.insert(beacon);
        });

    println!("part 1: {}", part1(&sensors, &beacons));
    println!("part 2: {}", part2(&sensors));

    Ok(())
}
