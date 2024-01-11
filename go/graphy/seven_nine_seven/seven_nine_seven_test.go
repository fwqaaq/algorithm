package graphy

import (
	"fmt"
	"testing"
)

/*
 * @lc app=leetcode.cn id=797 lang=golang
 *
 * [797] 所有可能的路径
 */

// @lc code=start
func allPathsSourceTarget(graph [][]int) [][]int {
	res := [][]int{}
	path := []int{0}
	dfs(graph, &res, path, 0)
	return res
}

func dfs(graph [][]int, res *[][]int, path []int, node int) {
	if node == len(graph)-1 {
		*res = append(*res, append([]int{}, path...))
		return
	}
	for _, v := range graph[node] {
		path = append(path, v)
		dfs(graph, res, path, v)
		path = path[:len(path)-1]
	}
}

// @lc code=end

func TestAllPathsSourceTarget(t *testing.T) {
	graph := [][]int{
		{4, 3, 1},
		{3, 2, 4},
		{3},
		{4},
		{},
	}
	fmt.Println(allPathsSourceTarget(graph))
}
