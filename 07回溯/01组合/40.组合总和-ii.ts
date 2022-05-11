/**
 * /*
 *
 * [40] 组合总和 II
 *
 * @format
 * @lc app=leetcode.cn id=40 lang=typescript
 */

// @lc code=start
function combinationSum2(candidates: number[], target: number): number[][] {
  candidates.sort((a, b) => a - b)
  const result: number[][] = []
  function trackbacking(startIndex: number, arr: number[], sum: number) {
    if (sum === target) {
      result.push([...arr])
      return
    }
    if (sum > target) return
    for (
      let i = startIndex;
      i < candidates.length && sum + candidates[i] <= target;
      i++
    ) {
      //* 因为是排序的，所以可以直接跳过重复的数字
      //* 只有当广度遍历的时候才会出现重复的数字,所以i一定是大于startIndex的
      if (i > startIndex && candidates[i] === candidates[i - 1]) continue
      //push需要在判断之后添加,否则会push之后才会进入下一层
      arr.push(candidates[i])
      trackbacking(i + 1, arr, sum + candidates[i])
      arr.pop()
    }
  }
  trackbacking(0, [], 0)
  return result
}
// @lc code=end
console.log(combinationSum2([10, 1, 2, 7, 6, 1, 5], 8))

export {}
