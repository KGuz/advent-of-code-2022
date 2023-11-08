use aoc::{AdventOfCode, Dataset};
use clap::Parser;

/// Solver of 2022 Advent of Code Puzzles
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Advent day number (1..=25)
    day: usize,
}

fn main() {
    let day = Args::parse().day;
    assert!(
        (1..=25).contains(&day),
        "Advent day number out of range (1..=25)"
    );

    let (part_one, part_two) = AdventOfCode[day].solve(&Dataset[day]);
    println!("{:*^60}", format!(" Advent of Code 2022 - Day {} ", day));
    println!("Part one {:.>51}", format!(" {}", part_one));
    println!("Part two {:.>51}", format!(" {}", part_two));
}
