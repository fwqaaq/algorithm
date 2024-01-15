package graphics

/*
 * @lc app=leetcode.cn id=200 lang=golang
 *
 * [200] 岛屿数量
 */

// @lc code=start

var DIRECTIONS = [4][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}

func numIslands(grid [][]byte) int {
	res := 0

	visited := make([][]bool, len(grid))
	for i := 0; i < len(grid); i++ {
		visited[i] = make([]bool, len(grid[0]))
	}

	for i, rows := range grid {
		for j, v := range rows {
			if v == '1' && !visited[i][j] {
				res++
				_dfs(grid, visited, i, j)
				// bfs(grid, visited, i, j)
			}
		}
	}

	return res
}

func _dfs(grid [][]byte, visited [][]bool, i, j int) {
	visited[i][j] = true // 标记已访问，循环中未标记会导致重复
	for _, d := range DIRECTIONS {
		x, y := i+d[0], j+d[1]
		if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
			continue
		}
		if grid[x][y] == '1' && !visited[x][y] {
			_dfs(grid, visited, x, y)
		}
	}
}

func bfs(grid [][]byte, visited [][]bool, i, j int) {
	queue := [][2]int{{i, j}}
	visited[i][j] = true // 标记已访问，循环中标记会导致重复
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range DIRECTIONS {
			x, y := cur[0]+d[0], cur[1]+d[1]
			if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
				continue
			}
			if grid[x][y] == '1' && !visited[x][y] {
				visited[x][y] = true
				queue = append(queue, [2]int{x, y})
			}
		}
	}
}

// @lc code=end
