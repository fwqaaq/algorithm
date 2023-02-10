/*
 * @lc app=leetcode.cn id=367 lang=typescript
 *
 * [367] 有效的完全平方数
 */

// @lc code=start
function isPerfectSquare(num: number): boolean {
  let left = 0,
    right = num
  while (left <= right) {
    let mid = Math.floor((right + left) / 2)
    if (mid * mid <= num) {
      left = mid + 1
    } else if (mid * mid >= num) {
      right = mid - 1
    }
  }
  return right * right === num
}
// @lc code=end

console.log(isPerfectSquare(14))
