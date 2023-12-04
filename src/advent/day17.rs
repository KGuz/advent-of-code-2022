use super::*;
use itertools::Itertools;
use pt::P2;
use std::collections::HashSet;

pub struct Day17 {
    /* --- Day 17: Pyroclastic Flow ---
    Your handheld device has located an alternative exit from the cave for you
    and the elephants.  The ground is rumbling almost continuously now, but the
    strange valves bought you some time. It's definitely getting warmer in
    here, though.

    The tunnels eventually open into a very tall, narrow chamber. Large,
    oddly-shaped rocks are falling into the chamber from above, presumably due
    to all the rumbling. If you can't work out where the rocks will fall next,
    you might be crushed!

    The five types of rocks have the following peculiar shapes, where # is rock
    and . is empty space:

    ####

    .#.
    ###
    .#.

    ..#
    ..#
    ###

    #
    #
    #
    #

    ##
    ##

    The rocks fall in the order shown above: first the - shape, then the +
    shape, and so on. Once the end of the list is reached, the same order
    repeats: the - shape falls first, sixth, 11th, 16th, etc.

    The rocks don't spin, but they do get pushed around by jets of hot gas
    coming out of the walls themselves. A quick scan reveals the effect the
    jets of hot gas will have on the rocks as they fall (your puzzle input).

    For example, suppose this was the jet pattern in your cave:

    >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>

    In jet patterns, < means a push to the left, while > means a push to
    the right. The pattern above means that the jets will push a falling rock
    right, then right, then right, then left, then left, then right, and so on.
    If the end of the list is reached, it repeats.

    The tall, vertical chamber is exactly seven units wide. Each rock appears
    so that its left edge is two units away from the left wall and its bottom
    edge is three units above the highest rock in the room (or the floor, if
    there isn't one).

    After a rock appears, it alternates between being pushed by a jet of hot
    gas one unit (in the direction indicated by the next symbol in the jet
    pattern) and then falling one unit down. If any movement would cause any
    part of the rock to move into the walls, floor, or a stopped rock, the
    movement instead does not occur. If a downward movement would have caused a
    falling rock to move into the floor or an already-fallen rock, the falling
    rock stops where it is (having landed on something) and a new rock
    immediately begins falling.

    Drawing falling rocks with @ and stopped rocks with #, the jet pattern in
    the example above manifests as follows:

    The first rock begins falling:
    |..@@@@.|
    |.......|
    |.......|
    |.......|
    +-------+

    Jet of gas pushes rock right:
    |...@@@@|
    |.......|
    |.......|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    |.......|
    |.......|
    +-------+

    Jet of gas pushes rock right, but nothing happens:
    |...@@@@|
    |.......|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    |.......|
    +-------+

    Jet of gas pushes rock right, but nothing happens:
    |...@@@@|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    +-------+

    Jet of gas pushes rock left:
    |..@@@@.|
    +-------+

    Rock falls 1 unit, causing it to come to rest:
    |..####.|
    +-------+

    A new rock begins falling:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock left:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |.......|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock right:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock left:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |..@....|
    |.@@@...|
    |..@....|
    |..####.|
    +-------+

    Jet of gas pushes rock right:
    |...@...|
    |..@@@..|
    |...@...|
    |..####.|
    +-------+

    Rock falls 1 unit, causing it to come to rest:
    |...#...|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    A new rock begins falling:
    |....@..|
    |....@..|
    |..@@@..|
    |.......|
    |.......|
    |.......|
    |...#...|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    The moment each of the next few rocks begins falling, you would see this:

    |..@....|
    |..@....|
    |..@....|
    |..@....|
    |.......|
    |.......|
    |.......|
    |..#....|
    |..#....|
    |####...|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@...|
    |..@@...|
    |.......|
    |.......|
    |.......|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@@@.|
    |.......|
    |.......|
    |.......|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |.......|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |....@..|
    |....@..|
    |..@@@..|
    |.......|
    |.......|
    |.......|
    |..#....|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@....|
    |..@....|
    |..@....|
    |..@....|
    |.......|
    |.......|
    |.......|
    |.....#.|
    |.....#.|
    |..####.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@...|
    |..@@...|
    |.......|
    |.......|
    |.......|
    |....#..|
    |....#..|
    |....##.|
    |....##.|
    |..####.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@@@.|
    |.......|
    |.......|
    |.......|
    |....#..|
    |....#..|
    |....##.|
    |##..##.|
    |######.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    To prove to the elephants your simulation is accurate, they want to know
    how tall the tower will get after 2022 rocks have stopped (but before the
    2023rd rock begins falling). In this example, the tower of rocks will be
    3068 units tall.

    How many units tall will the tower of rocks be after 2022 rocks have
    stopped falling?

    --- Part Two ---
    The elephants are not impressed by your simulation. They demand to know how
    tall the tower will be after 1000000000000 rocks have stopped! Only then
    will they feel confident enough to proceed through the cave.

    In the example above, the tower would be 1514285714288 units tall!

    How tall will the tower be after 1000000000000 rocks have stopped? */
}

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
