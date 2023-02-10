/*
 * @lc app=leetcode.cn id=459 lang=typescript
 *
 * [459] 重复的子字符串
 */

// @lc code=start
function repeatedSubstringPattern(s: string): boolean {
  let next: number[] = []
  let j = -1
  next[0] = j
  for (let i = 1; i < s.length; i++) {
    while (j >= 0 && s[j + 1] !== s[i]) {
      j = next[j]
    }
    if (s[j + 1] === s[i]) j++
    next[i] = j
  }
  let prefixLast = next[s.length - 1] + 1
  let childStr = s.length - prefixLast
  return prefixLast % childStr === 0 && prefixLast !== 0
}
// @lc code=end
console.log(repeatedSubstringPattern("abcabc"))
