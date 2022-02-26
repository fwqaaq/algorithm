/*
 * @lc app=leetcode.cn id=59 lang=typescript
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
function generateMatrix(n: number): number[][] {
  let loopNum: number = Math.floor(n / 2)
  const resArr: number[][] = new Array(n).fill(1).map((i) => new Array(n))
  let chunkNum: number = n - 1
  let startX: number = 0
  let startY: number = 0
  let value: number = 1
  let x: number, y: number
  while (loopNum--) {
    x = startX
    y = startY
    while (x < startX + chunkNum) {
      resArr[y][x] = value
      x++
      value++
    }
    while (y < startY + chunkNum) {
      resArr[y][x] = value
      y++
      value++
    }
    while (x > startX) {
      resArr[y][x] = value
      x--
      value++
    }
    while (y > startY) {
      resArr[y][x] = value
      y--
      value++
    }
    startX++
    startY++
    chunkNum -= 2
  }
  if (n % 2 === 1) {
    resArr[startX][startY] = value
  }
  return resArr
}
// @lc code=end
