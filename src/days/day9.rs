use crate::days::*;
use pt::P2;
use std::collections::HashSet;

#[derive(Debug)]
struct Move {
    dir: P2<i32>,
    steps: i32,
}

impl Move {
    #[rustfmt::skip]
    fn from(s: &str) -> Self {
        let (dir, steps) = s.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();

        let dir = match dir {
            "R" => P2 { x:  1, y:  0 },
            "L" => P2 { x: -1, y:  0 },
            "U" => P2 { x:  0, y:  1 },
            "D" => P2 { x:  0, y: -1 },
            _ => unreachable!(),
        };
        Self { dir, steps }
    }
}

#[rustfmt::skip]
fn update_tail(head: P2<i32>, tail: P2<i32>) -> P2<i32> {
    match head - tail {
        P2 { x: -2, y: -2 } => P2 { x: -1, y: -1 },
        P2 { x: -2, y: -1 } => P2 { x: -1, y: -1 },
        P2 { x: -2, y:  0 } => P2 { x: -1, y:  0 },
        P2 { x: -2, y:  1 } => P2 { x: -1, y:  1 },
        P2 { x: -2, y:  2 } => P2 { x: -1, y:  1 },

        P2 { x: -1, y: -2 } => P2 { x: -1, y: -1 },
        P2 { x: -1, y:  2 } => P2 { x: -1, y:  1 },
        P2 { x: 0,  y: -2 } => P2 { x:  0, y: -1 },
        P2 { x: 0,  y:  2 } => P2 { x:  0, y:  1 },
        P2 { x: 1,  y:  2 } => P2 { x:  1, y:  1 },
        P2 { x: 1,  y: -2 } => P2 { x:  1, y: -1 },

        P2 { x: 2, y: -2 } => P2 { x: 1, y: -1 },
        P2 { x: 2, y: -1 } => P2 { x: 1, y: -1 },
        P2 { x: 2, y:  0 } => P2 { x: 1, y:  0 },
        P2 { x: 2, y:  1 } => P2 { x: 1, y:  1 },
        P2 { x: 2, y:  2 } => P2 { x: 1, y:  1 },
        _ => P2 { x: 0, y: 0 },
    }
}

impl Puzzle for Day9 {
    fn part_one(&self, data: &'static str) -> String {
        let moves = data.lines().map(Move::from).collect::<Vec<_>>();

        let (mut head, mut tail) = (P2::default(), P2::default());
        let mut visited = HashSet::from([tail]);

        for Move { dir, steps } in moves {
            for _ in 0..steps {
                head += dir;
                tail += update_tail(head, tail);
                visited.insert(tail);
            }
        }

        visited.len().to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let moves = data.lines().map(Move::from).collect::<Vec<_>>();

        let mut rope = [P2::default(); 10];
        let mut visited = HashSet::from([rope[9]]);

        for Move { dir, steps } in moves {
            for _ in 0..steps {
                rope[0] += dir;
                for i in 1..10 {
                    rope[i] += update_tail(rope[i - 1], rope[i]);
                }
                visited.insert(rope[9]);
            }
        }

        visited.len().to_string()
    }
}
