use super::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day16 {
    /* --- Day 16: Proboscidea Volcanium ---
    The sensors have led you to the origin of the distress signal: yet another
    handheld device, just like the one the Elves gave you. However, you don't
    see any Elves around; instead, the device is surrounded by elephants! They
    must have gotten lost in these tunnels, and one of the elephants apparently
    figured out how to turn on the distress signal.

    The ground rumbles again, much stronger this time. What kind of cave is
    this, exactly? You scan the cave with your handheld device; it reports
    mostly igneous rock, some ash, pockets of pressurized gas, magma... this
    isn't just a cave, it's a volcano!

    You need to get the elephants out of here, quickly. Your device estimates
    that you have 30 minutes before the volcano erupts, so you don't have time
    to go back out the way you came in.

    You scan the cave for other options and discover a network of pipes and
    pressure-release valves. You aren't sure how such a system got into a
    volcano, but you don't have time to complain; your device produces a report
    (your puzzle input) of each valve's flow rate if it were opened (in
    pressure per minute) and the tunnels you could use to move between the
    valves.

    There's even a valve in the room you and the elephants are currently
    standing in labeled AA. You estimate it will take you one minute to open a
    single valve and one minute to follow any tunnel from one valve to another.
    What is the most pressure you could release?

    For example, suppose you had the following scan output:
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II

    All of the valves begin closed. You start at valve AA, but it must be
    damaged or jammed or something: its flow rate is 0, so there's no point in
    opening it. However, you could spend one minute moving to valve BB and
    another minute opening it; doing so would release pressure during the
    remaining 28 minutes at a flow rate of 13, a total eventual pressure
    release of 28 * 13 = 364. Then, you could spend your third minute moving to
    valve CC and your fourth minute opening it, providing an additional 26
    minutes of eventual pressure release at a flow rate of 2, or 52 total
    pressure released by valve CC.

    Making your way through the tunnels like this, you could probably open many
    or all of the valves by the time 30 minutes have elapsed. However, you need
    to release as much pressure as possible, so you'll need to be methodical.
    Instead, consider this approach:

    == Minute 1 ==
    No valves are open.
    You move to valve DD.

    == Minute 2 ==
    No valves are open.
    You open valve DD.

    == Minute 3 ==
    Valve DD is open, releasing 20 pressure.
    You move to valve CC.

    == Minute 4 ==
    Valve DD is open, releasing 20 pressure.
    You move to valve BB.

    == Minute 5 ==
    Valve DD is open, releasing 20 pressure.
    You open valve BB.

    == Minute 6 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve AA.

    == Minute 7 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve II.

    == Minute 8 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve JJ.

    == Minute 9 ==
    Valves BB and DD are open, releasing 33 pressure.
    You open valve JJ.

    == Minute 10 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve II.

    == Minute 11 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve AA.

    == Minute 12 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve DD.

    == Minute 13 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve EE.

    == Minute 14 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve FF.

    == Minute 15 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve GG.

    == Minute 16 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve HH.

    == Minute 17 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You open valve HH.

    == Minute 18 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve GG.

    == Minute 19 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve FF.

    == Minute 20 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve EE.

    == Minute 21 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You open valve EE.

    == Minute 22 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You move to valve DD.

    == Minute 23 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You move to valve CC.

    == Minute 24 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You open valve CC.

    == Minute 25 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 26 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 27 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 28 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 29 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 30 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    This approach lets you release the most pressure possible in 30 minutes
    with this valve layout, 1651.

    Work out the steps to release the most pressure in 30 minutes. What is the
    most pressure you can release?

    --- Part Two ---
    You're worried that even with an optimal approach, the pressure released
    won't be enough. What if you got one of the elephants to help you?

    It would take you 4 minutes to teach an elephant how to open the right
    valves in the right order, leaving you with only 26 minutes to actually
    execute your plan. Would having two of you working together be better, even
    if it means having less time? (Assume that you teach the elephant before
    opening any valves yourself, giving you both the same full 26 minutes.)

    In the example above, you could teach the elephant to help you as follows:

    == Minute 1 ==
    No valves are open.
    You move to valve II.
    The elephant moves to valve DD.

    == Minute 2 ==
    No valves are open.
    You move to valve JJ.
    The elephant opens valve DD.

    == Minute 3 ==
    Valve DD is open, releasing 20 pressure.
    You open valve JJ.
    The elephant moves to valve EE.

    == Minute 4 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve II.
    The elephant moves to valve FF.

    == Minute 5 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve AA.
    The elephant moves to valve GG.

    == Minute 6 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve BB.
    The elephant moves to valve HH.

    == Minute 7 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You open valve BB.
    The elephant opens valve HH.

    == Minute 8 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve CC.
    The elephant moves to valve GG.

    == Minute 9 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You open valve CC.
    The elephant moves to valve FF.

    == Minute 10 ==
    Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
    The elephant moves to valve EE.

    == Minute 11 ==
    Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
    The elephant opens valve EE.

    (At this point, all valves are open.)

    == Minute 12 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    ...

    == Minute 20 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    ...

    == Minute 26 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    With the elephant helping, after 26 minutes, the best you could do would
    release a total of 1707 pressure.

    With you and an elephant working together for 26 minutes, what is the most
    pressure you could release? */
}

