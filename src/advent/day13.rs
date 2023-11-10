use super::*;
use itertools::{izip, Itertools};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, multi::separated_list0,
    sequence::delimited, IResult, Parser,
};
use std::{cmp::Ordering, fmt::Debug};

#[derive(Clone, Eq)]
enum Packet {
    Val(u8),
    Lst(Vec<Packet>),
}
impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Val(val) => write!(f, "{}", val),
            Packet::Lst(lst) => write!(f, "{:?}", lst),
        }
    }
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Val(a), Packet::Val(b)) => a == b,
            (Packet::Lst(a), Packet::Val(b)) => a == &vec![Packet::Val(*b)],
            (Packet::Val(a), Packet::Lst(b)) => &vec![Packet::Val(*a)] == b,
            (Packet::Lst(a), Packet::Lst(b)) => a == b,
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Val(a), Packet::Val(b)) => a.partial_cmp(b),
            (Packet::Lst(a), Packet::Val(b)) => a.partial_cmp(&vec![Packet::Val(*b)]),
            (Packet::Val(a), Packet::Lst(b)) => vec![Packet::Val(*a)].partial_cmp(b),
            (Packet::Lst(a), Packet::Lst(b)) => a.partial_cmp(b),
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Packet {
    fn from(data: &str) -> Self {
        Self::parse(data).unwrap().1
    }

    fn parse(data: &str) -> IResult<&str, Packet> {
        alt((
            delimited(tag("["), separated_list0(tag(","), Self::parse), tag("]"))
                .map(Packet::Lst),
            u8.map(Packet::Val),
        ))(data)
    }
}


pub struct Day13 {}
impl Puzzle for Day13 {
    fn part_one(&self, data: &'static str) -> String {
        let packets = data.lines().filter(|l| !l.is_empty()).map(Packet::from).collect_vec();
        let pairs = izip!(&packets, &packets[1..]).step_by(2);

        let answer = pairs.enumerate().filter(|(_, (a, b))| a.cmp(b) == Ordering::Less);
        let answer = answer.map(|(n, _)| n + 1).sum::<usize>();
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut packets = data.lines().filter(|l| !l.is_empty()).map(Packet::from).collect_vec();
        let markers = [Packet::from("[[2]]"), Packet::from("[[6]]")];
        packets.extend_from_slice(&markers);
        packets.sort();

        let answer = packets.iter().enumerate().filter(|(_, p)| markers.contains(p));
        let answer = answer.map(|(n, _)| n + 1).reduce(|acc, n| acc * n).unwrap();
        answer.to_string()
    }
}
