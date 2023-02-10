/*
 * @lc app=leetcode.cn id=344 lang=typescript
 *
 * [344] 反转字符串
 */

// @lc code=start
/**
 Do not return anything, modify s in-place instead.
 */
function reverseString(s: string[]): void {
  let right = s.length - 1
  let left = 0
  while (left < right) {
    ;[s[left], s[right]] = [s[right], s[left]]
    right--
    left++
  }
}
// @lc code=end
