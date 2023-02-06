use crate::days::*;

impl Puzzle for Day8 {
    fn part_one(&self, data: &'static str) -> String {
        let grid = data.lines().fold(vec![], |mut grid, line| {
            grid.push(line.bytes().map(|x| x - b'0').collect::<Vec<_>>());
            grid
        });

        let (h, w) = (grid.len(), grid[0].len());
        let mut local_maxs = vec![vec![[0u8, 0, 0, 0]; w]; h];

        for y in 1..h - 1 {
            let mut left_max = grid[y][0];
            for x in 1..w - 1 {
                local_maxs[y][x][0] = left_max;
                left_max = left_max.max(grid[y][x]);
            }

            let mut right_max = grid[y][w - 1];
            for x in (1..w - 1).rev() {
                local_maxs[y][x][1] = right_max;
                right_max = right_max.max(grid[y][x]);
            }
        }

        for x in 1..w - 1 {
            let mut up_max = grid[0][x];
            for y in 1..h - 1 {
                local_maxs[y][x][2] = up_max;
                up_max = up_max.max(grid[y][x]);
            }

            let mut down_max = grid[h - 1][x];
            for y in (1..h - 1).rev() {
                local_maxs[y][x][3] = down_max;
                down_max = down_max.max(grid[y][x]);
            }
        }

        let mut answer = 2 * (w + h) - 4;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let t = grid[y][x];
                if local_maxs[y][x].iter().any(|&x| t > x) {
                    answer += 1;
                }
            }
        }
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let grid = data.lines().fold(vec![], |mut grid, line| {
            grid.push(line.bytes().map(|x| x - b'0').collect::<Vec<_>>());
            grid
        });

        let (h, w) = (grid.len(), grid[0].len());
        let mut score = vec![vec![[0, 0, 0, 0]; w]; h];

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let v = grid[y][x];

                let mut lx = x - 1;
                while lx > 0 && grid[y][lx] < v { lx -= 1 }
                score[y][x][0] = x - lx;

                let mut rx = x + 1;
                while rx < w - 1 && grid[y][rx] < v { rx += 1 }
                score[y][x][1] = rx - x;

                let mut uy = y - 1;
                while uy > 0 && grid[uy][x] < v { uy -= 1 }
                score[y][x][2] = y - uy;

                let mut dy = y + 1;
                while dy < h - 1 && grid[dy][x] < v { dy += 1 }
                score[y][x][3] = dy - y;
            }
        }

        let scenic_score = score.iter().flatten().map(|[l, r, u, d]| l * r * u * d);
        let answer = scenic_score.max().unwrap();
        answer.to_string()
    }
}
