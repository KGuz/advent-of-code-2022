use crate::{
    days::*,
    point::{pt, Point2d, Point3d},
};
use std::collections::HashMap;

type Pt3 = Point3d<usize>;
type Pt2 = Point2d<usize>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    L,
    R,
    U,
    D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dirs {
    _1([Dir; 1]),
    _2([Dir; 2]),
    _3([Dir; 3]),
    _4([Dir; 4]),
}
impl Dirs {
    fn from_vec(vec: Vec<Dir>) -> Option<Self> {
        match vec.len() {
            1 => Some(Dirs::_1([vec[0]])),
            2 => Some(Dirs::_2([vec[0], vec[1]])),
            3 => Some(Dirs::_3([vec[0], vec[1], vec[2]])),
            4 => Some(Dirs::_4([vec[0], vec[1], vec[2], vec[3]])),
            _ => None,
        }
    }
    fn contains(&self, dir: Dir) -> bool {
        match self {
            Dirs::_1(arr) => arr.contains(&dir),
            Dirs::_2(arr) => arr.contains(&dir),
            Dirs::_3(arr) => arr.contains(&dir),
            Dirs::_4(arr) => arr.contains(&dir),
        }
    }
    fn push(&mut self, d: Dir) {
        *self = match self {
            Dirs::_1(a) => Dirs::_2([a[0], d]),
            Dirs::_2(a) => Dirs::_3([a[0], a[1], d]),
            Dirs::_3(a) => Dirs::_4([a[0], a[1], a[2], d]),
            Dirs::_4(_) => panic!("Cant append more directions!"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Ground,
    Blizzard(Dirs),
}

struct Valley {
    view: HashMap<Pt3, Cell>,
    dims: Pt3,
    src: Pt2,
    dst: Pt2,
}
impl Valley {
    fn from(data: &str) -> Self {
        use {Cell::*, Dir::*, Dirs::*};

        let mut view = map![];
        let mut slice = vec![];
        for (y, line) in data.lines().enumerate() {
            let mut row = vec![];
            for (x, chr) in line.char_indices() {
                let cell = match chr {
                    '#' => Wall,
                    '.' => Ground,
                    '>' => Blizzard(_1([R])),
                    '<' => Blizzard(_1([L])),
                    '^' => Blizzard(_1([U])),
                    'v' => Blizzard(_1([D])),
                    _ => continue,
                };
                view.insert(pt!(0, y, x), cell);
                row.push(cell);
            }
            slice.push(row);
        }

        let (h, w) = (slice.len(), slice[0].len());
        let src_x = slice[0].iter().position(|&c| c == Ground).unwrap();
        let dst_x = slice[h - 1].iter().position(|&c| c == Ground).unwrap();

        Self {
            view: Self::expand_in_time_domain(view, slice),
            dims: pt!((h - 2) * (w - 2), h, w),
            src: pt!(0, src_x),
            dst: pt!(h - 1, dst_x),
        }
    }

    fn expand_in_time_domain(
        mut valley: HashMap<Pt3, Cell>,
        mut slice: Vec<Vec<Cell>>,
    ) -> HashMap<Pt3, Cell> {
        use {Cell::*, Dir::*, Dirs::*};

        let (h, w) = (slice.len(), slice[0].len());
        let mut next_slice = slice.clone();

        for t in 1..(w - 2) * (h - 2) {
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let mut blizzards = vec![];

                    match slice[y][x - 1] {
                        Blizzard(dirs) if dirs.contains(R) => blizzards.push(R),
                        _ => (),
                    };
                    match slice[y][x + 1] {
                        Blizzard(dirs) if dirs.contains(L) => blizzards.push(L),
                        _ => (),
                    };
                    match slice[y - 1][x] {
                        Blizzard(dirs) if dirs.contains(D) => blizzards.push(D),
                        _ => (),
                    };
                    match slice[y + 1][x] {
                        Blizzard(dirs) if dirs.contains(U) => blizzards.push(U),
                        _ => (),
                    };

                    let next_cell = match Dirs::from_vec(blizzards) {
                        Some(dirs) => Blizzard(dirs),
                        None => Ground,
                    };

                    next_slice[y][x] = next_cell;
                }
            }

            for x in 1..w - 1 {
                match slice[h - 2][x] {
                    Blizzard(dirs) if dirs.contains(D) => match &mut next_slice[1][x] {
                        Blizzard(d) => d.push(D),
                        cell => *cell = Blizzard(_1([D])),
                    },
                    _ => (),
                }
                match slice[1][x] {
                    Blizzard(dirs) if dirs.contains(U) => match &mut next_slice[h - 2][x] {
                        Blizzard(d) => d.push(U),
                        cell => *cell = Blizzard(_1([U])),
                    },
                    _ => (),
                }
            }
            for y in 1..h - 1 {
                match slice[y][w - 2] {
                    Blizzard(dirs) if dirs.contains(R) => match &mut next_slice[y][1] {
                        Blizzard(d) => d.push(R),
                        cell => *cell = Blizzard(_1([R])),
                    },
                    _ => (),
                }
                match slice[y][1] {
                    Blizzard(dirs) if dirs.contains(L) => match &mut next_slice[y][w - 2] {
                        Blizzard(d) => d.push(L),
                        cell => *cell = Blizzard(_1([L])),
                    },
                    _ => (),
                }
            }

            for (y, row) in next_slice.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    valley.insert(pt!(t, y, x), *c);
                }
            }

            slice = next_slice.clone();
        }
        valley
    }

    #[allow(unused)]
    fn visualize(&self, time: usize) {
        use {Cell::*, Dir::*, Dirs::*};

        println!("Valley view at {}", time);
        let z = time % self.dims.z;

        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                let ch = match self.view[&pt!(z, y, x)] {
                    Wall => '#',
                    Ground => '.',
                    Blizzard(dirs) => match dirs {
                        _1(dir) => match dir[0] {
                            L => '<',
                            R => '>',
                            U => '^',
                            D => 'v',
                        },
                        _2(_) => '2',
                        _3(_) => '3',
                        _4(_) => '4',
                    },
                };
                print!("{}", ch);
            }
            println!()
        }
    }

    fn find_path(&self, src: Pt3, dst: Pt2) -> usize {
        let mut queue = queue![src];
        let mut visited = set![];

        let mut time = 0;
        while let Some(Pt3 { z, y, x }) = queue.pop_front() {
            if pt!(y, x) == dst {
                time = z;
                break;
            }

            let pos_id = pt!(z % self.dims.z, y, x);
            if visited.contains(&pos_id) {
                continue;
            }
            visited.insert(pos_id);

            for p in self.paths(pt!(z, y, x)) {
                queue.push_back(p);
            }
        }
        time
    }

    fn paths(&self, Pt3 { z, y, x }: Pt3) -> Vec<Pt3> {
        let steps = [
            pt!(z + 1, y, x),
            pt!(z + 1, y, x - 1),
            pt!(z + 1, y, x + 1),
            pt!(z + 1, y - 1, x),
            pt!(z + 1, y + 1, x),
        ];

        steps.iter().filter(|&&Pt3{z, y, x}| {
            matches!(self.view.get(&pt!(z % self.dims.z, y, x)), Some(cell) if *cell == Cell::Ground)
        }).copied().collect()
    }
}

impl Puzzle for Day24 {
    fn part_one(&self, data: &'static str) -> String {
        let valley = Valley::from(data);
        let time = valley.find_path(pt!(0, valley.src.y, valley.src.x), valley.dst);
        time.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let valley = Valley::from(data);
        let mut time = valley.find_path(pt!(0, valley.src.y, valley.src.x), valley.dst);
        time = valley.find_path(pt!(time, valley.dst.y, valley.dst.x), valley.src);
        time = valley.find_path(pt!(time, valley.src.y, valley.src.x), valley.dst);
        time.to_string()
    }
}
