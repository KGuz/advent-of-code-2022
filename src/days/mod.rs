pub trait Puzzle {
    fn solve(&self, data: &'static str) -> (String, String) {
        (self.part_one(data), self.part_two(data))
    }
    fn part_one(&self, data: &'static str) -> String;
    fn part_two(&self, data: &'static str) -> String;
}

macro_rules! define_days {
    (@struct $($day: ident),*) => { $( pub struct $day; )* };
    (@mod $($day: ident),*) => { $( mod $day; )* };
}

define_days!(@struct Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12,
    Day13, Day14, Day15, Day16, Day17, Day18, Day19, Day20, Day21, Day22, Day23, Day24, Day25);

define_days!(@mod day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12,
    day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25);

macro_rules! parse {
    ($s: ident as $t: ty) => {
        $s.parse::<$t>().expect("Parsing error")
    };
    ($s: expr) => {
        $s.parse().expect("Parsing error")
    };
    ($s: ident else $d: expr) => {
        $s.parse().unwrap_or($d)
    };
}
pub(crate) use parse;

// macro_rules! uuid {
//     ($s: expr) => {
//         $s.bytes().fold(0u32, |id, b| id << 8 | b as u32)
//     };
// }
// pub(crate) use uuid;

macro_rules! re {
    ($($s: literal),*) => {
        regex::Regex::new(concat![$($s),*]).unwrap()
    };
}
pub(crate) use re;

macro_rules! captures {
    ($s: expr, $re: expr) => {{
        use itertools::Itertools;
        $re.captures($s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str())
            .collect_tuple()
            .unwrap()
    }};
}
pub(crate) use captures;

macro_rules! map {
    () => { std::collections::HashMap::new() };
    ($(($k: expr, $v: expr)),*) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}
pub(crate) use map;

macro_rules! set {
    () => { std::collections::HashSet::new() };
    ($($v: expr),*) => {{
        #[allow(unused_mut)]
        let mut queue = std::collections::HashSet::new();
        $(queue.insert($v);)*
        queue
    }};
}
pub(crate) use set;

macro_rules! queue {
    () => { std::collections::VecDeque::new() };
    ($($v: expr),*) => {{
        let mut queue = std::collections::VecDeque::new();
        $(queue.push_back($v);)*
        queue
    }};
}
pub(crate) use queue;
