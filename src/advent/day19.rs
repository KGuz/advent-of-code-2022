use super::*;

struct Blueprint {
    id: u16,
    costs: [[u16; 4]; 4],
}
impl Blueprint {
    fn from(data: &str) -> Self {
        let re = re!(
            r"Blueprint (\d+): ",
            r"Each ore robot costs (\d+) ore. ",
            r"Each clay robot costs (\d+) ore. ",
            r"Each obsidian robot costs (\d+) ore and (\d+) clay. ",
            r"Each geode robot costs (\d+) ore and (\d+) obsidian."
        );
        let (id, orebot, claybot, obsidianbot1, obsidianbot2, geodebot1, geodebot2) =
            captures!(data, re);

        Blueprint {
            id: parse!(id),
            costs: [
                [parse!(orebot), 0, 0, 0],
                [parse!(claybot), 0, 0, 0],
                [parse!(obsidianbot1), parse!(obsidianbot2), 0, 0],
                [parse!(geodebot1), 0, parse!(geodebot2), 0],
            ],
        }
    }
}

struct Factory {
    resources: [u16; 4],
    bots: [u16; 4],
}
impl Default for Factory {
    fn default() -> Self {
        Self {
            resources: [0; 4],
            bots: [1, 0, 0, 0],
        }
    }
}
impl Factory {
    fn new(resources: [u16; 4], bots: [u16; 4]) -> Self {
        Factory { resources, bots }
    }
}

