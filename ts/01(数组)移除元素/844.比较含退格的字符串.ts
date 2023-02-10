/*
 * @lc app=leetcode.cn id=844 lang=typescript
 *
 * [844] 比较含退格的字符串
 */

// @lc code=start
function backspaceCompare(s: string, t: string): boolean {
  function Compare(str: string): string {
    let res: string[] = []
    for (let i = 0; i < str.length; i++) {
      let code = str.charAt(i)
      code === "#" ? res.pop() : res.push(code)
    }
    return res.join("")
  }
  return Compare(s) === Compare(t)
}
// @lc code=end

console.log(backspaceCompare("ab#b", "ad#c"))
