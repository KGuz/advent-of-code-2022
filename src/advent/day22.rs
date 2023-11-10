use super::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, i32},
    multi::many1,
    IResult, Parser,
};
use pt::P2;
use std::collections::HashMap;

type WrappingFn = fn(&HashMap<P2<i32>, Tile>, P2<i32>, P2<i32>, P2<i32>) -> (P2<i32>, P2<i32>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Space,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Space,
            '#' => Tile::Wall,
            _ => Tile::Space,
        }
    }
}

struct BoardMap;
impl BoardMap {
    fn from(data: &[&str]) -> HashMap<P2<i32>, Tile> {
        let mut board = HashMap::new();
        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as i32 + 1, y as i32 + 1);
                match c {
                    '.' => board.insert(P2 { x, y }, Tile::Space),
                    '#' => board.insert(P2 { x, y }, Tile::Wall),
                    _ => continue,
                };
            }
        }
        board
    }
}

#[derive(Debug)]
enum Step {
    Move(i32),
    TurnLeft,
    TurnRight,
}
struct Steps;
impl Steps {
    fn from(data: &str) -> Vec<Step> {
        let result = many1(alt((
            i32.map(Step::Move),
            alt((
                char('L').map(|_| Step::TurnLeft),
                char('R').map(|_| Step::TurnRight),
            )),
        )))(data) as IResult<&str, Vec<Step>>;
        result.unwrap().1
    }
}

fn start_params(board: &HashMap<P2<i32>, Tile>) -> (P2<i32>, P2<i32>) {
    let x_start = board
        .iter()
        .filter(|(pt, _)| pt.y == 1)
        .map(|(pt, _)| pt.x)
        .min()
        .unwrap();

    (P2::new(x_start, 1), P2::new(1, 0))
}

