use crate::days::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
