/**
 * /*
 *
 * [455] 分发饼干
 *
 * @format
 * @lc app=leetcode.cn id=455 lang=typescript
 */

// @lc code=start
function findContentChildren(g: number[], s: number[]): number {
  g.sort((a: number, b: number) => a - b)
  s.sort((a: number, b: number) => a - b)
  let result = 0
  for (let i = 0; i < s.length; i++) {
    //* 判断s[i]饼干是否大于g[j]的胃口
    if (result < g.length && s[i] >= g[result]) {
      result++
    }
  }
  return result
}
// @lc code=end
export {}
