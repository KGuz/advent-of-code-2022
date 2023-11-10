use super::*;
use itertools::Itertools;

fn mix(nums: &[(usize, i64)], carry: Option<Vec<(usize, i64)>>) -> Vec<(usize, i64)> {
    let mut state = carry.unwrap_or_else(|| nums.to_vec());

    for &(id, _) in nums.iter() {
        let idx = state.iter().position(|&s| s.0 == id).unwrap();
        let cur = state.remove(idx);

        let new_idx = (idx as i64 + cur.1).rem_euclid(state.len() as i64);
        state.insert(new_idx as usize, cur);
    }
    state
}

fn find_grove(nums: &[(usize, i64)]) -> i64 {
    let zero = nums.iter().position(|s| s.1 == 0).unwrap();

    nums[(1000 + zero) % nums.len()].1
        + nums[(2000 + zero) % nums.len()].1
        + nums[(3000 + zero) % nums.len()].1
}

pub struct Day20 {}
impl Puzzle for Day20 {
    fn part_one(&self, data: &'static str) -> String {
        let nums = data.lines().enumerate().map(|(n, s)| (n, parse!(s as i64))).collect_vec();
        let decrypted = mix(&nums, None);
        find_grove(&decrypted).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let nums = data.lines().enumerate().map(|(n, s)| (n, 811589153 * parse!(s as i64))).collect_vec();

        let mut decrypted = mix(&nums, None);
        for _ in 0..9 {
            decrypted = mix(&nums, Some(decrypted));
        }
        find_grove(&decrypted).to_string()
    }
}
