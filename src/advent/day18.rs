use super::*;
use itertools::Itertools;
use pt::P3;
use std::collections::{HashSet, VecDeque};

pub struct Day18 {
    /* --- Day 18: Boiling Boulders ---
    You and the elephants finally reach fresh air. You've emerged near the base
    of a large volcano that seems to be actively erupting! Fortunately, the
    lava seems to be flowing away from you and toward the ocean.

    Bits of lava are still being ejected toward you, so you're sheltering in
    the cavern exit a little longer. Outside the cave, you can see the lava
    landing in a pond and hear it loudly hissing as it solidifies.

    Depending on the specific compounds in the lava and speed at which it
    cools, it might be forming obsidian! The cooling rate should be based on
    the surface area of the lava droplets, so you take a quick scan of a
    droplet as it flies past you (your puzzle input).

    Because of how quickly the lava is moving, the scan isn't very good; its
    resolution is quite low and, as a result, it approximates the shape of the
    lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z
    position.

    To approximate the surface area, count the number of sides of each cube
    that are not immediately connected to another cube. So, if your scan were
    only two adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single
    side covered and five sides exposed, a total surface area of 10 sides.

    Here's a larger example:

    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5

    In the above example, after counting up all the sides that aren't connected
    to another cube, the total surface area is 64.

    What is the surface area of your scanned lava droplet?

    --- Part Two ---
    Something seems off about your calculation. The cooling rate depends on
    exterior surface area, but your calculation also included the surface area
    of air pockets trapped in the lava droplet.

    Instead, consider only cube sides that could be reached by the water and
    steam as the lava droplet tumbles into the pond. The steam will expand to
    reach as much as possible, completely displacing any air on the outside of
    the lava droplet but never expanding diagonally.

    In the larger example above, exactly one cube of air is trapped within the
    lava droplet (at 2,2,5), so the exterior surface area of the lava droplet
    is 58.

    What is the exterior surface area of your scanned lava droplet? */
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
    let Some(caps) = re.captures(s) else {
        panic!(r"Pattern (-?\d+).(-?\d+).(-?\d+) not recognised in input")
    };
    let Some(nums) = caps
        .iter()
        .skip(1)
        .map(|cap| cap.map(|cap| cap.as_str()))
        .collect_tuple()
    else {
        panic!("Number of elements in string is incorrect")
    };
    let (Some(x), Some(y), Some(z)) = nums else {
        panic!("Number of elements in string is incorrect")
    };
    let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) else {
        panic!("Could not parse one or more elements")
    };
    P3 { x, y, z }
}
