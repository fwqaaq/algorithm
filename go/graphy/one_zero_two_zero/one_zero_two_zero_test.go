package graphics

/*
 * @lc app=leetcode.cn id=1020 lang=golang
 *
 * [1020] 飞地的数量
 */

// @lc code=start

var DIRECTIONS = [4][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}
var count int = 0

func numEnclaves(grid [][]int) int {
	rows, cols := len(grid), len(grid[0])
	// 行
	for i := range grid[0] {
		if grid[0][i] == 1 {
			// dfs(grid, 0, i)
			bfs(grid, 0, i)
		}
		if grid[rows-1][i] == 1 {
			// dfs(grid, rows-1, i)
			bfs(grid, rows-1, i)
		}
	}
	// 列
	for j := range grid {
		if grid[j][0] == 1 {
			// dfs(grid, j, 0)
			bfs(grid, j, 0)
		}
		if grid[j][cols-1] == 1 {
			// dfs(grid, j, cols-1)
			bfs(grid, j, cols-1)
		}
	}
	count = 0
	for i := range grid {
		for j := range grid[0] {
			if grid[i][j] == 1 {
				// dfs(grid, i, j)
				bfs(grid, i, j)
			}
		}
	}
	return count
}

func dfs(grid [][]int, i, j int) {
	grid[i][j] = 0
	count++
	for _, d := range DIRECTIONS {
		x, y := i+d[0], j+d[1]
		if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
			continue
		}
		if grid[x][y] == 1 {
			dfs(grid, x, y)
		}
	}
}

func bfs(grid [][]int, i, j int) {
	queue := [][]int{}
	queue = append(queue, []int{i, j})
	grid[i][j] = 0
	count++
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range DIRECTIONS {
			x, y := cur[0]+d[0], cur[1]+d[1]
			if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
				continue
			}
			if grid[x][y] == 1 {
				count++
				queue = append(queue, []int{x, y})
				grid[x][y] = 0
			}
		}
	}
}

// @lc code=end