impl Puzzle for Day16 {
    fn part_one(&self, data: &'static str) -> String {
        let valves = data.lines().map(Valve::from).collect_vec();
        let pressure = find_optimal_path(&valves);
        pressure.to_string()
        // let pressure = Graph::from(valves).bfs();
    }

    fn part_two(&self, data: &'static str) -> String {
        let valves = data.lines().map(Valve::from).collect_vec();
        let pressure = find_optimal_path_with_elephant(&valves);
        pressure.to_string()
    }
}

struct Valve {
    id: &'static str,
    flow_rate: u32,
    tunnels: Vec<&'static str>,
}
impl Valve {
    fn from(data: &'static str) -> Self {
        let re = re!(r"Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (\w.*)");
        let (valve_id, flow_rate, tunnels) = captures!(data, re);

        Valve {
            id: valve_id,
            flow_rate: parse!(flow_rate),
            tunnels: tunnels.split(", ").collect(),
        }
    }
}

fn build_flow_rate(valves: &[Valve]) -> HashMap<&str, u32> {
    valves.iter().map(|v| (v.id, v.flow_rate)).collect()
}

fn build_time_cost_map(valves: &[Valve]) -> HashMap<&str, HashMap<&str, u32>> {
    let mut graph = petgraph::prelude::UnGraph::new_undirected();
    let nodes: HashMap<_, _> = valves
        .iter()
        .map(|v| (v.id, graph.add_node((v.id, v.flow_rate))))
        .collect();
    let indexes: HashMap<_, _> = nodes.iter().map(|(&v, &n)| (n, v)).collect();

    for v in valves {
        for t in &v.tunnels {
            graph.update_edge(nodes[v.id], nodes[t], ());
        }
    }

    let mut time_cost_map = HashMap::new();
    for (id, node) in nodes {
        let shortest_paths = petgraph::algo::dijkstra(&graph, node, None, |_| 1u32);
        time_cost_map.insert(
            id,
            shortest_paths
                .into_iter()
                .map(|(n, dist)| (indexes[&n], dist))
                .collect(),
        );
    }
    time_cost_map
}

