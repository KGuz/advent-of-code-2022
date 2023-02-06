use crate::days::*;
use regex::{Captures, Regex};

fn containers() -> [Vec<char>; 9] {
    [
        vec!['D', 'H', 'N', 'Q', 'T', 'W', 'V', 'B'],
        vec!['D', 'W', 'B'],
        vec!['T', 'S', 'Q', 'W', 'J', 'C'],
        vec!['F', 'J', 'R', 'N', 'Z', 'T', 'P'],
        vec!['G', 'P', 'V', 'J', 'M', 'S', 'T'],
        vec!['B', 'W', 'F', 'T', 'N'],
        vec!['B', 'L', 'D', 'Q', 'F', 'H', 'V', 'N'],
        vec!['H', 'P', 'F', 'R'],
        vec!['Z', 'S', 'M', 'B', 'L', 'N', 'P', 'H'],
    ]
}

struct Op {
    count: usize,
    from: usize,
    to: usize,
}
impl Op {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
    fn from(captures: Captures) -> Self {
        let cap = |i| captures.get(i).unwrap().as_str().parse().unwrap();
        Self::new(cap(1), cap(2) - 1, cap(3) - 1)
    }
}

impl Puzzle for Day5 {
    fn part_one(&self, data: &'static str) -> String {
        let re = re!(r"move (\d+) from (\d+) to (\d+)");
        let operations = data.lines().filter_map(|l| re.captures(l)).map(Op::from);

        let mut cargo = containers();
        for op in operations {
            for _ in 0..op.count {
                let c = cargo[op.from].pop().unwrap();
                cargo[op.to].push(c);
            }
        }
        cargo.into_iter().map(|c| *c.last().unwrap()).collect()
    }

    fn part_two(&self, data: &'static str) -> String {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let operations = data.lines().filter_map(|l| re.captures(l)).map(Op::from);

        let mut cargo = containers();
        for op in operations {
            let count = cargo[op.from].len() - op.count;

            let group = cargo[op.from][count..].to_vec();
            cargo[op.from].truncate(count);
            cargo[op.to].extend(group);
        }
        cargo.into_iter().map(|c| *c.last().unwrap()).collect()
    }
}