fn password(pos: P2<i32>, dir: P2<i32>) -> i32 {
    let (column, row) = pos.into();
    let facing = match dir.into() {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * column + facing
}

fn simple_wrap(
    board: &HashMap<P2<i32>, Tile>,
    pos: P2<i32>,
    dir: P2<i32>,
    bounds: P2<i32>,
) -> (P2<i32>, P2<i32>) {
    let mut wrapped_pos = match dir.into() {
        (-1, _) => P2::new(bounds.x, pos.y),
        (1, _) => P2::new(1, pos.y),
        (_, 1) => P2::new(pos.x, 1),
        (_, -1) => P2::new(pos.x, bounds.y),
        _ => unreachable!(),
    };

    while board.get(&wrapped_pos).is_none() {
        wrapped_pos += dir;
    }
    if *board.get(&wrapped_pos).unwrap() == Tile::Wall {
        return (pos, dir);
    }
    (wrapped_pos, dir)
}

fn segment(pos: P2<i32>) -> i32 {
    match pos.y {
        ..=50 => match pos.x {
            ..=100 => 1,
            101.. => 2,
        },
        51..=100 => 3,
        101..=150 => match pos.x {
            ..=50 => 4,
            51.. => 5,
        },
        151.. => 6,
    }
}

#[rustfmt::skip]
fn magic_wrap(
    board: &HashMap<P2<i32>, Tile>,
    pos: P2<i32>,
    dir: P2<i32>,
    _: P2<i32>,
) -> (P2<i32>, P2<i32>) {
    let (wrapped_pos, wrapped_dir) = match segment(pos) {
        1 => match dir.into() {
            (-1, _) => ((1, 151 - pos.y), (1, 0)), // 4
            (_, -1) => ((1, 100 + pos.x), (1, 0)), // 6
            _ => unreachable!(),
        },
        2 => match dir.into() {
            (_, -1) => ((pos.x - 100, 200), (0, -1)), // 6
            (_,  1) => ((100, pos.x - 50 ), (-1, 0)), // 3
            (1,  _) => ((100, 151 - pos.y), (-1, 0)), // 5
            _ => unreachable!(),
        },
        3 => match dir.into() {
            (-1, _) => ((pos.y - 50, 101), (0, 1)),   // 4
            ( 1, _) => ((pos.y + 50, 50 ),  (0, -1)), // 2
            _ => unreachable!(),
        },
        4 => match dir.into() {
            (-1, _) => ((51, pos.y - 100), (1, 0)), // 1
            (_, -1) => ((51, pos.x + 50 ), (1, 0)), // 3
            _ => unreachable!(),
        },
        5 => match dir.into() {
            (1, _) => ((150, 151 - pos.y), (-1, 0)), // 2
            (_, 1) => ((50,  pos.x + 100), (-1, 0)), // 6
            _ => unreachable!(),
        },
        6 => match dir.into() {
            (-1, _) => ((pos.y - 100, 1  ), (0,  1)), // 1
            ( 1, _) => ((pos.y - 100, 150), (0, -1)), // 5
            (_,  1) => ((pos.x + 100, 1  ), (0,  1)), // 2
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let (wrapped_pos, wrapped_dir) = (wrapped_pos.into(), wrapped_dir.into());

    if *board.get(&wrapped_pos).unwrap() == Tile::Wall {
        return (pos, dir);
    }
    (wrapped_pos, wrapped_dir)
}

fn follow_steps(
    steps: Vec<Step>,
    board: HashMap<P2<i32>, Tile>,
    mut pos: P2<i32>,
    mut dir: P2<i32>,
    wrapping_fn: WrappingFn,
) -> (P2<i32>, P2<i32>) {
    use {Step::*, Tile::*};

    let bounds = P2 {
        x: board.keys().map(|pt| pt.x).max().unwrap(),
        y: board.keys().map(|pt| pt.y).max().unwrap(),
    };

    for step in steps {
        match step {
            Move(mut val) => {
                while val > 0 {
                    if let Some(&next_tile) = board.get(&(pos + dir)) {
                        if next_tile == Wall {
                            break;
                        }
                        pos += dir
                    } else {
                        (pos, dir) = wrapping_fn(&board, pos, dir, bounds); // after changing dir - rotation change!
                    }
                    val -= 1;
                }
            }
            #[rustfmt::skip]
            TurnLeft => {
                dir = match dir.into() {
                    // should be dependent on previous dir state as well
                    (-1,  0) => P2::new( 0,  1),
                    ( 1,  0) => P2::new( 0, -1),
                    ( 0, -1) => P2::new(-1,  0),
                    ( 0,  1) => P2::new( 1,  0),
                    _ => unreachable!(),
                };
            }
            #[rustfmt::skip]
            TurnRight => {
                dir = match dir.into() {
                    (-1,  0) => P2::new( 0, -1),
                    ( 1,  0) => P2::new( 0,  1),
                    ( 0, -1) => P2::new( 1,  0),
                    ( 0,  1) => P2::new(-1,  0),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}

pub struct Day22 {}
impl Puzzle for Day22 {
    fn part_one(&self, data: &'static str) -> String {
        let lines = data.lines().filter(|l| !l.is_empty()).collect_vec();
        let (s, b) = lines.split_last().unwrap();

        let (steps, board) = (Steps::from(s), BoardMap::from(b));
        let (start_pos, start_dir) = start_params(&board);

        let (end_pos, end_dir) = follow_steps(steps, board, start_pos, start_dir, simple_wrap);
        password(end_pos, end_dir).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let lines = data.lines().filter(|l| !l.is_empty()).collect_vec();
        let (s, b) = lines.split_last().unwrap();

        let (steps, board) = (Steps::from(s), BoardMap::from(b));
        let (start_pos, start_dir) = start_params(&board);

        let (end_pos, end_dir) = follow_steps(steps, board, start_pos, start_dir, magic_wrap);
        // assert_eq!(password, 124302)
        password(end_pos, end_dir).to_string()
    }
}
