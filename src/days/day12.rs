use crate::days::*;
use std::collections::{HashMap, hash_map::Entry};

type Point = (usize, usize);
type Graph = Vec<Vec<u8>>;

struct HeightMap {
    graph: Graph,
    src: Point,
    dst: Point,
}
impl HeightMap {
    fn from(data: &str) -> Self {
        let mut graph: Graph = data.lines().map(|line| line.as_bytes().to_vec()).collect();
        let (mut src, mut dst) = ((0, 0), (0, 0));

        for (y, row) in graph.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                if *val == b'S' { *val = b'a'; src = (y, x)}
                if *val == b'E' { *val = b'z'; dst = (y, x)}
            }
        }
        Self { graph, src, dst }
    }
}

impl Puzzle for Day12 {
    fn part_one(&self, data: &'static str) -> String {
        let heightmap = HeightMap::from(data);
        let path = bfs(&heightmap.graph, heightmap.src, heightmap.dst);
        (path.unwrap().len() - 1).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let heightmap = HeightMap::from(data);
        let path = bfs_reversed(&heightmap.graph, heightmap.dst, b'a');
        (path.unwrap().len() - 1).to_string()
    }
}

fn neighbours(graph: &Graph, p: Point) -> Vec<Point> {
    let (h, w) = (graph.len() - 1, graph[0].len() - 1);
    let mut neighbours = vec![];

    if p.0 > 0 { neighbours.push((p.0 - 1, p.1)) }
    if p.0 < h { neighbours.push((p.0 + 1, p.1)) }
    if p.1 > 0 { neighbours.push((p.0, p.1 - 1)) }
    if p.1 < w { neighbours.push((p.0, p.1 + 1)) }
    neighbours
}

fn attainable_neighbours(graph: &Graph, p: Point, f: impl Fn(u8, u8) -> bool) -> Vec<Point> {
    let neighbours = neighbours(graph, p).into_iter();
    neighbours.filter(|&n| f(graph[p.0][p.1], graph[n.0][n.1])).collect()
}

fn bfs(graph: &Graph, src: Point, dst: Point) -> Option<Vec<Point>> {
    let mut visited = map![(src, None)];
    let mut queue =  queue![src];

    while let Some(current) = queue.pop_front() {
        if current == dst {
            return Some(backtrace(&visited, current));
        }

        for neighbour in attainable_neighbours(graph, current, |p, n| p >= n - 1) {
            if let Entry::Vacant(e) = visited.entry(neighbour) {
                e.insert(Some(current));
                queue.push_back(neighbour);
            }
        }
    }
    None
}

fn bfs_reversed(graph: &Graph, src: Point, dst: u8) -> Option<Vec<Point>> {
    let mut visited = map![(src, None)];
    let mut queue =  queue![src];

    while let Some(current) = queue.pop_front() {
        if graph[current.0][current.1] == dst {
            return Some(backtrace(&visited, current));
        }

        for neighbour in attainable_neighbours(graph, current, |p, n| n >= p - 1) {
            if let Entry::Vacant(e) = visited.entry(neighbour) {
                e.insert(Some(current));
                queue.push_back(neighbour);
            }
        }
    }
    None
}

fn backtrace(visited: &HashMap<Point, Option<Point>>, dst: Point) -> Vec<Point> {
    let mut backtrace = vec![dst];
    let mut current = dst;

    while let Some(Some(next)) = visited.get(&current) {
        backtrace.push(*next);
        current = *next;
    }
    backtrace.reverse();
    backtrace
}
