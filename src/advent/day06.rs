use super::*;
use std::collections::{HashSet, VecDeque};

pub struct Day06 {}
impl Puzzle for Day06 {
    fn part_one(&self, data: &'static str) -> String {
        let bytes = data.as_bytes();

        let is_unique = |queue: &VecDeque<u8>| HashSet::<&u8>::from_iter(queue.iter()).len() == 4;
        let mut queue = VecDeque::from_iter(bytes[..4].iter().copied());

        for (n, c) in bytes.iter().enumerate() {
            if is_unique(&queue) {
                return n.to_string();
            } else {
                queue.push_back(*c);
                queue.pop_front();
            }
        }
        data.len().to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let bytes = data.as_bytes();

        let is_unique = |queue: &VecDeque<u8>| HashSet::<&u8>::from_iter(queue.iter()).len() == 14;
        let mut queue = VecDeque::from_iter(bytes[..14].iter().copied());
        for (n, c) in bytes.iter().enumerate() {
            if is_unique(&queue) {
                return n.to_string();
            } else {
                queue.push_back(*c);
                queue.pop_front();
            }
        }
        data.len().to_string()
    }
}
