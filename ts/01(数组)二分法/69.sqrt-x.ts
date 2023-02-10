/*
 * @lc app=leetcode.cn id=69 lang=typescript
 *
 * [69] Sqrt(x)
 */

// @lc code=start
function mySqrt(x: number): number {
  let left = 0,
    right = x
  while (left <= right) {
    let mid = Math.floor((left + right) / 2)
    if (mid * mid <= x) {
      left = mid + 1
    } else if (mid * mid >= x) {
      right = mid - 1
    }
  }
  return right
}
// @lc code=end

console.log(mySqrt(10))
