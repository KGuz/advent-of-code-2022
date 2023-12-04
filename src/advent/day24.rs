use super::*;
use pt::{pt, P2, P3};
use std::collections::HashMap;

pub struct Day24 {
    /* --- Day 24: Blizzard Basin ---
    With everything replanted for next year (and with elephants and monkeys to
    tend the grove), you and the Elves leave for the extraction point.

    Partway up the mountain that shields the grove is a flat, open area that
    serves as the extraction point. It's a bit of a climb, but nothing the
    expedition can't handle.

    At least, that would normally be true; now that the mountain is covered in
    snow, things have become more difficult than the Elves are used to.

    As the expedition reaches a valley that must be traversed to reach the
    extraction site, you find that strong, turbulent winds are pushing small
    blizzards of snow and sharp ice around the valley. It's a good thing
    everyone packed warm clothes! To make it across safely, you'll need to find
    a way to avoid them.

    Fortunately, it's easy to see all of this from the entrance to the valley,
    so you make a map of the valley and the blizzards (your puzzle input). For
    example:

    #.#####
    #.....#
    #>....#
    #.....#
    #...v.#
    #.....#
    #####.#

    The walls of the valley are drawn as #; everything else is ground. Clear
    ground - where there is currently no blizzard - is drawn as .. Otherwise,
    blizzards are drawn with an arrow indicating their direction of motion: up
    (^), down (v), left (<), or right (>).

    The above map includes two blizzards, one moving right (>) and one
    moving down (v). In one minute, each blizzard moves one position in the
    direction it is pointing:

    #.#####
    #.....#
    #.>...#
    #.....#
    #.....#
    #...v.#
    #####.#

    Due to conservation of blizzard energy, as a blizzard reaches the wall of
    the valley, a new blizzard forms on the opposite side of the valley moving
    in the same direction. After another minute, the bottom downward-moving
    blizzard has been replaced with a new downward-moving blizzard at the top
    of the valley instead:

    #.#####
    #...v.#
    #..>..#
    #.....#
    #.....#
    #.....#
    #####.#

    Because blizzards are made of tiny snowflakes, they pass right through each
    other. After another minute, both blizzards temporarily occupy the same
    position, marked 2:

    #.#####
    #.....#
    #...2.#
    #.....#
    #.....#
    #.....#
    #####.#

    After another minute, the situation resolves itself, giving each blizzard
    back its personal space:

    #.#####
    #.....#
    #....>#
    #...v.#
    #.....#
    #.....#
    #####.#

    Finally, after yet another minute, the rightward-facing blizzard on the
    right is replaced with a new one on the left facing the same direction:

    #.#####
    #.....#
    #>....#
    #.....#
    #...v.#
    #.....#
    #####.#

    This process repeats at least as long as you are observing it, but probably
    forever.

    Here is a more complex example:

    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#

    Your expedition begins in the only non-wall position in the top row and
    needs to reach the only non-wall position in the bottom row. On each
    minute, you can move up, down, left, or right, or you can wait in place.
    You and the blizzards act simultaneously, and you cannot share a position
    with a blizzard.

    In the above example, the fastest way to reach your goal requires 18 steps.
    Drawing the position of the expedition as E, one way to achieve this is:

    Initial state:
    #E######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#

    Minute 1, move down:
    #.######
    #E>3.<.#
    #<..<<.#
    #>2.22.#
    #>v..^<#
    ######.#

    Minute 2, move down:
    #.######
    #.2>2..#
    #E^22^<#
    #.>2.^>#
    #.>..<.#
    ######.#

    Minute 3, wait:
    #.######
    #<^<22.#
    #E2<.2.#
    #><2>..#
    #..><..#
    ######.#

    Minute 4, move up:
    #.######
    #E<..22#
    #<<.<..#
    #<2.>>.#
    #.^22^.#
    ######.#

    Minute 5, move right:
    #.######
    #2Ev.<>#
    #<.<..<#
    #.^>^22#
    #.2..2.#
    ######.#

    Minute 6, move right:
    #.######
    #>2E<.<#
    #.2v^2<#
    #>..>2>#
    #<....>#
    ######.#

    Minute 7, move down:
    #.######
    #.22^2.#
    #<vE<2.#
    #>>v<>.#
    #>....<#
    ######.#

    Minute 8, move left:
    #.######
    #.<>2^.#
    #.E<<.<#
    #.22..>#
    #.2v^2.#
    ######.#

    Minute 9, move up:
    #.######
    #<E2>>.#
    #.<<.<.#
    #>2>2^.#
    #.v><^.#
    ######.#

    Minute 10, move right:
    #.######
    #.2E.>2#
    #<2v2^.#
    #<>.>2.#
    #..<>..#
    ######.#

    Minute 11, wait:
    #.######
    #2^E^2>#
    #<v<.^<#
    #..2.>2#
    #.<..>.#
    ######.#

    Minute 12, move down:
    #.######
    #>>.<^<#
    #.<E.<<#
    #>v.><>#
    #<^v^^>#
    ######.#

    Minute 13, move down:
    #.######
    #.>3.<.#
    #<..<<.#
    #>2E22.#
    #>v..^<#
    ######.#

    Minute 14, move right:
    #.######
    #.2>2..#
    #.^22^<#
    #.>2E^>#
    #.>..<.#
    ######.#

    Minute 15, move right:
    #.######
    #<^<22.#
    #.2<.2.#
    #><2>E.#
    #..><..#
    ######.#

    Minute 16, move right:
    #.######
    #.<..22#
    #<<.<..#
    #<2.>>E#
    #.^22^.#
    ######.#

    Minute 17, move down:
    #.######
    #2.v.<>#
    #<.<..<#
    #.^>^22#
    #.2..2E#
    ######.#

    Minute 18, move down:
    #.######
    #>2.<.<#
    #.2v^2<#
    #>..>2>#
    #<....>#
    ######E#

    What is the fewest number of minutes required to avoid the blizzards and
    reach the goal?

    --- Part Two ---
    As the expedition reaches the far side of the valley, one of the Elves
    looks especially dismayed:

    He forgot his snacks at the entrance to the valley!

    Since you're so good at dodging blizzards, the Elves humbly request that
    you go back for his snacks. From the same initial conditions, how quickly
    can you make it from the start to the goal, then back to the start, then
    back to the goal?

    In the above example, the first trip to the goal takes 18 minutes, the trip
    back to the start takes 23 minutes, and the trip back to the goal again
    takes 13 minutes, for a total time of 54 minutes.

    What is the fewest number of minutes required to reach the goal, go back to
    the start, then reach the goal again? */
}

