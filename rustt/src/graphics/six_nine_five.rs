/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */
pub struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        let mut res = 0;
        for (i, nums) in grid.iter().enumerate() {
            for (j, &num) in nums.iter().enumerate() {
                if !visited[i][j] && num == 1 {
                    // let mut count = 1;
                    // visited[i][j] = true;
                    let mut count = 0;
                    // Self::dfs(&grid, &mut visited, (i as i32, j as i32), &mut count);

                    res = res.max(count);
                }
            }
        }

        res
    }

    pub fn bfs(grid: &[Vec<i32>], visited: &mut [Vec<bool>], (x, y): (i32, i32), count: &mut i32) {
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        visited[x as usize][y as usize] = true;
        *count += 1;
        while let Some((cur_x, cur_y)) = queue.pop_front() {
            for (dx, dy) in Self::DIRECTIONS {
                let (nx, ny) = (cur_x + dx, cur_y + dy);
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == 1 {
                    visited[nx][ny] = true;
                    queue.push_back((nx as i32, ny as i32));
                    *count += 1;
                }
            }
        }
    }

    pub fn dfs(grid: &[Vec<i32>], visited: &mut [Vec<bool>], (x, y): (i32, i32), count: &mut i32) {
        if visited[x as usize][y as usize] || grid[x as usize][y as usize] == 0 {
            return;
        }
        visited[x as usize][y as usize] = true;
        *count += 1;
        for (dx, dy) in Self::DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                continue;
            }
            // let (nx, ny) = (nx as usize, ny as usize);
            // if !visited[nx][ny] && grid[nx][ny] == 1 {
            // visited[nx][ny] = true;
            // *count += 1;
            Self::dfs(grid, visited, (nx, ny), count);
            // }
        }
    }
}
// @lc code=end
