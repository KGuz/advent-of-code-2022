use crate::days::*;

type Action = Box<dyn Fn(u64) -> u64>;

#[allow(unused)]
struct Monkey {
    inspections: usize,
    index: usize,
    items: Vec<u64>,
    test_div: u64,
    inspect: Action,
    throw: Action,
}

impl Monkey {
    fn from(data: &'static str) -> Self {
        let re = re!(
            r"(?:Monkey)? (\d+):(?:\s+)",
            r"Starting items: ((?:\d+)(?:(?:,\s*)(?:\d+))*)(?:\s+)",
            r"Operation: new = old ([*+] (?:(?:\d+)|old))(?:\s+)",
            r"Test: divisible by (\d+)(?:\s+)",
            r"If true: throw to monkey (\d+)(?:\s+)",
            r"If false: throw to monkey (\d+)"
        );
        let (index, items, operation, test_div, if_ture, if_false) = captures!(data, re);

        let inspections = 0;
        let index = parse!(index);
        let items = items.split(", ").map(|x| parse!(x)).collect();

        let inspect: Action = match operation.split_once(' ').unwrap() {
            ("*", val) => Box::new(|x: u64| (x * parse!(val else x))),
            ("+", val) => Box::new(|x: u64| (x + parse!(val else x))),
            _ => unreachable!(),
        };

        let (test_div, if_true, if_false) = (parse!(test_div), parse!(if_ture), parse!(if_false));
        let throw = Box::new(move |x: u64| if x % test_div == 0 { if_true } else { if_false });

        Monkey {
            inspections,
            index,
            items,
            test_div,
            inspect,
            throw,
        }
    }

    fn inspect_items(&mut self) {
        for item in &mut self.items {
            *item = (self.inspect)(*item)
        }
        self.inspections += self.items.len();
    }

    fn throw_items(&mut self) -> Vec<(usize, u64)> {
        let monkeys = self.items.iter().fold(vec![], |mut monkeys, &item| {
            monkeys.push(((self.throw)(item) as usize, item));
            monkeys
        });
        self.items.clear();
        monkeys
    }

    fn mapv_items(&mut self, f: impl Fn(u64) -> u64) {
        self.items.iter_mut().for_each(|x| *x = f(*x));
    }
}

impl Puzzle for Day11 {
    fn part_one(&self, data: &'static str) -> String {
        let mut monkeys: Vec<_> = data.split("Monkey").skip(1).map(Monkey::from).collect();

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                monkeys[i].inspect_items();
                monkeys[i].mapv_items(|x| x / 3);

                for (j, item) in monkeys[i].throw_items() {
                    monkeys[j].items.push(item);
                }
            }
        }

        monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspections));
        let monkey_buisiness = monkeys[..2].iter().fold(1, |acc, m| acc * m.inspections);
        monkey_buisiness.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut monkeys: Vec<_> = data.split("Monkey").skip(1).map(Monkey::from).collect();
        let lcm = monkeys.iter().fold(1, |lcm, m| lcm * m.test_div);

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                monkeys[i].inspect_items();
                monkeys[i].mapv_items(|x| x % lcm);

                for (j, item) in monkeys[i].throw_items() {
                    monkeys[j].items.push(item);
                }
            }
        }

        monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspections));
        let monkey_buisiness = monkeys[..2].iter().fold(1, |acc, m| acc * m.inspections);
        monkey_buisiness.to_string()
    }
}
