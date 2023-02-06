use crate::{days::*, Point2d};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, i32},
    multi::many1,
    IResult, Parser,
};
use std::collections::HashMap;

type Pt = Point2d<i32>;
type WrappingFn = fn(&HashMap<Pt, Tile>, Pt, Pt, Pt) -> (Pt, Pt);

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
    fn from(data: &[&str]) -> HashMap<Pt, Tile> {
        let mut board = HashMap::new();
        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as i32 + 1, y as i32 + 1);
                match c {
                    '.' => board.insert(Pt { x, y }, Tile::Space),
                    '#' => board.insert(Pt { x, y }, Tile::Wall),
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

fn start_params(board: &HashMap<Pt, Tile>) -> (Pt, Pt) {
    let x_start = board
        .iter()
        .filter(|(pt, _)| pt.y == 1)
        .map(|(pt, _)| pt.x)
        .min()
        .unwrap();

    (Pt::new(1, x_start), Pt::new(0, 1))
}

fn password(pos: Pt, dir: Pt) -> i32 {
    let (row, column) = pos.into();
    let facing = match dir.into() {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * column + facing
}

fn simple_wrap(board: &HashMap<Pt, Tile>, pos: Pt, dir: Pt, bounds: Pt) -> (Pt, Pt) {
    let mut wrapped_pos = match dir.into() {
        (_, x) if x == -1 => Pt::new(pos.y, bounds.x),
        (_, x) if x == 1 => Pt::new(pos.y, 1),
        (y, _) if y == 1 => Pt::new(1, pos.x),
        (y, _) if y == -1 => Pt::new(bounds.y, pos.x),
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

fn segment(pos: Pt) -> i32 {
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

fn magic_wrap(board: &HashMap<Pt, Tile>, pos: Pt, dir: Pt, _: Pt) -> (Pt, Pt) {
    let (wrapped_pos, wrapped_dir) = match segment(pos) {
        1 => match dir.into() {
            (_, x) if x == -1 => ((151 - pos.y, 1), (0, 1)), // 4
            (y, _) if y == -1 => ((100 + pos.x, 1), (0, 1)), // 6
            _ => unreachable!(),
        },
        2 => match dir.into() {
            (y, _) if y == -1 => ((200, pos.x - 100), (-1, 0)), // 6
            (y, _) if y == 1 => ((pos.x - 50, 100), (0, -1)),   // 3
            (_, x) if x == 1 => ((151 - pos.y, 100), (0, -1)),  // 5
            _ => unreachable!(),
        },
        3 => match dir.into() {
            (_, x) if x == -1 => ((101, pos.y - 50), (1, 0)), // 4
            (_, x) if x == 1 => ((50, pos.y + 50), (-1, 0)),  // 2
            _ => unreachable!(),
        },
        4 => match dir.into() {
            (_, x) if x == -1 => ((pos.y - 100, 51), (0, 1)), // 1
            (y, _) if y == -1 => ((pos.x + 50, 51), (0, 1)),  // 3
            _ => unreachable!(),
        },
        5 => match dir.into() {
            (_, x) if x == 1 => ((151 - pos.y, 150), (0, -1)), // 2
            (y, _) if y == 1 => ((pos.x + 100, 50), (0, -1)),  // 6
            _ => unreachable!(),
        },
        6 => match dir.into() {
            (_, x) if x == -1 => ((1, pos.y - 100), (1, 0)), // 1
            (_, x) if x == 1 => ((150, pos.y - 100), (-1, 0)), // 5
            (y, _) if y == 1 => ((1, pos.x + 100), (1, 0)),  // 2
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
    board: HashMap<Pt, Tile>,
    mut pos: Pt,
    mut dir: Pt,
    wrapping_fn: WrappingFn,
) -> (Pt, Pt) {
    use {Step::*, Tile::*};

    let bounds = Pt {
        y: board.keys().map(|pt| pt.y).max().unwrap(),
        x: board.keys().map(|pt| pt.x).max().unwrap(),
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
            TurnLeft => {
                dir = match dir.into() {
                    // should be dependent on previous dir state as well
                    (0, -1) => Pt::new(1, 0),
                    (0, 1) => Pt::new(-1, 0),
                    (-1, 0) => Pt::new(0, -1),
                    (1, 0) => Pt::new(0, 1),
                    _ => unreachable!(),
                };
            }
            TurnRight => {
                dir = match dir.into() {
                    (0, -1) => Pt::new(-1, 0),
                    (0, 1) => Pt::new(1, 0),
                    (-1, 0) => Pt::new(0, 1),
                    (1, 0) => Pt::new(0, -1),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}

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
