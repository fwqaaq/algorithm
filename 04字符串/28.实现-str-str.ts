/*
 * @lc app=leetcode.cn id=28 lang=typescript
 *
 * [28] 实现 strStr()
 */

// @lc code=start
/**
 * Given a string, find the first occurrence of a substring
 * @param {string} haystack - the string to be searched
 * @param {string} needle - the string to be searched for in haystack
 * @returns The index of the first occurrence of needle in haystack, or -1 if needle is not part of
 * haystack.
 */
function strStr(haystack: string, needle: string): number {
  if (needle.length === 0) return 0
  //构造前缀表
  let j = -1,
    next: number[] = []
  next[0] = j
  for (let i = 1; i < needle.length; i++) {
    //*前后缀不相同
    while (j >= 0 && needle[i] !== needle[j + 1]) {
      //todo向前回退
      j = next[j]
    }
    if (needle[i] === needle[j + 1]) j++
    next[i] = j
  }
  let w = -1
  for (let i = 0; i < haystack.length; i++) {
    while (w >= 0 && haystack[i] !== needle[w + 1]) {
      w = next[w]
    }
    if (haystack[i] === needle[w + 1]) {
      if (w === needle.length - 2) {
        return i - w - 1
      }
      w++
    }
  }
  return -1
}
// @lc code=end

console.log(strStr("mississippi", "mississippi"))
