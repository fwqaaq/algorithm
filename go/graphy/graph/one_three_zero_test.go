package graphics

/*
 * @lc app=leetcode.cn id=130 lang=golang
 *
 * [130] 被围绕的区域
 */

// @lc code=start
var DIRECTIONS = [4][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}

func solve(board [][]byte) {
	rows, cols := len(board), len(board[0])
	// 列
	for i := 0; i < rows; i++ {
		if board[i][0] == 'O' {
			// dfs(board, i, 0)
			bfs(board, i, 0)
		}
		if board[i][cols-1] == 'O' {
			// dfs(board, i, cols-1)
			bfs(board, i, cols-1)
		}
	}
	// 行
	for j := 0; j < cols; j++ {
		if board[0][j] == 'O' {
			// dfs(board, 0, j)
			bfs(board, 0, j)
		}
		if board[rows-1][j] == 'O' {
			// dfs(board, rows-1, j)
			bfs(board, rows-1, j)
		}
	}

	for _, r := range board {
		for j, c := range r {
			if c == 'A' {
				r[j] = 'O'
			}
			if c == 'O' {
				r[j] = 'X'
			}
		}
	}
}

func dfs(board [][]byte, i, j int) {
	board[i][j] = 'A'
	for _, d := range DIRECTIONS {
		x, y := i+d[0], j+d[1]
		if x < 0 || x >= len(board) || y < 0 || y >= len(board[0]) {
			continue
		}
		if board[x][y] == 'O' {
			dfs(board, x, y)
		}
	}
}

func bfs(board [][]byte, i, j int) {
	queue := [][]int{{i, j}}
	board[i][j] = 'A'
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range DIRECTIONS {
			x, y := cur[0]+d[0], cur[1]+d[1]
			if x < 0 || x >= len(board) || y < 0 || y >= len(board[0]) {
				continue
			}
			if board[x][y] == 'O' {
				board[x][y] = 'A'
				queue = append(queue, []int{x, y})
			}
		}
	}
}

// @lc code=end
