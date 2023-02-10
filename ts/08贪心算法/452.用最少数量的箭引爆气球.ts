/**
 * /*
 *
 * [452] 用最少数量的箭引爆气球
 *
 * @format
 * @lc app=leetcode.cn id=452 lang=typescript
 */

// @lc code=start
function findMinArrowShots(points: number[][]): number {
  let count = 1
  points
    .sort((a, b) => a[0] - b[0])
    .forEach((item, index, array) => {
      if (index === 0) return
      // 如果当前的开头和上一个结尾不相交,增加 1
      if (item[0] > array[index - 1][1]) {
        count++
      } else {
        // 如果相交,则更新结尾
        array[index][1] = Math.min(item[1], array[index - 1][1])
      }
    })

  return count
}
// @lc code=end

console.log(
  findMinArrowShots([
    [10, 16],
    [2, 8],
    [1, 6],
    [7, 12],
  ])
)
export {}
