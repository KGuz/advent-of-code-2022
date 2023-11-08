use crate::days::*;
use itertools::Itertools;
use pt::P3;
use std::collections::{HashSet, VecDeque};

fn get_neighbours(cubes: &HashSet<P3<i32>>, cube: P3<i32>) -> Vec<P3<i32>> {
    #[rustfmt::skip]
    let offsets = [
        P3{x:-1, y: 0, z: 0}, P3{x: 1, y: 0, z: 0},
        P3{x: 0, y:-1, z: 0}, P3{x: 0, y: 1, z: 0},
        P3{x: 0, y: 0, z:-1}, P3{x: 0, y: 0, z: 1},
    ];

    offsets
        .into_iter()
        .map(|o| cube + o)
        .filter(|n| cubes.contains(n))
        .collect()
}

fn find_surface_area(cubes: &HashSet<P3<i32>>) -> usize {
    let mut queue: VecDeque<P3<i32>> = cubes.iter().copied().collect();
    let mut visited = set![];

    let mut surface_area = 0;
    while let Some(cur) = queue.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let neigbours = get_neighbours(cubes, cur);
        surface_area += 6 - neigbours.len();
        for next in neigbours {
            if !visited.contains(&next) {
                queue.push_back(next);
            }
        }
    }
    surface_area
}

fn find_bounds(cubes: &HashSet<P3<i32>>) -> (i32, i32) {
    let vec = Vec::from_iter(cubes.iter());
    let mut bounds = (i32::MAX, i32::MIN);

    for p in vec {
        bounds.0 = bounds.0.min(p.x).min(p.y).min(p.z);
        bounds.1 = bounds.1.max(p.x).max(p.y).max(p.z);
    }
    bounds
}

fn find_air_pockets(cubes: &HashSet<P3<i32>>) -> HashSet<P3<i32>> {
    let (lb, ub) = find_bounds(cubes);
    let (lb, ub) = (lb - 2, ub + 2);

    let grid: HashSet<P3<i32>> = (lb..ub)
        .cartesian_product(lb..ub)
        .cartesian_product(lb..ub)
        .map(|((z, y), x)| P3 { x, y, z })
        .filter(|p| !cubes.contains(p))
        .collect();

    let mut queue = queue![P3::new(lb, lb, lb)];
    let mut visited = set![];

    while let Some(cur) = queue.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let neigbours = get_neighbours(&grid, cur);
        for next in neigbours {
            if !visited.contains(&next) {
                queue.push_back(next);
            }
        }
    }

    grid.into_iter().filter(|p| !visited.contains(p)).collect()
}

fn point_from_str(s: &str) -> P3<i32> {
    let re = re!(r"(-?\d+).(-?\d+).(-?\d+)");
    let Some(caps) = re.captures(s) else { panic!(r"Pattern (-?\d+).(-?\d+).(-?\d+) not recognised in input") };
    let Some(nums) = caps.iter().skip(1).map(|cap| cap.map(|cap| cap.as_str())).collect_tuple() else {
        panic!("Number of elements in string is incorrect")
    };
    let (Some(x), Some(y), Some(z)) = nums else { panic!("Number of elements in string is incorrect") };
    let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) else { panic!("Could not parse one or more elements") };
    P3 { x, y, z }
}

impl Puzzle for Day18 {
    fn part_one(&self, data: &'static str) -> String {
        let cubes = data.lines().map(point_from_str).collect();

        let surface_area = find_surface_area(&cubes);
        surface_area.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut cubes = data.lines().map(point_from_str).collect();
        let air_pockets = find_air_pockets(&cubes);

        cubes.extend(air_pockets.iter());
        let surface_area = find_surface_area(&cubes);
        surface_area.to_string()
    }
}
