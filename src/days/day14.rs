use crate::days::*;
use itertools::{izip, Itertools};
use std::fmt::Debug;

#[derive(Debug)]
struct Rock(Vec<(u32, u32)>);
impl Rock {
    fn from(data: &str) -> Self {
        Rock(
            data.split(" -> ")
                .filter_map(|s| s.split_once(','))
                .map(|(x, y)| (parse!(x), parse!(y)))
                .collect_vec(),
        )
    }
}

struct Cave {
    scan: Vec<Vec<char>>,
    x0: usize,
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = String::with_capacity(self.scan.len() * self.scan[0].len());

        for y in 0..self.scan.len() {
            stdout.push_str(&format!("{: >3} ", y));
            for x in 0..self.scan[0].len() {
                stdout.push_str(&format!("{}", self.scan[y][x]));
            }
            stdout.push('\n');
        }
        write!(f, "\n{}", stdout)
    }
}

impl Cave {
    fn from(rocks: &[Rock]) -> Self {
        let (mut xmin, mut xmax) = (u32::MAX, u32::MIN);
        let (mut ymin, mut ymax) = (u32::MAX, u32::MIN);
        for r in rocks {
            let (r_xmin, r_xmax) = r.0.iter().map(|(x, _)| x).minmax().into_option().unwrap();
            let (r_ymin, r_ymax) = r.0.iter().map(|(_, y)| y).minmax().into_option().unwrap();
            (xmin, xmax) = (xmin.min(*r_xmin), xmax.max(*r_xmax));
            (ymin, ymax) = (ymin.min(*r_ymin), ymax.max(*r_ymax));
        }
        ymin = 0;

        let (w, h) = ((xmax - xmin) as usize + 1, (ymax - ymin) as usize + 1);
        let x0 = 500 - xmin as usize;
        let mut scan = vec![vec!['.'; w]; h];
        for r in rocks {
            for (&(x1, y1), &(x2, y2)) in izip!(&r.0, &r.0[1..]) {
                let (x1, y1) = ((x1 - xmin) as usize, (y1 - ymin) as usize);
                let (x2, y2) = ((x2 - xmin) as usize, (y2 - ymin) as usize);

                let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

                if x2 - x1 == 0 {
                    for col in &mut scan[y1..=y2] {
                        col[x1] = '#'
                    }
                } else {
                    scan[y1][x1..=x2].fill('#');
                }
            }
        }
        scan[0][x0] = '+';
        Self { scan, x0 }
    }

    fn inf_from(rocks: &[Rock]) -> Self {
        let mut cave = Self::from(rocks);
        let (h, w) = (cave.scan.len(), cave.scan[0].len());
        let pad = h - (cave.x0 - w / 2);

        for y in 0..h {
            cave.scan[y] = [&vec!['.'; pad], &cave.scan[y][..], &vec!['.'; 2 * h - pad]].concat();
        }

        cave.scan.extend([vec!['.'; w + 2 * h], vec!['#'; w + 2 * h]]);
        cave.x0 += pad;
        cave
    }

    fn move_sand(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (h, w) = (self.scan.len(), self.scan[0].len());

        if y + 1 > h - 1 {
            None
        } else if self.scan[y + 1][x] == '.' {
            Some((x, y + 1))
        } else if x < 1 {
            None
        } else if self.scan[y + 1][x - 1] == '.' {
            Some((x - 1, y + 1))
        } else if x > w - 2 {
            None
        } else if self.scan[y + 1][x + 1] == '.' {
            Some((x + 1, y + 1))
        } else if (x, y) == (self.x0, 0) {
            None
        } else {
            Some((x, y))
        }
    }

    fn drop_sand(&mut self) -> bool {
        let (mut xcur, mut ycur) = (self.x0, 0);
        while let Some((xnext, ynext)) = self.move_sand(xcur, ycur) {
            if (xcur, ycur) == (xnext, ynext) {
                self.scan[ynext][xnext] = 'o';
                return true;
            }
            (xcur, ycur) = (xnext, ynext);
        }
        false
    }

    fn simulate(&mut self) -> u32 {
        let mut sand_units = 0;
        while self.drop_sand() { sand_units += 1 }
        sand_units
    }
}

impl Puzzle for Day14 {
    fn part_one(&self, data: &'static str) -> String {
        let rocks = data.lines().map(Rock::from).collect_vec();
        let mut cave = Cave::from(&rocks);

        let units = cave.simulate();
        // println!("{:?}", cave);
        units.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let rocks = data.lines().map(Rock::from).collect_vec();
        let mut cave = Cave::inf_from(&rocks);
        
        let units = cave.simulate() + 1;
        // println!("{:?}", cave);
        units.to_string()
    }
}
