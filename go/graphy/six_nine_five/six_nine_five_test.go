package graphy

/*
 * @lc app=leetcode.cn id=695 lang=golang
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
var DIRECTIONS = [4][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}
var count int = 0

func maxAreaOfIsland(grid [][]int) int {
	res := 0
	visited := make([][]bool, len(grid))
	for i := 0; i < len(grid); i++ {
		visited[i] = make([]bool, len(grid[0]))
	}

	for i, rows := range grid {
		for j, v := range rows {
			if v == 1 && !visited[i][j] {
				// 第一种写法，重制 count，必定有 1 个
				// count = 1
				// 第二种写法以及 bfs
				count = 0
				// dfs(grid, visited, i, j)
				bfs(grid, visited, i, j)
				res = max(res, count)
			}

		}
	}

	return res
}

// 第一种写法
// func dfs(grid [][]int, visited [][]bool, i, j int) {
// 	visited[i][j] = true // 标记已访问，循环中未标记会导致重复
// 	for _, d := range DIRECTIONS {
// 		x, y := i+d[0], j+d[1]
// 		if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
// 			continue
// 		}
// 		if grid[x][y] == 1 && !visited[x][y] {
// 			count++
// 			dfs(grid, visited, x, y)
// 		}
// 	}
// }

// 第二种写法
// func dfs(grid [][]int, visited [][]bool, i, j int) {
// 	if visited[i][j] || grid[i][j] == 0 {
// 		return
// 	}
// 	visited[i][j] = true
// 	count++
// 	for _, d := range DIRECTIONS {
// 		x, y := i+d[0], j+d[1]
// 		if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
// 			continue
// 		}
// 		dfs(grid, visited, x, y)
// 	}
// }

// bfs
func bfs(grid [][]int, visited [][]bool, i, j int) {
	visited[i][j] = true
	count++
	queue := [][2]int{{i, j}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range DIRECTIONS {
			x, y := cur[0]+d[0], cur[1]+d[1]
			if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
				continue
			}
			if grid[x][y] == 1 && !visited[x][y] {
				count++
				queue = append(queue, [2]int{x, y})
				visited[x][y] = true
			}
		}
	}
}

// @lc code=end
