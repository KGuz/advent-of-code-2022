use std::ops::Index;

pub struct Dataset;
impl Index<usize> for Dataset {
    type Output = str;
    fn index(&self, index: usize) -> &'static Self::Output {
        match index {
            1 => include_str!("../dataset/day1.txt"),
            2 => include_str!("../dataset/day2.txt"),
            3 => include_str!("../dataset/day3.txt"),
            4 => include_str!("../dataset/day4.txt"),
            5 => include_str!("../dataset/day5.txt"),
            6 => include_str!("../dataset/day6.txt"),
            7 => include_str!("../dataset/day7.txt"),
            8 => include_str!("../dataset/day8.txt"),
            9 => include_str!("../dataset/day9.txt"),
            10 => include_str!("../dataset/day10.txt"),
            11 => include_str!("../dataset/day11.txt"),
            12 => include_str!("../dataset/day12.txt"),
            13 => include_str!("../dataset/day13.txt"),
            14 => include_str!("../dataset/day14.txt"),
            15 => include_str!("../dataset/day15.txt"),
            16 => include_str!("../dataset/day16.txt"),
            17 => include_str!("../dataset/day17.txt"),
            18 => include_str!("../dataset/day18.txt"),
            19 => include_str!("../dataset/day19.txt"),
            20 => include_str!("../dataset/day20.txt"),
            21 => include_str!("../dataset/day21.txt"),
            22 => include_str!("../dataset/day22.txt"),
            23 => include_str!("../dataset/day23.txt"),
            24 => include_str!("../dataset/day24.txt"),
            25 => include_str!("../dataset/day25.txt"),
            _ => unreachable!(),
        }
    }
}
