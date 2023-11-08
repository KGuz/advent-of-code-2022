use crate::days::*;
use itertools::all;
use pt::{pt, P2};
use std::{collections::HashSet, hash::Hash};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Elf(P2<i32>);
impl Elf {
    fn neighbours(&self, dir: char) -> [P2<i32>; 3] {
        match dir {
            'N' => [pt!(-1, -1), pt!(0, -1), pt!(1, -1)],
            'S' => [pt!(-1, 1), pt!(0, 1), pt!(1, 1)],
            'W' => [pt!(-1, -1), pt!(-1, 0), pt!(-1, 1)],
            'E' => [pt!(1, -1), pt!(1, 0), pt!(1, 1)],
            _ => unreachable!(),
        }
        .map(|p| self.0 + p)
    }

    fn propose_move(
        &self,
        elves: &HashSet<Elf>,
        cycle: impl Iterator<Item = char>,
    ) -> Option<P2<i32>> {
        let mut moves = vec![];
        for neighbours in cycle.map(|dir| self.neighbours(dir)) {
            if all(neighbours, |p| !elves.contains(&Elf(p))) {
                moves.push(neighbours[1])
            }
        }
        match moves.len() {
            0 | 4 => None,
            _ => Some(moves[0]),
        }
    }
}

struct Elves;
impl Elves {
    fn from(data: &str) -> HashSet<Elf> {
        let mut elves = set![];
        for (y, line) in data.lines().enumerate() {
            for (x, _) in line.char_indices().filter(|&(_, c)| c == '#') {
                let pos = pt!(x as i32, y as i32);
                elves.insert(Elf(pos));
            }
        }
        elves
    }
    fn bounding_box(elves: &HashSet<Elf>) -> (P2<i32>, P2<i32>) {
        let mut min = pt!(i32::MAX, i32::MAX);
        let mut max = pt!(i32::MIN, i32::MIN);

        for Elf(pos) in elves {
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
        }
        (min, max)
    }
}

fn remove_collisions(moves: Vec<(Elf, P2<i32>)>) -> Vec<(Elf, P2<i32>)> {
    let mut visited = map![];
    for &(_, new_pos) in &moves {
        *visited.entry(new_pos).or_insert(0) += 1;
    }

    moves
        .into_iter()
        .filter(|(_, new_pos)| visited[new_pos] == 1)
        .collect()
}

impl Puzzle for Day23 {
    fn part_one(&self, data: &'static str) -> String {
        let mut elves = Elves::from(data);
        let directions = ['N', 'S', 'W', 'E'].into_iter().cycle();

        for n in 0..10 {
            let mut moves = vec![];
            for elf in &elves {
                let cycle = directions.clone().skip(n).take(4);
                if let Some(new_pos) = elf.propose_move(&elves, cycle) {
                    moves.push((*elf, new_pos));
                }
            }

            for (elf, new_pos) in remove_collisions(moves) {
                elves.remove(&elf);
                elves.insert(Elf(new_pos));
            }
        }

        let (min, max) = Elves::bounding_box(&elves);
        let area = (1 + max.y - min.y) * (1 + max.x - min.x);
        (area - elves.len() as i32).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut elves = Elves::from(data);
        let directions = ['N', 'S', 'W', 'E'].into_iter().cycle();

        for n in 0.. {
            let mut moves = vec![];
            for elf in &elves {
                let cycle = directions.clone().skip(n).take(4);
                if let Some(new_pos) = elf.propose_move(&elves, cycle) {
                    moves.push((*elf, new_pos));
                }
            }
            if moves.is_empty() {
                return (n + 1).to_string();
            }

            for (elf, new_pos) in remove_collisions(moves) {
                elves.remove(&elf);
                elves.insert(Elf(new_pos));
            }
        }
        unreachable!()
    }
}
