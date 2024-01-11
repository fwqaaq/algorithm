/*
 * @lc app=leetcode.cn id=200 lang=typescript
 *
 * [200] 岛屿数量
 */

// @lc code=start

const DIRECTIONS = [
  [-1, 0],
  [0, -1],
  [1, 0],
  [0, 1],
]

function numIslands(grid: string[][]): number {
  let res = 0
  const visited = Array.from({ length: grid.length }, () =>
    Array(grid[0].length).fill(false)
  )

  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      if (grid[i][j] === '1' && !visited[i][j]) {
        res++
        dfs(grid, visited, i, j)
      }
    }
  }
  return res
}

function dfs(grid: string[][], visited: boolean[][], x: number, y: number) {
  for (const [dx, dy] of DIRECTIONS) {
    const [nx, ny] = [x + dx, y + dy]
    if (nx < 0 || nx >= grid.length || ny < 0 || ny >= grid[0].length) continue
    if (grid[nx][ny] === '1' && !visited[nx][ny]) {
      visited[nx][ny] = true
      dfs(grid, visited, nx, ny)
    }
  }
}
// @lc code=end
