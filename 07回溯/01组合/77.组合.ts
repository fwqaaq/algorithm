/**
 * /*
 *
 * [77] 组合
 *
 * @format
 * @lc app=leetcode.cn id=77 lang=typescript
 */

// @lc code=start
function combine(n: number, k: number): number[][] {
  const iniArr: number[][] = []
  function backtracking(startIndex: number, arr: number[]) {
    //* 终止条件,达到规定的k个数
    if (arr.length === k) {
      iniArr.push(arr.slice())
      //如果是数组的长度达到k,就不用再往下走了
      return
    }

    // startIndex:当前推入数组的数字
    // k - arr.length: 剩余需要组合的个数
    // n - (k - arr.length) + 1 : 组合的终止数字
    for (let i = startIndex; i <= n - k + 1 + arr.length; i++) {
      arr.push(i)
      backtracking(i + 1, arr)
      //回溯,如果for循环没有遍历完,就需要回溯继续遍历for循环
      arr.pop()
    }
  }
  backtracking(1, [])
  return iniArr
}
// @lc code=end

export {}
