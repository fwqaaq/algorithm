/**
 * /*
 *
 * [51] N 皇后
 *
 * @format
 * @lc app=leetcode.cn id=51 lang=typescript
 */

// @lc code=start
function solveNQueens(n: number): string[][] {
  function trackback(row: number, board: string[][]) {
    //* 终止条件:当递归的行数到n的时候,
    if (row === n) {
      //* 使用flatMap将二维数组打平
      res.push(board.flatMap((x) => x.join("")))
      return
    }
    //* for循环的是列数
    for (let col = 0; col < n; col++) {
      if (isVaild(row, col, board)) {
        board[row][col] = "Q"
        trackback(row + 1, board)
        board[row][col] = "."
      }
    }
  }
  const board = new Array(n).fill(0).map((_) => new Array(n).fill("."))
  const res: string[][] = []
  trackback(0, board)
  return res

  function isVaild(row: number, col: number, board: string[][]): boolean {
    //检查列
    for (const row of board) {
      if (row[col] === "Q") return false
    }
    // 检查45度方向
    let x: number = col,
      y: number = row
    while (y >= 0 && x < n) {
      if (board[y--][x++] === "Q") return false
    }
    // 检查135度方向
    x = col
    y = row
    while (x >= 0 && y >= 0) {
      if (board[y--][x--] === "Q") return false
    }
    return true
  }
}

export {}
