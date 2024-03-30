/*
 * @lc app=leetcode.cn id=417 lang=rust
 *
 * [417] 太平洋大西洋水流问题
 */
struct Solution;

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        let mut res = vec![];
        let (mut pacific, mut atlantic) = (vec![vec![false; n]; m], vec![vec![false; n]; m]);

        // 列
        for i in 0..m {
            // Self::dfs(&heights, &mut pacific, i, 0);
            // Self::dfs(&heights, &mut atlantic, i, n - 1);
            Self::bfs(&heights, &mut pacific, i, 0);
            Self::bfs(&heights, &mut atlantic, i, n - 1);
        }

        for j in 0..n {
            // Self::dfs(&heights, &mut pacific, 0, j);
            // Self::dfs(&heights, &mut atlantic, m - 1, j);
            Self::bfs(&heights, &mut pacific, 0, j);
            Self::bfs(&heights, &mut atlantic, m - 1, j);
        }

        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }

        res
    }

    pub fn dfs(heights: &[Vec<i32>], visited: &mut [Vec<bool>], i: usize, j: usize) {
        visited[i][j] = true;
        for (dx, dy) in Self::DIRECTIONS {
            let (x, y) = (i as isize + dx, j as isize + dy);
            if x < 0 || x >= heights.len() as isize || y < 0 || y >= heights[0].len() as isize {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if !visited[x][y] && heights[x][y] >= heights[i][j] {
                Self::dfs(heights, visited, x, y);
            }
        }
    }

    pub fn bfs(heights: &[Vec<i32>], visited: &mut [Vec<bool>], i: usize, j: usize) {
        let mut queue = VecDeque::from([(i, j)]);
        visited[i][j] = true;
        while let Some((i, j)) = queue.pop_front() {
            for (dx, dy) in Self::DIRECTIONS {
                let (x, y) = (i as isize + dx, j as isize + dy);
                if x < 0 || x >= heights.len() as isize || y < 0 || y >= heights[0].len() as isize {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if !visited[x][y] && heights[x][y] >= heights[i][j] {
                    queue.push_back((x, y));
                    visited[x][y] = true;
                }
            }
        }
    }
}
// @lc code=end

#[test]
fn test_pacific_atlantic() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let res = Solution::pacific_atlantic(heights);
    assert_eq!(
        res,
        vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0]
        ]
    );
}
