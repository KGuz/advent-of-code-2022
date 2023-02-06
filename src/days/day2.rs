use crate::days::*;

impl Puzzle for Day2 {
    fn part_one(&self, data: &'static str) -> String {
        let mut score = 0;
        for line in data.lines() {
            let bytes = line.as_bytes();
            let (opponent, player) = (bytes[0], bytes[2]);

            score += match (opponent, player) {
                (b'A', b'X') => 1 + 3, // rock : rock
                (b'A', b'Y') => 2 + 6, // rock : paper
                (b'A', b'Z') => 3,     // rock : scissors

                (b'B', b'X') => 1,     // paper : rock
                (b'B', b'Y') => 2 + 3, // paper : paper
                (b'B', b'Z') => 3 + 6, // paper : scissors

                (b'C', b'X') => 1 + 6, // scissors : rock
                (b'C', b'Y') => 2,     // scissors : paper
                (b'C', b'Z') => 3 + 3, // scissors : scissors
                _ => unreachable!(),
            };
        }
        score.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut score = 0;
        for line in data.lines() {
            let bytes = line.as_bytes();
            let (opponent, player) = (bytes[0], bytes[2]);

            score += match (opponent, player) {
                (b'A', b'X') => 3,     // rock : scissors
                (b'A', b'Y') => 1 + 3, // rock : rock
                (b'A', b'Z') => 2 + 6, // rock : paper

                (b'B', b'X') => 1,     // paper : rock
                (b'B', b'Y') => 2 + 3, // paper : paper
                (b'B', b'Z') => 3 + 6, // paper : scissors

                (b'C', b'X') => 2,     // scissors : paper
                (b'C', b'Y') => 3 + 3, // scissors : scissors
                (b'C', b'Z') => 1 + 6, // scissors : rock
                _ => unreachable!(),
            }
        }
        score.to_string()
    }
}
