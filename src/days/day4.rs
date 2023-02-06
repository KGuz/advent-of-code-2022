use crate::days::*;
use std::ops::Range;

fn split(line: &str, delimiter: char) -> (&str, &str) {
    let mut pair = line.split(delimiter);
    (pair.next().unwrap(), pair.next().unwrap())
}

fn parse(line: &str) -> Range<u32> {
    let (left, right) = split(line, '-');
    let range = (left.parse().unwrap(), right.parse().unwrap());
    range.0..range.1
}

fn as_ranges(line: &str) -> (Range<u32>, Range<u32>) {
    let (left, right) = split(line, ',');
    (parse(left), parse(right))
}

impl Puzzle for Day4 {
    fn part_one(&self, data: &'static str) -> String {
        let fully_overlap = |(a, b): &(Range<u32>, Range<u32>)| {
            (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
        };

        let answer = data.lines().map(as_ranges).filter(fully_overlap).count();
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let overlap = |(a, b): &(Range<u32>, Range<u32>)| {
            (a.start <= b.start && a.end >= b.start) || (b.start <= a.start && b.end >= a.start)
        };

        let answer = data.lines().map(as_ranges).filter(overlap).count();
        answer.to_string()
    }
}
