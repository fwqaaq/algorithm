/**
 * /*
 *
 * [131] 分割回文串
 *
 * @format
 * @lc app=leetcode.cn id=131 lang=typescript
 */

// @lc code=start
function partition(s: string): string[][] {
  const result: string[][] = []
  function trackback(startIndex: number, arr: string[]) {
    // 当开始的索引位置是字符串的长度时,说明已经到达最后一个字符
    if (startIndex === s.length) {
      result.push([...arr])
      return
    }

    for (let i = startIndex; i < s.length; i++) {
      if (isPalindrome(s.slice(startIndex, i + 1))) {
        //* startIndex是切割的深度.树深度的位置
        //*i是切割的宽度
        arr.push(s.slice(startIndex, i + 1))
        trackback(i + 1, arr)
        arr.pop()
      }
    }
  }
  function isPalindrome(s: string): boolean {
    for (let start = 0, end = s.length - 1; start < end; start++, end--) {
      if (s[start] !== s[end]) return false
    }
    return true
  }
  trackback(0, [])
  return result
}
// @lc code=end

console.log(partition("aab"))

export {}
