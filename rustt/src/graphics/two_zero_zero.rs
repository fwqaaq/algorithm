/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */
pub struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut res = 0;
        for (i, chars) in grid.iter().enumerate() {
            for (j, &c) in chars.iter().enumerate() {
                if !visited[i][j] && c == '1' {
                    res += 1;
                    // Self::dfs(&grid, &mut visited, (i as i32, j as i32));
                    Self::bfs(&grid, &mut visited, (i as i32, j as i32));
                }
            }
        }
        res
    }

    pub fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (x, y): (i32, i32)) {
        for (dx, dy) in Self::DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[nx][ny] == '1' && !visited[nx][ny] {
                visited[nx][ny] = true;
                Self::dfs(grid, visited, (nx as i32, ny as i32));
            }
        }
    }

    pub fn bfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (x, y): (i32, i32)) {
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        visited[x as usize][y as usize] = true;
        while let Some((cur_x, cur_y)) = queue.pop_front() {
            for (dx, dy) in Self::DIRECTIONS {
                let (nx, ny) = (cur_x + dx, cur_y + dy);
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] == '1' && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    queue.push_back((nx as i32, ny as i32));
                }
            }
        }
    }
}
// @lc code=end
