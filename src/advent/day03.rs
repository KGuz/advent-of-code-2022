use super::*;

macro_rules! to_set {
    ($iterable: expr) => {
        std::collections::HashSet::<u8>::from_iter($iterable)
    };
}

fn priority(present: u8) -> u32 {
    (match present {
        b'a'..=b'z' => present - b'a' + 1,
        b'A'..=b'Z' => present - b'A' + 27,
        _ => unreachable!(),
    }) as u32
}

pub struct Day03 {}
impl Puzzle for Day03 {
    fn part_one(&self, data: &'static str) -> String {
        let mut answer = 0;
        for line in data.lines() {
            let (left, right) = line.split_at(line.len() / 2);
            let (left, right) = (to_set!(left.bytes()), to_set!(right.bytes()));

            let present = *left.intersection(&right).next().unwrap();
            answer += priority(present);
        }
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut answer = 0;
        let mut ls = data.lines();
        while let (Some(l1), Some(l2), Some(l3)) = (ls.next(), ls.next(), ls.next()) {
            let (l1, l2, l3) = (
                to_set!(l1.bytes()),
                to_set!(l2.bytes()),
                to_set!(l3.bytes()),
            );

            let l12 = to_set!(l1.intersection(&l2).copied());
            let present = *l12.intersection(&l3).next().unwrap();
            answer += priority(present);
        }
        answer.to_string()
    }
}
