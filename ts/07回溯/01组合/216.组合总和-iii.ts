/**
 * /*
 *
 * [216] 组合总和 III
 *
 * @format
 * @lc app=leetcode.cn id=216 lang=typescript
 */

// @lc code=start
function combinationSum3(k: number, n: number): number[][] {
  const iniArr: number[][] = []
  function backtracking(startIndex: number, arr: number[]) {
    if (arr.length === k && arr.reduce((a, b) => a + b) === n) {
      iniArr.push(arr.slice())
      return
    }

    for (let i = startIndex; i < 10; i++) {
      arr.push(i)
      backtracking(i + 1, arr)
      arr.pop()
    }
  }
  backtracking(1, [])
  return iniArr
}
// @lc code=end

export {}
