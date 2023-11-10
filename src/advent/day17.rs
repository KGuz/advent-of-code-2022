use super::*;
use itertools::Itertools;
use pt::P2;
use std::collections::HashSet;

struct Input {
    len: usize,
    current: usize,
    input: Vec<i64>,
}
impl Input {
    fn from(data: &'static str) -> Self {
        let input = data
            .chars()
            .map(|c| if c == '<' { -1 } else { 1 })
            .collect_vec();
        Self {
            len: input.len(),
            current: 0,
            input,
        }
    }
    fn next(&mut self) -> i64 {
        let next = self.input[self.current];
        self.current = (self.current + 1) % self.len;
        next
    }
}

#[derive(Clone)]
struct Shape {
    shape: Vec<P2<i64>>,
    position: P2<i64>,
    bounds: P2<i64>,
}
impl Shape {
    fn positions(&self) -> impl Iterator<Item = P2<i64>> + '_ {
        self.shape.iter().map(|&p| p + self.position)
    }

    fn is_colliding(&self, points: &HashSet<P2<i64>>) -> bool {
        self.positions().any(|s| points.contains(&s))
    }
}
struct Shapes {
    shapes: [Shape; 5],
    current: usize,
}
impl Default for Shapes {
    fn default() -> Self {
        Self {
            shapes: [
                Shape {
                    shape: vec![
                        P2 { x: 0, y: 0 },
                        P2 { x: 1, y: 0 },
                        P2 { x: 2, y: 0 },
                        P2 { x: 3, y: 0 },
                    ],
                    position: P2 { x: 3, y: 0 },
                    bounds: P2 { x: 3, y: 0 },
                },
                Shape {
                    shape: vec![
                        P2 { x: 1, y: 0 },
                        P2 { x: 0, y: 1 },
                        P2 { x: 1, y: 1 },
                        P2 { x: 2, y: 1 },
                        P2 { x: 1, y: 2 },
                    ],
                    position: P2 { x: 3, y: 0 },
                    bounds: P2 { x: 2, y: 2 },
                },
                Shape {
                    shape: vec![
                        P2 { x: 0, y: 0 },
                        P2 { x: 1, y: 0 },
                        P2 { x: 2, y: 0 },
                        P2 { x: 2, y: 1 },
                        P2 { x: 2, y: 2 },
                    ],
                    position: P2 { x: 3, y: 0 },
                    bounds: P2 { x: 2, y: 2 },
                },
                Shape {
                    shape: vec![
                        P2 { x: 0, y: 0 },
                        P2 { x: 0, y: 1 },
                        P2 { x: 0, y: 2 },
                        P2 { x: 0, y: 3 },
                    ],
                    position: P2 { x: 3, y: 0 },
                    bounds: P2 { x: 0, y: 3 },
                },
                Shape {
                    shape: vec![
                        P2 { x: 0, y: 0 },
                        P2 { x: 1, y: 0 },
                        P2 { x: 0, y: 1 },
                        P2 { x: 1, y: 1 },
                    ],
                    position: P2 { x: 3, y: 0 },
                    bounds: P2 { x: 1, y: 1 },
                },
            ],
            current: 0,
        }
    }
}
impl Shapes {
    fn next(&mut self) -> Shape {
        let next = self.shapes[self.current].clone();
        self.current = (self.current + 1) % 5;
        next
    }
}

struct Tetris;
impl Tetris {
    fn simulate(mut input: Input, blocks: i64) -> i64 {
        let mut shapes = Shapes::default();
        let bounds = (1, 7);

        let mut h = 0;
        let mut stack = set![];
        let mut unique = map![];
        let mut skip = None;

        let mut b = 0;
        while b < blocks {
            let mut fall_height = 0;
            let mut shape = shapes.next();
            shape.position.y = h + 4;

            loop {
                let (dx, dy) = (input.next(), -1);

                shape.position.x += dx;
                let x_pos = shape.position.x + if dx < 0 { 0 } else { shape.bounds.x };
                if x_pos < bounds.0 || x_pos > bounds.1 || shape.is_colliding(&stack) {
                    shape.position.x -= dx;
                }

                shape.position.y += dy;
                fall_height += 1;
                if shape.position.y < 1 || shape.is_colliding(&stack) {
                    shape.position.y -= dy;
                    fall_height -= 1;
                    break;
                }
            }

            stack.extend(shape.positions());
            h = h.max(shape.bounds.y + shape.position.y);
            b += 1;

            if skip.is_none() {
                if unique.contains_key(&(input.current, shapes.current, fall_height)) {
                    let (last_b, last_h) = unique[&(input.current, shapes.current, fall_height)];
                    let (db, dh) = ((b - last_b), (h - last_h));
                    let cycles = (blocks - b) / db;

                    b += db * cycles;
                    skip = Some(dh * cycles);
                }
                unique.insert((input.current, shapes.current, fall_height), (b, h));
            }
        }
        h + skip.unwrap()
    }
}

pub struct Day17 {}
impl Puzzle for Day17 {
    fn part_one(&self, data: &'static str) -> String {
        let input = Input::from(data);
        let peak = Tetris::simulate(input, 2022);
        peak.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let input = Input::from(data);
        let peak = Tetris::simulate(input, 1000000000000);
        peak.to_string()
    }
}
