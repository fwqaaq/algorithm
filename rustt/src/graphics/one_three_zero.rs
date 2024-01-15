pub struct Solution;
/*
* @lc app=leetcode.cn id=130 lang=rust
*
* [130] 被围绕的区域
*/

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (rows, cols) = (board.len(), board[0].len());
        // 列
        for i in 0..rows {
            if board[i][0] == 'O' {
                // Self::dfs(board, i, 0);
                Self::bfs(board, i, 0);
            }
            if board[i][cols - 1] == 'O' {
                // Self::dfs(board, i, cols - 1);
                Self::bfs(board, i, cols - 1);
            }
        }
        //行
        for j in 0..cols {
            if board[0][j] == 'O' {
                // Self::dfs(board, 0, j);
                Self::bfs(board, 0, j);
            }
            if board[rows - 1][j] == 'O' {
                // Self::dfs(board, rows - 1, j);
                Self::bfs(board, rows - 1, j);
            }
        }

        for v in board.iter_mut() {
            for c in v.iter_mut() {
                if *c == 'A' {
                    *c = 'O';
                    continue;
                }
                if *c == 'O' {
                    *c = 'X';
                }
            }
        }
    }

    pub fn dfs(board: &mut [Vec<char>], i: usize, j: usize) {
        board[i][j] = 'A';
        for (d1, d2) in Self::DIRECTIONS {
            let (x, y) = (i as isize + d1, j as isize + d2);
            if x < 0 || x >= board.len() as isize || y < 0 || y >= board[0].len() as isize {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if board[x][y] == 'O' {
                Self::dfs(board, x, y);
            }
        }
    }

    pub fn bfs(board: &mut [Vec<char>], i: usize, j: usize) {
        let mut queue = VecDeque::from([(i, j)]);
        board[i][j] = 'A';
        while let Some((i, j)) = queue.pop_front() {
            for (d1, d2) in Self::DIRECTIONS {
                let (x, y) = (i as isize + d1, j as isize + d2);
                if x < 0 || x >= board.len() as isize || y < 0 || y >= board[0].len() as isize {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if board[x][y] == 'O' {
                    board[x][y] = 'A';
                    queue.push_back((x, y));
                }
            }
        }
    }
}
// @lc code=end

#[test]
fn test_solve() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
    );
}
