use crate::days::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Job {
    Yell(i64),
    Add(&'static str, &'static str),
    Sub(&'static str, &'static str),
    Mul(&'static str, &'static str),
    Div(&'static str, &'static str),
    Cmp(&'static str, &'static str),
    Unknown,
}

impl Job {
    fn waits_for(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Job::Add(a, b) => Some((a, b)),
            Job::Sub(a, b) => Some((a, b)),
            Job::Mul(a, b) => Some((a, b)),
            Job::Div(a, b) => Some((a, b)),
            Job::Cmp(a, b) => Some((a, b)),
            _ => None,
        }
    }
}

struct Monkey;
impl Monkey {
    fn from(data: &'static str) -> (&'static str, Job) {
        let yell = re!(r"(\w+): (-?\d+)");
        let wait = re!(r"(\w+): (\w+) (.) (\w+)");

        if yell.is_match(data) {
            let (name, number) = captures!(data, yell);
            (name, Job::Yell(parse!(number)))
        } else {
            let (name, monkey1, sign, monkey2) = captures!(data, wait);
            match sign {
                "+" => (name, Job::Add(monkey1, monkey2)),
                "-" => (name, Job::Sub(monkey1, monkey2)),
                "*" => (name, Job::Mul(monkey1, monkey2)),
                "/" => (name, Job::Div(monkey1, monkey2)),
                _ => unreachable!(),
            }
        }
    }
}

fn solve(monkeys: &HashMap<&str, Job>, name: &str) -> i64 {
    match monkeys[name] {
        Job::Yell(num) => num,
        Job::Add(a, b) => solve(monkeys, a) + solve(monkeys, b),
        Job::Sub(a, b) => solve(monkeys, a) - solve(monkeys, b),
        Job::Mul(a, b) => solve(monkeys, a) * solve(monkeys, b),
        Job::Div(a, b) => solve(monkeys, a) / solve(monkeys, b),
        _ => unreachable!(),
    }
}

fn simplify(monkeys: &HashMap<&str, Job>, name: &str) -> Option<i64> {
    match monkeys[name] {
        Job::Add(a, b) => {
            simplify(monkeys, a).and_then(|x| simplify(monkeys, b).map(|y| x + y))
        }
        Job::Sub(a, b) => {
            simplify(monkeys, a).and_then(|x| simplify(monkeys, b).map(|y| x - y))
        }
        Job::Mul(a, b) => {
            simplify(monkeys, a).and_then(|x| simplify(monkeys, b).map(|y| x * y))
        }
        Job::Div(a, b) => {
            simplify(monkeys, a).and_then(|x| simplify(monkeys, b).map(|y| x / y))
        }
        Job::Yell(num) => Some(num),
        _ => None,
    }
}

fn solve_humn(monkeys: &HashMap<&str, Job>, mut name: &str) -> i64 {
    let mut x = 0;
    while let Some((left, right)) = monkeys[name].waits_for() {
        if let Some(val) = simplify(monkeys, left) {
            x = match monkeys[name] {
                Job::Cmp(_, _) => val,
                Job::Add(_, _) => x - val,
                Job::Sub(_, _) => val - x,
                Job::Mul(_, _) => x / val,
                Job::Div(_, _) => val / x,
                _ => unreachable!(),
            };
            name = right;
        } else if let Some(val) = simplify(monkeys, right) {
            x = match monkeys[name] {
                Job::Cmp(_, _) => val,
                Job::Add(_, _) => x - val,
                Job::Sub(_, _) => x + val,
                Job::Mul(_, _) => x / val,
                Job::Div(_, _) => x * val,
                _ => unreachable!(),
            };
            name = left;
        }
    }
    x
}

impl Puzzle for Day21 {
    fn part_one(&self, data: &'static str) -> String {
        let monkeys: HashMap<&str, Job> = data.lines().map(Monkey::from).collect();
        let answer = solve(&monkeys, "root");
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut monkeys: HashMap<&str, Job> = data.lines().map(Monkey::from).collect();

        let (a, b) = monkeys["root"].waits_for().unwrap();
        monkeys.insert("root", Job::Cmp(a, b));
        monkeys.insert("humn", Job::Unknown);

        let answer = solve_humn(&monkeys, "root");
        answer.to_string()
    }
}