fn simulate(blueprint: &Blueprint, time_limit: u16) -> u16 {
    // calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    // it doesn't make sense to build more bots than that maximum if the resources a bot type generates are
    // enough to cover that type (ore, clay, obsidian) cost for any possible bot (per question, you can only build 1 bot per turn)
    // for geode bots, there is no logical maximum amount
    // [ore, clay, obsidian, geode]
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.costs.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes = 0;

    let mut queue = queue![(Factory::default(), 0)];

    while let Some((Factory { resources, bots }, elapsed)) = queue.pop_front() {
        // for every bot cost, run simulation
        for i in 0..4 {
            // if we already have enough of this bot type, skip
            if bots[i] == max_robots[i] {
                continue;
            }
            let costs = &blueprint.costs[i];

            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= resources[idx] => 0,
                        // no target bot type made yet
                        // we can't build it (it takes more than max_time to build it).
                        _ if bots[idx] == 0 => time_limit + 1,
                        _ => (costs[idx] - resources[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= time_limit {
                continue;
            }

            // gather ores with previously available bots
            let mut new_resources = [0; 4];
            for idx in 0..bots.len() {
                new_resources[idx] = resources[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = bots;
            new_bots[i] += 1;

            // extra optimization:
            // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
            let remaining_time = time_limit - new_elapsed;
            if ((remaining_time - 1) * remaining_time) / 2
                + new_resources[3]
                + remaining_time * new_bots[3]
                < max_geodes
            {
                continue;
            }

            queue.push_back((Factory::new(new_resources, new_bots), new_elapsed));
        }

        let geodes = resources[3] + bots[3] * (time_limit - elapsed);
        max_geodes = geodes.max(max_geodes);
    }
    max_geodes
}

pub struct Day19 {}
impl Puzzle for Day19 {
    fn part_one(&self, data: &'static str) -> String {
        let blueprints = data.lines().map(Blueprint::from);

        let quality_level: u16 = blueprints
            .map(|blueprint| simulate(&blueprint, 24) * blueprint.id)
            .sum();

        quality_level.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let blueprints = data.lines().map(Blueprint::from).take(3);

        let geodes: u16 = blueprints
            .map(|blueprint| simulate(&blueprint, 32))
            .product();

        geodes.to_string()
    }
}

// use super::*;
// use itertools::Itertools;
// use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
// use std::{fmt::Debug, time::Instant};
// 
// struct Blueprint {
//     id: u16,
//     orebot: u16,
//     claybot: u16,
//     obsidianbot: (u16, u16),
//     geodebot: (u16, u16),
// }
// 
// impl Blueprint {
//     fn from(data: &str) -> Self {
//         let re = re!(
//             r"Blueprint (\d+): ",
//             r"Each ore robot costs (\d+) ore. ",
//             r"Each clay robot costs (\d+) ore. ",
//             r"Each obsidian robot costs (\d+) ore and (\d+) clay. ",
//             r"Each geode robot costs (\d+) ore and (\d+) obsidian."
//         );
//         let (id, ore, clay, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = captures!(data, re);
// 
//         Blueprint {
//             id: parse!(id),
//             orebot: parse!(ore),
//             claybot: parse!(clay),
//             obsidianbot: (parse!(obsidian_ore), parse!(obsidian_clay)),
//             geodebot: (parse!(geode_ore), parse!(geode_obsidian)),
//         }
//     }
// }
// 
// #[derive(Clone, Copy)]
// struct Factory {
//     ore: u16,
//     clay: u16,
//     obsidian: u16,
//     geode: u16,
//     orebot: u16,
//     claybot: u16,
//     obsidianbot: u16,
//     geodebot: u16,
// }
// 
// impl Default for Factory {
//     fn default() -> Self {
//         Self { ore: 0, clay: 0, obsidian: 0, geode: 0, orebot: 1, claybot: 0, obsidianbot: 0, geodebot: 0 }
//     }
// }
// 
// impl Factory {
//     fn collect(&mut self) {
//         self.ore += self.orebot;
//         self.clay += self.claybot;
//         self.obsidian += self.obsidianbot;
//         self.geode += self.geodebot;
//     }
// 
//     fn build_orebot(mut self, blueprint: &Blueprint) -> Self {
//         self.ore -= blueprint.orebot;
//         self.collect();
//         self.orebot += 1;
//         self
//     }
//     fn build_claybot(mut self, blueprint: &Blueprint) -> Self {
//         self.ore -= blueprint.claybot;
//         self.collect();
//         self.claybot += 1;
//         self
//     }
//     fn build_obsidianbot(mut self, blueprint: &Blueprint) -> Self {
//         self.ore -= blueprint.obsidianbot.0;
//         self.clay -= blueprint.obsidianbot.1;
//         self.collect();
//         self.obsidianbot += 1;
//         self
//     }
//     fn build_geodebot(mut self, blueprint: &Blueprint) -> Self {
//         self.ore -= blueprint.geodebot.0;
//         self.obsidian -= blueprint.geodebot.1;
//         self.collect();
//         self.geodebot += 1;
//         self
//     }
// 
//     fn should_build_orebot(&self, blueprint: &Blueprint) -> bool {
//         self.ore >= blueprint.orebot && 
//         self.orebot < blueprint.claybot.max(blueprint.obsidianbot.0).max(blueprint.geodebot.0)
//     }
// 
//     fn should_build_claybot(&self, blueprint: &Blueprint) -> bool {
//         self.ore >= blueprint.claybot && 
//         self.claybot < blueprint.obsidianbot.1
//     }
// 
//     fn should_build_obsidianbot(&self, blueprint: &Blueprint) -> bool {
//         self.ore >= blueprint.obsidianbot.0 && self.clay >= blueprint.obsidianbot.1 &&
//         self.obsidianbot < blueprint.geodebot.1
//     }
// 
//     fn should_build_geode(&self, blueprint: &Blueprint) -> bool {
//         self.ore >= blueprint.geodebot.0 && self.obsidian >= blueprint.geodebot.1 
//     }
// 
//     fn possible_states(mut self, blueprint: &Blueprint) -> Vec<Self> {
//         let mut possibilities = vec![];
// 
//         if self.should_build_orebot(blueprint) {
//             possibilities.push(self.clone().build_orebot(blueprint))
//         }
//         if self.should_build_geode(blueprint) {
//             possibilities.push(self.clone().build_geodebot(blueprint))
//         } else if self.should_build_obsidianbot(blueprint) {
//             possibilities.push(self.clone().build_obsidianbot(blueprint))
//         }  else if self.should_build_claybot(blueprint) {
//             possibilities.push(self.clone().build_claybot(blueprint))
//         }
// 
//         self.collect();
//         possibilities.push(self);
// 
//         possibilities
//     }
// }
// 
// fn simulate(blueprint: &Blueprint, time_limit: u16) -> u16 {
//     let mut queue = queue![(Factory::default(), 1)];
//     let mut geodes = 0;
// 
//     while let Some((factory, time)) = queue.pop_front() {
//         geodes = geodes.max(factory.geode);
//  
//         for new_factory in factory.possible_states(blueprint) {
//             if time + 1 <= time_limit {
//                 queue.push_back((new_factory, time + 1));
//             }
//         }
//     }
//     geodes
// }
