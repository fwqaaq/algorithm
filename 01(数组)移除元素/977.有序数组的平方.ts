/*
 * @lc app=leetcode.cn id=977 lang=typescript
 *
 * [977] 有序数组的平方
 */

// @lc code=start
function sortedSquares(nums: number[]): number[] {
  return nums
    .map(function (item) {
      return item * item
    })
    .sort((a, b) => a - b)
}
// @lc code=end

console.log(sortedSquares([-4, -1, 0, 3, 10]))
