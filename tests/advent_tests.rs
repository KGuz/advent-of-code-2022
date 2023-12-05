use aoc::{dispatch, load_example};
use concat_idents::concat_idents;

fn extract_day(name: &str) -> usize {
    name.strip_prefix("day").unwrap().parse().unwrap()
}

macro_rules! advent_tests {
    ($($name:ident: ($expect_one:literal, $expect_two:literal),)*) => {
    $(
        concat_idents!(test_name = $name, _part_one {
            #[test]
            fn test_name() {
                let day = extract_day(stringify!($name));
                let (puzzle, input) = (dispatch(day), load_example(day));

                let result = puzzle.part_one(input);
                assert_eq!($expect_one, result, "expected: {}, result: {}", $expect_one, result);
            }
        });

        concat_idents!(test_name = $name, _part_two {
            #[test]
            fn test_name() {
                let day = extract_day(stringify!($name));
                let (puzzle, input) = (dispatch(day), load_example(day));

               let result = puzzle.part_two(input);
                assert_eq!($expect_two, result, "expected: {}, result: {}", $expect_two, result);
            }
        });
    )*
    }
}

advent_tests! {
    day01: ("24000", "45000"),
    day02: ("15", "12"),
    day03: ("157", "70"),
    day04: ("2", "4"),
    // day05: ("CMZ", "MCD"),
    day06: ("7", "19"),
    day07: ("95437", "24933642"),
    day08: ("21", "8"),
    day09: ("13", "1"),
    day10: ("13140", "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"),
    day11: ("10605", "2713310158"),
    day12: ("31", "29"),
    day13: ("13", "140"),
    day14: ("24", "93"),
    // day15: ("26", "56000011"),
    // day16: ("1651", "1707"),
    day17: ("3068", "1514285714288"),
    day18: ("64", "58"),
    // day19: ("33", "62"),
    day20: ("3", "1623178306"),
    day21: ("152", "301"),
    // day22: ("6032", "5031"),
    day23: ("110", "20"),
    day24: ("18", "54"),
    day25: ("2=-1=0", "The End!"),
}
