package graphics

/*
 * @lc app=leetcode.cn id=417 lang=golang
 *
 * [417] 太平洋大西洋水流问题
 */

// @lc code=start
var DIRECTIONS = [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func pacificAtlantic(heights [][]int) [][]int {
	res := make([][]int, 0)
	pacific := make([][]bool, len(heights))
	atlantic := make([][]bool, len(heights))
	for i := 0; i < len(heights); i++ {
		pacific[i] = make([]bool, len(heights[0]))
		atlantic[i] = make([]bool, len(heights[0]))
	}
	// 列
	for i := 0; i < len(heights); i++ {
		// dfs(heights, pacific, i, 0)
		// dfs(heights, atlantic, i, len(heights[0])-1)
		bfs(heights, pacific, i, 0)
		bfs(heights, atlantic, i, len(heights[0])-1)
	}
	// 行
	for j := 0; j < len(heights[0]); j++ {
		bfs(heights, pacific, 0, j)
		bfs(heights, atlantic, len(heights)-1, j)
	}

	for i := 0; i < len(heights); i++ {
		for j := 0; j < len(heights[0]); j++ {
			if pacific[i][j] && atlantic[i][j] {
				res = append(res, []int{i, j})
			}
		}
	}

	return res
}

func dfs(heights [][]int, visited [][]bool, i, j int) {
	visited[i][j] = true
	for _, d := range DIRECTIONS {
		x, y := i+d[0], j+d[1]
		if x < 0 || x >= len(heights) || y < 0 || y >= len(heights[0]) || heights[i][j] > heights[x][y] || visited[x][y] {
			continue
		}

		dfs(heights, visited, x, y)
	}
}

func bfs(heights [][]int, visited [][]bool, i, j int) {
	queue := make([][]int, 0)
	queue = append(queue, []int{i, j})
	visited[i][j] = true
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range DIRECTIONS {
			x, y := cur[0]+d[0], cur[1]+d[1]
			if x < 0 || x >= len(heights) || y < 0 || y >= len(heights[0]) || heights[cur[0]][cur[1]] > heights[x][y] || visited[x][y] {
				continue
			}
			queue = append(queue, []int{x, y})
			visited[x][y] = true
		}
	}
}

// @lc code=end
