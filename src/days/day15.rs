use crate::{days::*, Point2d};
use itertools::Itertools;
use std::ops::Range;
type Pt = Point2d<i64>;

#[derive(Debug)]
struct Sensor {
    signal: Pt,
    beacon: Pt,
    radius: i64,
}
impl Sensor {
    fn from(data: &str) -> Self {
        let re = re!(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");
        let (sx, sy, bx, by) = captures!(data, re);

        Sensor::new(
            Pt::new(parse!(sy), parse!(sx)),
            Pt::new(parse!(by), parse!(bx)),
        )
    }
    fn new(signal: Pt, beacon: Pt) -> Self {
        let mut sensor = Self { signal, beacon, radius: 0 };
        sensor.radius = sensor.dist(sensor.beacon);
        sensor
    }
    fn dist(&self, pt: Pt) -> i64 {
        let diff = self.signal - pt;
        diff.y.abs() + diff.x.abs()
    }
    fn contains(&self, pt: Pt) -> bool {
        self.dist(pt) <= self.radius
    }
    fn skip_sensor_range(&self, pt: Pt) -> i64 {
        let dx = self.radius - (self.signal.y - pt.y).abs();
        dx + (self.signal.x - pt.x)
    }
}

fn filter_vacant(sensors: &[Sensor], range: Range<i64>, y: i64) -> Vec<Pt> {
    let mut vacancies = vec![];

    let mut x = range.start;
    while x < range.end {
        let p = Pt { y, x };
        match sensors.iter().find(|s| s.contains(p)) {
            Some(s) => x += s.skip_sensor_range(p),
            None => vacancies.push(p),
        }
        x += 1;
    }
    vacancies
}

fn find_vacant(sensors: &[Sensor], range: Range<i64>, y: i64) -> Option<Pt> {
    let mut x = range.start;
    while x < range.end {
        let p = Pt { y, x };
        match sensors.iter().find(|s| s.contains(p)) {
            Some(s) => x += s.skip_sensor_range(p),
            None => return Some(p),
        }
        x += 1;
    }
    None
}

impl Puzzle for Day15 {
    fn part_one(&self, data: &'static str) -> String {
        let sensors = data.lines().map(Sensor::from).collect_vec();
        let xmin = sensors.iter().map(|s| s.signal.x - s.radius).min().unwrap();
        let xmax = sensors.iter().map(|s| s.signal.x + s.radius).max().unwrap();

        const Y: i64 = 2_000_000;
        let vacancies = filter_vacant(&sensors, xmin..xmax + 1, Y).len() as i64;
        (xmax - xmin - vacancies).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let sensors = data.lines().map(Sensor::from).collect_vec();

        const N: i64 = 4_000_000;
        let beacon = (0..N).find_map(|y| find_vacant(&sensors, 0..N, y)).unwrap();

        (beacon.x * N + beacon.y).to_string()
    }
}
