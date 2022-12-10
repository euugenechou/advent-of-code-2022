use anyhow::Result;
use itertools::Itertools;
use std::fs;

macro_rules! tick {
    ($cycles:ident, $strength:ident, $crt:ident, $x:expr) => {
        if ($x - ($cycles % 40)).abs() <= 1 {
            $crt[($cycles / 40) as usize][($cycles % 40) as usize] = '#';
        }

        $cycles += 1;

        if $cycles != 0 && ($cycles - 20) % 40 == 0 {
            $strength += $cycles * $x;
        }
    };
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut x = 1isize;
    let mut cycles = 0;
    let mut strength = 0;
    let mut crt = [[' '; 40]; 6];

    input.lines().for_each(|line| {
        if let Some((_, val)) = line.split_once(" ") {
            tick!(cycles, strength, crt, x);
            tick!(cycles, strength, crt, x);
            x += val.parse::<isize>().unwrap();
        } else {
            tick!(cycles, strength, crt, x);
        }
    });

    println!("part 1: {strength}");
    println!(
        "part 2:\n\t{}",
        crt.iter()
            .map(|row| row.iter().collect::<String>())
            .collect_vec()
            .join("\n\t")
    );

    Ok(())
}
