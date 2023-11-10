use super::*;

fn elves(data: &'static str) -> Vec<u32> {
    let mut elves = vec![0];
    for line in data.lines() {
        match line.parse::<u32>() {
            Ok(val) => *elves.last_mut().unwrap() += val,
            Err(_) => elves.push(0),
        }
    }
    elves
}

pub struct Day01 {}
impl Puzzle for Day01 {
    fn part_one(&self, data: &'static str) -> String {
        let answer = elves(data).into_iter().max().unwrap();
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut elves = elves(data);
        elves.sort_unstable();

        let (_, top_elves) = elves.split_at(elves.len() - 3);
        let answer = top_elves.iter().sum::<u32>();
        answer.to_string()
    }
}