fn find_optimal_path(valves: &[Valve]) -> u32 {
    let flow_rate = build_flow_rate(valves);
    let time_cost_map = build_time_cost_map(valves);
    let dynamic_gain_map = |time_left: u32, visited: HashSet<&str>| {
        time_cost_map
            .iter()
            .map(|(&valve, tunnels)| {
                (
                    valve,
                    tunnels
                        .iter()
                        .map(|(&v, &c)| (v, time_left.saturating_sub(c + 1) * flow_rate[v]))
                        .filter(|(v, gain)| *gain > 0 && !visited.contains(v))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    };

    let mut queue = queue![("AA", 30, 0, set![], "".to_string())];
    let mut max_pressure = 0;

    while let Some((current, time_left, pressure, mut visited, indent)) = queue.pop_front() {
        max_pressure = max_pressure.max(pressure);
        visited.insert(current);

        let pressure_gain_map = dynamic_gain_map(time_left, visited.clone());
        for (&next, &dp) in &pressure_gain_map[current] {
            let dt = time_cost_map[current][next] + 1;

            queue.push_back((
                next,
                time_left - dt,
                pressure + dp,
                visited.clone(),
                format!("{} ", indent),
            ))
        }
    }
    max_pressure
}

fn find_optimal_path_with_elephant(valves: &[Valve]) -> u32 {
    let flow_rate = build_flow_rate(valves);
    let time_cost_map = build_time_cost_map(valves);
    let dynamic_gain_map = |time_left: u32, visited: HashSet<&str>| {
        time_cost_map
            .iter()
            .map(|(&valve, tunnels)| {
                (
                    valve,
                    tunnels
                        .iter()
                        .map(|(&v, &c)| (v, time_left.saturating_sub(c + 1) * flow_rate[v]))
                        .filter(|(v, gain)| *gain > 0 && !visited.contains(v))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    };

    let mut queue = queue![("AA", 26, 0, set![], "".to_string())];
    let mut max_pressure = 0;

    let mut global_paths = vec![];
    while let Some((current, time_left, pressure, mut visited, indent)) = queue.pop_front() {
        max_pressure = max_pressure.max(pressure);
        visited.insert(current);
        global_paths.push((visited.clone(), pressure));

        let pressure_gain_map = &dynamic_gain_map(time_left, visited.clone())[current];
        for (&next, &dp) in pressure_gain_map {
            let dt = time_cost_map[current][next] + 1;

            queue.push_back((
                next,
                time_left - dt,
                pressure + dp,
                visited.clone(),
                format!("{} ", indent),
            ))
        }
    }

    let elephant = global_paths
        .into_iter()
        .filter(|(_, p)| *p == max_pressure)
        .collect_vec()
        .pop()
        .unwrap();
    let (visited_by_elephant, elephant_pressure) = elephant;

    queue = queue![("AA", 26, 0, visited_by_elephant, "".to_string())];
    max_pressure = 0;

    let mut global_paths = vec![];
    while let Some((current, time_left, pressure, mut visited, indent)) = queue.pop_front() {
        max_pressure = max_pressure.max(pressure);
        visited.insert(current);
        global_paths.push((visited.clone(), pressure));

        let pressure_gain_map = &dynamic_gain_map(time_left, visited.clone())[current];
        for (&next, &dp) in pressure_gain_map {
            let dt = time_cost_map[current][next] + 1;
            queue.push_back((
                next,
                time_left - dt,
                pressure + dp,
                visited.clone(),
                format!("{} ", indent),
            ))
        }
    }
    elephant_pressure + max_pressure
}

// struct Node {
//     val: u32,
//     adj: Vec<usize>,
// }

// struct Graph {
//     nodes: Vec<Node>,
//     weights: Vec<Vec<u32>>,
// }

// impl Graph {
//     fn from(valves: Vec<Valve>) -> Self {
//         let nodes = Self::build_nodes(valves);
//         let weights = Self::build_weights(&nodes);
//         Self { nodes, weights }
//     }

//     fn build_nodes(mut valves: Vec<Valve>) -> Vec<Node> {
//         valves.sort_by_key(|v| v.id);
//         let uuid_map: HashMap<_, _> = valves.iter().enumerate().map(|(n, v)| (v.id, n)).collect();

//         valves.into_iter().map(|v| Node {
//             val: v.flow_rate,
//             adj: v.tunnels.into_iter().map(|t| uuid_map[t]).collect()
//         }).collect()
//     }

//     fn build_weights(nodes: &[Node]) -> Vec<Vec<u32>> {
//         use petgraph::{algo::dijkstra, prelude::UnGraph};

//         let mut graph = UnGraph::new_undirected();
//         let nidx = (0..nodes.len()).map(|n| graph.add_node(n)).collect_vec();

//         for n in 0..nodes.len() {
//             for &m in &nodes[n].adj {
//                 graph.update_edge(nidx[n], nidx[m], ());
//             }
//         }

//         let mut weights = vec![];
//         for n in 0..nodes.len() {
//             let w = dijkstra(&graph, nidx[n], None, |_| 1u32).into_iter();
//             let mut w = w.map(|(k, v)| (k.index(), v)).collect_vec();
//             w.sort_by_key(|(k, _)| *k);

//             weights.push(w.into_iter().map(|(_, v)| v).collect());
//         }
//         weights
//     }

//     fn available_neighbors(&self, node: usize, visited: &HashSet<usize>) -> Vec<usize> {
//         let all_neighbors = self.nodes[node].adj.iter().copied();
//         all_neighbors.filter(|n| !visited.contains(n)).collect()
//     }

//     fn bfs(&self) -> u32 {
//         let mut queue = queue![(0, 30, 0, set![])];
//         let mut pmax = 0;

//         while let Some((cur, time, pressure, mut visited)) = queue.pop_front() {
//             pmax = pmax.max(pressure);
//             visited.insert(cur);

//             for next in self.available_neighbors(cur, &visited) {
//                 let dt = self.weights[cur][next] + 1;
//                 let dp = self.nodes[next].val * dt;

//                 if dp > 0 && time > dt {
//                     queue.push_back((next, time - dt, pressure + dp, visited.clone()))
//                 }
//             }
//         }
//         pmax
//     }
// }