impl Puzzle for Day24 {
    fn part_one(&self, data: &'static str) -> String {
        let valley = Valley::from(data);
        let time = valley.find_path(pt!(valley.src.x, valley.src.y, 0), valley.dst);
        time.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let valley = Valley::from(data);
        let mut time = valley.find_path(pt!(valley.src.x, valley.src.y, 0), valley.dst);
        time = valley.find_path(pt!(valley.dst.x, valley.dst.y, time), valley.src);
        time = valley.find_path(pt!(valley.src.x, valley.src.y, time), valley.dst);
        time.to_string()
    }
}

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
    view: HashMap<P3<usize>, Cell>,
    dims: P3<usize>,
    src: P2<usize>,
    dst: P2<usize>,
}
impl Valley {
    fn from(data: &str) -> Self {
        use {Cell::*, Dir::*, Dirs::*};

        let mut view = map![];
        let mut slice = vec![];
        for (y, line) in data.lines().enumerate() {
            let mut row: Vec<Cell> = vec![];
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
                view.insert(pt!(x, y, 0), cell);
                row.push(cell);
            }
            slice.push(row);
        }

        let (h, w) = (slice.len(), slice[0].len());
        let src_x = slice[0].iter().position(|&c| c == Ground).unwrap();
        let dst_x = slice[h - 1].iter().position(|&c| c == Ground).unwrap();

        Self {
            view: Self::expand_in_time_domain(view, slice),
            dims: pt!(w, h, (h - 2) * (w - 2)),
            src: pt!(src_x, 0),
            dst: pt!(dst_x, h - 1),
        }
    }

    fn expand_in_time_domain(
        mut valley: HashMap<P3<usize>, Cell>,
        mut slice: Vec<Vec<Cell>>,
    ) -> HashMap<P3<usize>, Cell> {
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
                    valley.insert(pt!(x, y, t), *c);
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
                let ch = match self.view[&pt!(x, y, z)] {
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

    fn find_path(&self, src: P3<usize>, dst: P2<usize>) -> usize {
        let mut queue = queue![src];
        let mut visited = set![];

        let mut time = 0;
        while let Some(P3 { x, y, z }) = queue.pop_front() {
            if pt!(x, y) == dst {
                time = z;
                break;
            }

            let pos_id = pt!(x, y, z % self.dims.z);
            if visited.contains(&pos_id) {
                continue;
            }
            visited.insert(pos_id);

            for p in self.paths(pt!(x, y, z)) {
                queue.push_back(p);
            }
        }
        time
    }

    fn paths(&self, P3 { x, y, z }: P3<usize>) -> Vec<P3<usize>> {
        let steps = [
            pt!(x, y, z + 1),
            pt!(x - 1, y, z + 1),
            pt!(x + 1, y, z + 1),
            pt!(x, y - 1, z + 1),
            pt!(x, y + 1, z + 1),
        ];

        steps.iter().filter(|&&P3{x, y, z}| {
            matches!(self.view.get(&pt!(x, y, z % self.dims.z)), Some(cell) if *cell == Cell::Ground)
        }).copied().collect()
    }
}
