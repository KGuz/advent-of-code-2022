use super::*;
use itertools::{izip, Itertools};
use std::fmt::Debug;

pub struct Day14 {
    /* --- Day 14: Regolith Reservoir ---
    The distress signal leads you to a giant waterfall! Actually, hang on - the
    signal seems like it's coming from the waterfall itself, and that doesn't
    make any sense. However, you do notice a little path that leads behind the
    waterfall.

    Correction: the distress signal leads you behind a giant waterfall! There
    seems to be a large cave system here, and the signal definitely leads
    further inside.

    As you begin to make your way deeper underground, you feel the ground
    rumble for a moment. Sand begins pouring into the cave! If you don't
    quickly figure out where the sand is going, you could quickly become
    trapped!

    Fortunately, your familiarity with analyzing the path of falling material
    will come in handy here. You scan a two-dimensional vertical slice of the
    cave above you (your puzzle input) and discover that it is mostly air with
    structures made of rock.

    Your scan traces the path of each solid rock structure and reports the x,y
    coordinates that form the shape of the path, where x represents distance to
    the right and y represents distance down. Each path appears as a single
    line of text in your scan. After the first point of each path, each point
    indicates the end of a straight horizontal or vertical line to be drawn
    from the previous point. For example:
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9

    This scan means that there are two paths of rock; the first path consists
    of two straight lines, and the second path consists of three straight
    lines. (Specifically, the first path consists of a line of rock from 498,4
    through 498,6 and another line of rock from 498,6 through 496,6.)

    The sand is pouring into the cave from point 500,0.

    Drawing rock as #, air as ., and the source of the sand as +, this becomes:

      4     5  5
      9     0  0
      4     0  3
    0 ......+...
    1 ..........
    2 ..........
    3 ..........
    4 ....#...##
    5 ....#...#.
    6 ..###...#.
    7 ........#.
    8 ........#.
    9 #########.

    Sand is produced one unit at a time, and the next unit of sand is not
    produced until the previous unit of sand comes to rest. A unit of sand is
    large enough to fill one tile of air in your scan.

    A unit of sand always falls down one step if possible. If the tile
    immediately below is blocked (by rock or sand), the unit of sand attempts
    to instead move diagonally one step down and to the left. If that tile is
    blocked, the unit of sand attempts to instead move diagonally one step down
    and to the right. Sand keeps moving as long as it is able to do so, at each
    step trying to move down, then down-left, then down-right. If all three
    possible destinations are blocked, the unit of sand comes to rest and no
    longer moves, at which point the next unit of sand is created back at the
    source.

    So, drawing sand that has come to rest as o, the first unit of sand simply
    falls straight down and then stops:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    ......o.#.
    #########.

    The second unit of sand then falls straight down, lands on the first one,
    and then comes to rest to its left:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    .....oo.#.
    #########.

    After a total of five units of sand have come to rest, they form this
    pattern:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ......o.#.
    ....oooo#.
    #########.

    After a total of 22 units of sand:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ....#ooo#.
    ..###ooo#.
    ....oooo#.
    ...ooooo#.
    #########.

    Finally, only two more units of sand can possibly come to rest:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ...o#ooo#.
    ..###ooo#.
    ....oooo#.
    .o.ooooo#.
    #########.

    Once all 24 units of sand shown above have come to rest, all further sand
    flows out the bottom, falling into the endless void. Just for fun, the path
    any new sand takes before falling forever is shown here with ~:

    .......+...
    .......~...
    ......~o...
    .....~ooo..
    ....~#ooo##
    ...~o#ooo#.
    ..~###ooo#.
    ..~..oooo#.
    .~o.ooooo#.
    ~#########.
    ~..........
    ~..........
    ~..........

    Using your scan, simulate the falling sand. How many units of sand come to
    rest before sand starts flowing into the abyss below?

    --- Part Two ---
    You realize you misread the scan. There isn't an endless void at the bottom
    of the scan - there's floor, and you're standing on it!

    You don't have time to scan the floor, so assume the floor is an infinite
    horizontal line with a y coordinate equal to two plus the highest y
    coordinate of any point in your scan.

    In the example above, the highest y coordinate of any point is 9, and so
    the floor is at y=11. (This is as if your scan contained one extra rock
    path like -infinity,11 -> infinity,11.) With the added floor, the
    example above now looks like this:

            ...........+........
            ....................
            ....................
            ....................
            .........#...##.....
            .........#...#......
            .......###...#......
            .............#......
            .............#......
            .....#########......
            ....................
    <-- etc #################### etc -->

    To find somewhere safe to stand, you'll need to simulate falling sand until
    a unit of sand comes to rest at 500,0, blocking the source entirely and
    stopping the flow of sand into the cave. In the example above, the
    situation finally looks like this after 93 units of sand come to rest:

    ............o............
    ...........ooo...........
    ..........ooooo..........
    .........ooooooo.........
    ........oo#ooo##o........
    .......ooo#ooo#ooo.......
    ......oo###ooo#oooo......
    .....oooo.oooo#ooooo.....
    ....oooooooooo#oooooo....
    ...ooo#########ooooooo...
    ..ooooo.......ooooooooo..
    #########################

    Using your scan, simulate the falling sand until the source of the sand
    becomes blocked. How many units of sand come to rest? */
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

        cave.scan
            .extend([vec!['.'; w + 2 * h], vec!['#'; w + 2 * h]]);
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
        while self.drop_sand() {
            sand_units += 1
        }
        sand_units
    }
}
