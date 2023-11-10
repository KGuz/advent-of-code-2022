use super::*;

struct Snafu(String);
impl Snafu {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }

    fn decode(ch: char) -> i64 {
        match ch {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => unreachable!("decode {}", ch),
        }
    }

    fn encode(val: i64) -> char {
        match val {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!("encode {}", val),
        }
    }

    fn to_decimal(&self) -> i64 {
        let chars = self.0.chars().rev().enumerate();
        chars.fold(0i64, |val, (n, c)| {
            val + Snafu::decode(c) * 5i64.pow(n as u32)
        })
    }

    fn from_decimal(mut num: i64) -> Self {
        let mut snafu = "".to_string();
        while num != 0 {
            snafu.insert(0, Snafu::encode(num.rem_euclid(5)));
            if num > 2 {
                num += 2
            }
            num /= 5
        }
        Snafu(snafu)
    }
}

pub struct Day25 {}
impl Puzzle for Day25 {
    fn part_one(&self, data: &'static str) -> String {
        let fuel_requirements = data.lines().map(Snafu::from);
        let sum: i64 = fuel_requirements.map(|snafu| snafu.to_decimal()).sum();

        Snafu::from_decimal(sum).0
    }

    fn part_two(&self, _: &'static str) -> String {
        "The End!".to_string()
    }
}
