use crate::{Point2d, days::*};
use std::collections::HashSet;

type Pt = Point2d<i32>;

#[derive(Debug)]
struct Move {
    dir: Pt,
    steps: i32,
}

impl Move {
    fn from(s: &str) -> Self {
        let (dir, steps) = s.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();

        let dir = match dir {
            "R" => Pt{x: 1, y: 0},
            "L" => Pt{x:-1, y: 0},
            "U" => Pt{x: 0, y: 1},
            "D" => Pt{x: 0, y:-1},
            _ => unreachable!(),
        };
        Self { dir, steps }
    }
}

fn update_tail(head: Pt, tail: Pt) -> Pt {
    match head - tail {
        Pt{x:-2, y:-2} => Pt{x:-1, y:-1},
        Pt{x:-2, y:-1} => Pt{x:-1, y:-1},
        Pt{x:-2, y: 0} => Pt{x:-1, y: 0},
        Pt{x:-2, y: 1} => Pt{x:-1, y: 1},
        Pt{x:-2, y: 2} => Pt{x:-1, y: 1},

        Pt{x:-1, y:-2} => Pt{x:-1, y:-1},
        Pt{x:-1, y: 2} => Pt{x:-1, y: 1},
        Pt{x: 0, y:-2} => Pt{x: 0, y:-1},
        Pt{x: 0, y: 2} => Pt{x: 0, y: 1},
        Pt{x: 1, y: 2} => Pt{x: 1, y: 1},
        Pt{x: 1, y:-2} => Pt{x: 1, y:-1},
        
        Pt{x: 2, y:-2} => Pt{x: 1, y:-1},
        Pt{x: 2, y:-1} => Pt{x: 1, y:-1},
        Pt{x: 2, y: 0} => Pt{x: 1, y: 0},
        Pt{x: 2, y: 1} => Pt{x: 1, y: 1},
        Pt{x: 2, y: 2} => Pt{x: 1, y: 1},
        _ => Pt{x: 0, y: 0}
    }
}

impl Puzzle for Day9 {
    fn part_one(&self, data: &'static str) -> String {
        let moves = data.lines().map(Move::from).collect::<Vec<_>>();

        let (mut head, mut tail) = (Pt::default(), Pt::default());
        let mut visited = HashSet::from([tail]);
        
        for Move{dir, steps} in moves {
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
        
        let mut rope = [Pt::default(); 10];
        let mut visited = HashSet::from([rope[9]]);

        for Move{dir, steps} in moves {
            for _ in 0..steps {
                rope[0] += dir;
                for i in 1..10 {
                    rope[i] += update_tail(rope[i-1], rope[i]);
                }
                visited.insert(rope[9]);
            }
        }

        visited.len().to_string()
    }
}