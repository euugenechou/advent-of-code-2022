fn process(round: (&str, &str)) -> (usize, usize) {
    (
        (u32::from(round.0.chars().next().unwrap()) - 0x41) as usize,
        (u32::from(round.1.chars().next().unwrap()) - 0x58) as usize,
    )
}

fn score((opponent, you): (usize, usize)) -> usize {
    (1 + you) + (3 * ((4 + you - opponent) % 3))
}

fn rig((opponent, result): (usize, usize)) -> (usize, usize) {
    (opponent, (opponent + result + 2) % 3)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let rounds = input
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .map(|round| process(round))
        .collect::<Vec<_>>();

    println!(
        "part 1: {}",
        rounds.iter().map(|round| score(*round)).sum::<usize>()
    );

    println!(
        "part 2: {}",
        rounds.iter().map(|round| score(rig(*round))).sum::<usize>()
    );
}
