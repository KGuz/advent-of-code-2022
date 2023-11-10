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
}
