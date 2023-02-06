use crate::{days::*, Point3d};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

type Pt = Point3d<i32>;

fn get_neighbours(cubes: &HashSet<Pt>, cube: Pt) -> Vec<Pt> {
    #[rustfmt::skip]
    let offsets = [
        Pt{x:-1, y: 0, z: 0}, Pt{x: 1, y: 0, z: 0},
        Pt{x: 0, y:-1, z: 0}, Pt{x: 0, y: 1, z: 0},
        Pt{x: 0, y: 0, z:-1}, Pt{x: 0, y: 0, z: 1},
    ];

    offsets
        .into_iter()
        .map(|o| cube + o)
        .filter(|n| cubes.contains(n))
        .collect()
}

fn find_surface_area(cubes: &HashSet<Pt>) -> usize {
    let mut queue: VecDeque<Pt> = cubes.iter().copied().collect();
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

fn find_bounds(cubes: &HashSet<Pt>) -> (i32, i32) {
    let vec = Vec::from_iter(cubes.iter());
    let mut bounds = (i32::MAX, i32::MIN);

    for p in vec {
        bounds.0 = bounds.0.min(p.x).min(p.y).min(p.z);
        bounds.1 = bounds.1.max(p.x).max(p.y).max(p.z);
    }
    bounds
}

fn find_air_pockets(cubes: &HashSet<Pt>) -> HashSet<Pt> {
    let (lb, ub) = find_bounds(cubes);
    let (lb, ub) = (lb - 2, ub + 2);

    let grid: HashSet<Pt> = (lb..ub)
        .cartesian_product(lb..ub)
        .cartesian_product(lb..ub)
        .map(|((z, y), x)| Pt { z, y, x })
        .filter(|p| !cubes.contains(p))
        .collect();

    let mut queue = queue![Pt::new(lb, lb, lb)];
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

impl Puzzle for Day18 {
    fn part_one(&self, data: &'static str) -> String {
        let cubes = data.lines().map(|s| Pt::from_str(s).unwrap()).collect();

        let surface_area = find_surface_area(&cubes);
        surface_area.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut cubes = data.lines().map(|s| Pt::from_str(s).unwrap()).collect();
        let air_pockets = find_air_pockets(&cubes);

        cubes.extend(air_pockets.iter());
        let surface_area = find_surface_area(&cubes);
        surface_area.to_string()
    }
}
