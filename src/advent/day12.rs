use super::*;
use std::collections::{hash_map::Entry, HashMap};

pub struct Day12 {
    /* --- Day 12: Hill Climbing Algorithm ---
    You try contacting the Elves using your handheld device, but the river
    you're following must be too low to get a decent signal.

    You ask the device for a heightmap of the surrounding area (your puzzle
    input). The heightmap shows the local area from above broken into a grid;
    the elevation of each square of the grid is given by a single lowercase
    letter, where a is the lowest elevation, b is the next-lowest, and so on up
    to the highest elevation, z.

    Also included on the heightmap are marks for your current position (S) and
    the location that should get the best signal (E). Your current position (S)
    has elevation a, and the location that should get the best signal (E) has
    elevation z.

    You'd like to reach E, but to save energy, you should do it in as few steps
    as possible. During each step, you can move exactly one square up, down,
    left, or right. To avoid needing to get out your climbing gear, the
    elevation of the destination square can be at most one higher than the
    elevation of your current square; that is, if your current elevation is m,
    you could step to elevation n, but not to elevation o. (This also means
    that the elevation of the destination square can be much lower than the
    elevation of your current square.)

    For example:

    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi

    Here, you start in the top-left corner; your goal is near the middle. You
    could start by moving down or right, but eventually you'll need to head
    toward the e at the bottom. From there, you can spiral around to the goal:

    v..v<<<<
    >v.vv<<^
    .>vv>E^^
    ..v>>>^^
    ..>>>>>^

    In the above diagram, the symbols indicate whether the path exits each
    square moving up (^), down (v), left (<), or right (>). The location
    that should get the best signal is still E, and . marks unvisited squares.

    This path reaches the goal in 31 steps, the fewest possible.

    What is the fewest steps required to move from your current position to the
    location that should get the best signal?

    --- Part Two ---
    As you walk up the hill, you suspect that the Elves will want to turn this
    into a hiking trail. The beginning isn't very scenic, though; perhaps you
    can find a better starting point.

    To maximize exercise while hiking, the trail should start as low as
    possible: elevation a. The goal is still the square marked E. However, the
    trail should still be direct, taking the fewest steps to reach its goal.
    So, you'll need to find the shortest path from any square at elevation a to
    the square marked E.

    Again consider the example from above:

    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi

    Now, there are six choices for starting position (five marked a, plus the
    square marked S that counts as being at elevation a). If you start at the
    bottom-left square, you can reach the goal most quickly:
    ...v<<<<
    ...vv<<^
    ...v>E^^
    .>v>>>^^
    >^>>>>>^

    This path reaches the goal in only 29 steps, the fewest possible.

    What is the fewest steps required to move starting from any square with
    elevation a to the location that should get the best signal? */
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
                if *val == b'S' {
                    *val = b'a';
                    src = (y, x)
                }
                if *val == b'E' {
                    *val = b'z';
                    dst = (y, x)
                }
            }
        }
        Self { graph, src, dst }
    }
}

fn neighbours(graph: &Graph, p: Point) -> Vec<Point> {
    let (h, w) = (graph.len() - 1, graph[0].len() - 1);
    let mut neighbours = vec![];

    if p.0 > 0 {
        neighbours.push((p.0 - 1, p.1))
    }
    if p.0 < h {
        neighbours.push((p.0 + 1, p.1))
    }
    if p.1 > 0 {
        neighbours.push((p.0, p.1 - 1))
    }
    if p.1 < w {
        neighbours.push((p.0, p.1 + 1))
    }
    neighbours
}

fn attainable_neighbours(graph: &Graph, p: Point, f: impl Fn(u8, u8) -> bool) -> Vec<Point> {
    let neighbours = neighbours(graph, p).into_iter();
    neighbours
        .filter(|&n| f(graph[p.0][p.1], graph[n.0][n.1]))
        .collect()
}

fn bfs(graph: &Graph, src: Point, dst: Point) -> Option<Vec<Point>> {
    let mut visited = map![(src, None)];
    let mut queue = queue![src];

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
    let mut queue = queue![src];

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
