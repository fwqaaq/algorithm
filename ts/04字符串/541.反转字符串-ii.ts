/*
 * @lc app=leetcode.cn id=541 lang=typescript
 *
 * [541] 反转字符串 II
 */

// @lc code=start
function reverseStr(s: string, k: number): string {
  let left = 0
  let right = 0
  let arr: string[] = s.split("")
  for (let i = 0; i < s.length; i += 2 * k) {
    right = s.length - i < k ? s.length - 1 : i + k - 1
    left = i
    while (left < right) {
      ;[arr[left], arr[right]] = [s[right], s[left]]
      left++
      right--
    }
  }
  return arr.join("")
}
// @lc code=end
console.log(reverseStr("abcdefg", 2))
