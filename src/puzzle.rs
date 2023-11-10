use crate::advent::*;

pub trait Puzzle {
    fn part_one(&self, data: &'static str) -> String;
    fn part_two(&self, data: &'static str) -> String;
}

pub fn dispatch(day: usize) -> Box<dyn Puzzle> {
    match day {
        1 => Box::new(day01::Day01 {}),
        2 => Box::new(day02::Day02 {}),
        3 => Box::new(day03::Day03 {}),
        4 => Box::new(day04::Day04 {}),
        5 => Box::new(day05::Day05 {}),
        6 => Box::new(day06::Day06 {}),
        7 => Box::new(day07::Day07 {}),
        8 => Box::new(day08::Day08 {}),
        9 => Box::new(day09::Day09 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        14 => Box::new(day14::Day14 {}),
        15 => Box::new(day15::Day15 {}),
        16 => Box::new(day16::Day16 {}),
        17 => Box::new(day17::Day17 {}),
        18 => Box::new(day18::Day18 {}),
        19 => Box::new(day19::Day19 {}),
        20 => Box::new(day20::Day20 {}),
        21 => Box::new(day21::Day21 {}),
        22 => Box::new(day22::Day22 {}),
        23 => Box::new(day23::Day23 {}),
        24 => Box::new(day24::Day24 {}),
        25 => Box::new(day25::Day25 {}),
        _ => unreachable!(),
    }
}
