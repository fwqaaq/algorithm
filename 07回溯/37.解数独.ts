/**
 * /*
 *
 * [37] 解数独
 *
 * @format
 * @lc app=leetcode.cn id=37 lang=typescript
 */

// @lc code=start
/**
 Do not return anything, modify board in-place instead.
 */
function solveSudoku(board: string[][]): void {
  let n: number = 9
  backTracking(n, board)
  function backTracking(n: number, board: string[][]): boolean {
    for (let row = 0; row < n; row++) {
      for (let col = 0; col < n; col++) {
        if (board[row][col] === ".") {
          for (let i = 1; i <= n; i++) {
            if (isValid(col, row, String(i), board)) {
              board[row][col] = String(i)
              if (backTracking(n, board) === true) return true
              board[row][col] = "."
            }
          }
          return false
        }
      }
    }
    return true
  }
}
// @lc code=end

function isValid(
  row: number,
  col: number,
  val: string,
  board: string[][]
): boolean {
  let n = board.length
  // 列向检查
  for (let rowIndex = 0; rowIndex < n; rowIndex++) {
    if (board[rowIndex][col] === val) return false
  }
  // 横向检查
  for (let colIndex = 0; colIndex < n; colIndex++) {
    if (board[row][colIndex] === val) return false
  }
  // 九宫格检查
  const startX = Math.floor(col / 3) * 3
  const startY = Math.floor(row / 3) * 3
  for (let rowIndex = startY; rowIndex < startY + 3; rowIndex++) {
    for (let colIndex = startX; colIndex < startX + 3; colIndex++) {
      if (board[rowIndex][colIndex] === val) return false
    }
  }
  return true
}

export {}
