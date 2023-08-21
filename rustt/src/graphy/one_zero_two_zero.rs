/*
 * @lc app=leetcode.cn id=1020 lang=rust
 *
 * [1020] 飞地的数量
 */
pub struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1)
                    && grid[i][j] == 1
                {
                    // Self::dfs(&mut grid, (i as i32, j as i32));
                    Self::bfs(&mut grid, (i as i32, j as i32));
                }
            }
        }
        grid.iter()
            .map(|nums| nums.iter().filter(|&&num| num == 1).count() as i32)
            .sum()
    }

    pub fn bfs(grid: &mut [Vec<i32>], (x, y): (i32, i32)) {
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        grid[x as usize][y as usize] = 0;
        while let Some((cur_x, cur_y)) = queue.pop_front() {
            for (dx, dy) in Self::DIRECTIONS {
                let (nx, ny) = (cur_x + dx, cur_y + dy);
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                    continue;
                }

                if grid[nx as usize][ny as usize] == 0 {
                    continue;
                }
                queue.push_back((nx, ny));
                grid[nx as usize][ny as usize] = 0;
            }
        }
    }

    pub fn dfs(grid: &mut [Vec<i32>], (x, y): (i32, i32)) {
        grid[x as usize][y as usize] = 0;
        for (dx, dy) in Self::DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                continue;
            }
            if grid[nx as usize][ny as usize] == 0 {
                continue;
            }
            Self::dfs(grid, (nx, ny));
        }
    }
}
// @lc code=end
